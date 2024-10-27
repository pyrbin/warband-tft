use bevy::ecs::{
    component::{ComponentHooks, ComponentId, StorageType},
    world::{Command, DeferredWorld},
};

use super::{cast::AbilityCaster, AbilityId, Mana, TargetTeam};
use crate::{
    ability::Caster,
    prelude::*,
    stats::modifier::{Flat, Mult},
    unit::{stats::Range, Allegiance},
};

pub(super) fn plugin(app: &mut App) {
    app_register_types!(
        AbilitySlot,
        AbilitySlots,
        AbilitySlotEvent,
        AbilitySource,
        AbilitySlotStatus,
        EquippedAbility
    );

    app.configure::<(AbilitySlots, AbilitySlot)>();
    app.add_event::<AbilitySlotEvent>();

    app.add_systems(Update, ((mana, per_slot_mana).chain(), ability_slot_events));
}

#[derive(Component, Reflect, Deref, Clone, From)]
pub(crate) struct Slot(pub Entity);

#[derive(Component, Reflect, Deref, Clone, From)]
pub(crate) struct AbilitySlots(Entity);

impl AbilitySlots {
    pub(crate) fn empty() -> Self {
        Self(Entity::PLACEHOLDER)
    }

    pub(crate) fn with(ability: impl Into<AbilityId>) -> AbilitySlotsBuilder {
        AbilitySlotsBuilder::new().with(ability)
    }

    pub(crate) fn get(&self) -> Entity {
        self.0
    }
}

impl Configure for AbilitySlots {
    fn configure(app: &mut App) {
        app.observe(
            |trigger: Trigger<OnAdd, AbilitySlots>,
             mut commands: Commands,
             mut slots: Query<&mut AbilitySlots>| {
                const NAME: &str = "ability slots";
                let entity = trigger.entity();
                let container = commands
                    .spawn_empty()
                    .insert(Name::new(NAME))
                    .insert(Flat::<Mana>::default())
                    .id();

                let mut slots = or_return!(slots.get_mut(entity));
                slots.0 = container;

                commands.entity(entity).add_child(container);
            },
        );

        app.observe(
            |trigger: Trigger<OnRemove, AbilitySlots>,
             mut commands: Commands,
             slots: Query<&AbilitySlots>| {
                let entity = trigger.entity();
                let slots = or_return!(slots.get(entity));
                commands.entity(slots.0).despawn_recursive();
            },
        );
    }
}

#[derive(Component, Reflect)]
pub(crate) struct AbilitySlot;

impl Configure for AbilitySlot {
    fn configure(app: &mut App) {
        app.observe(
            |trigger: Trigger<OnAdd, AbilitySlot>, mut commands: Commands| {
                let entity = trigger.entity();
                commands.entity(entity).insert((
                    Name::new("ability slot"),
                    Mana::pool(0.0),
                    Mult::<Mana>::default(),
                    Flat::<Mana>::default(),
                    AbilitySlotStatus::UnsufficientMana,
                ));
            },
        );
    }
}

#[derive(Reflect, Clone)]
pub(crate) struct AbilitySlotsBuilder {
    abilities: Vec<AbilityId>,
}

impl AbilitySlotsBuilder {
    fn new() -> Self {
        Self {
            abilities: Vec::new(),
        }
    }

    pub(crate) fn with(mut self, ability: impl Into<AbilityId>) -> Self {
        self.abilities.push(ability.into());
        self
    }
}

impl Component for AbilitySlotsBuilder {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(ability_slots_builder);
    }
}

fn ability_slots_builder(mut world: DeferredWorld<'_>, entity: Entity, _cid: ComponentId) {
    let ability_slots = {
        let ability_slots = world.get::<AbilitySlotsBuilder>(entity);
        ability_slots.cloned()
    }
    .expect("ability slot builder should exist");

    world.commands().add(AbilitySlotsBuilderCommand {
        entity,
        abilities: ability_slots.abilities,
    });
}

#[derive(Component, Reflect, Clone)]
pub(crate) struct AbilitySlotsBuilderCommand {
    entity: Entity,
    abilities: Vec<AbilityId>,
}

impl Command for AbilitySlotsBuilderCommand {
    fn apply(self, world: &mut World) {
        world
            .commands()
            .entity(self.entity)
            .remove::<AbilitySlotsBuilder>()
            .insert(AbilitySlots::empty());

        let mut event_writer = world
            .get_resource_mut::<Events<AbilitySlotEvent>>()
            .unwrap();

        for ability in self.abilities {
            event_writer.send(AbilitySlotEvent::PushAbility {
                caster: self.entity,
                ability: AbilitySource::AbilityId(ability),
            });
        }
    }
}

#[derive(Component, Reflect)]
pub(crate) struct EquippedAbility(pub Entity);

#[derive(Component, Reflect)]
pub(crate) enum AbilitySlotStatus {
    Ready,
    UnsufficientMana,
}

fn mana(
    mut ability_slots: Query<&AbilitySlots>,
    mut ability_slots_entity: Query<(&Children, &mut Flat<Mana>)>,
    slot: Query<&EquippedAbility, With<AbilitySlot>>,
    ability: Query<&Mana, With<AbilityId>>,
) {
    for slot_id in &mut ability_slots {
        let (children, mut flat_mana) = or_continue_quiet!(ability_slots_entity.get_mut(slot_id.0));

        let mut total_mana: f32 = 0.0;

        for ability_id in children.iter() {
            let equipped_ability = or_continue!(slot.get(*ability_id));
            let mana = or_continue!(ability.get(equipped_ability.0));
            total_mana += mana.value();
        }

        if total_mana != flat_mana.value() {
            flat_mana.0 = Mana(total_mana);
        }
    }
}

fn per_slot_mana(
    caster: Query<(&AbilitySlots, &Mana), (With<AbilityCaster>, Without<AbilityId>)>,
    ability_slots: Query<(&Children, &Flat<Mana>), Without<AbilitySlot>>,
    mut slot: Query<(&mut Flat<Mana>, &EquippedAbility), With<AbilitySlot>>,
    ability: Query<&Mana, With<AbilityId>>,
) {
    for (slots_id, caster_mana) in &caster {
        let (ability_slots, total_mana) = or_continue_quiet!(ability_slots.get(slots_id.0));

        for id in ability_slots.iter() {
            let (mut slot_mana, equipped_ability) = or_continue!(slot.get_mut(*id));
            let mana = or_continue!(ability.get(equipped_ability.0));
            let part = mana.value() / total_mana.value();
            let mana_per_slot = part * caster_mana.value();

            if !(slot_mana.0.value() - mana_per_slot).abs().is_approx_zero() {
                slot_mana.0 = Mana(mana_per_slot);
            }
        }
    }
}

fn ability_slot_mana(
    ability: Query<(&Slot, &Flat<Mana>), (Changed<Mana>, Changed<Slot>, With<AbilityId>)>,
    mut slot: Query<
        &mut Flat<Mana>,
        (With<EquippedAbility>, Without<AbilityId>, With<AbilitySlot>),
    >,
) {
    for (slot_id, mana) in &ability {
        let mut slot_mana = or_continue!(slot.get_mut(slot_id.0));
        if (mana.0.value() - slot_mana.0.value()).is_approx_zero() {
            continue;
        }
        slot_mana.0 = Mana(mana.0.value());
    }
}

#[derive(Clone, Reflect)]
pub(crate) enum AbilitySource {
    Entity(Entity),
    AbilityId(AbilityId),
}

#[derive(Event, Reflect)]
pub(crate) enum AbilitySlotEvent {
    AddAbilityToSlot {
        caster: Entity,
        ability_slot: Entity,
        ability: AbilitySource,
    },
    PushAbility {
        caster: Entity,
        ability: AbilitySource,
    },
    SwapAbilitySlots {
        source_slot_index: usize,
        target_slot_index: usize,
    },
    TryCastAbility {
        caster: Entity,
        ability_slot: Entity,
        target: Target,
    },
}

pub(super) fn ability_slot_events(
    mut events: EventReader<AbilitySlotEvent>,
    mut commands: Commands,
    casters: Query<(&AbilitySlots, &Mana), With<AbilityCaster>>,
    ability_slots: Query<
        (
            Option<&EquippedAbility>,
            &Caster,
            Pool<Mana>,
            &AbilitySlotStatus,
            Option<&Range>,
        ),
        With<AbilitySlot>,
    >,
    units: Query<(&Allegiance, &GlobalTransform)>,
    abilities: Query<(&TargetTeam), With<AbilityId>>,
) {
    for event in events.read() {
        match event {
            AbilitySlotEvent::PushAbility { caster, ability } => {
                let ability = match ability.clone() {
                    AbilitySource::Entity(entity) => entity,
                    AbilitySource::AbilityId(id) => {
                        commands.spawn_empty().insert(super::Ability(id)).id()
                    },
                };

                let ability_slot = commands
                    .spawn_empty()
                    .insert((AbilitySlot, Caster(*caster), EquippedAbility(ability)))
                    .add_child(ability)
                    .id();

                let (ability_slots, _) = or_continue!(casters.get(*caster));
                commands.entity(**ability_slots).add_child(ability_slot);

                commands
                    .entity(ability)
                    .insert((Slot(ability_slot), Caster(*caster)));
            },
            AbilitySlotEvent::TryCastAbility {
                caster,
                ability_slot,
                target,
            } => {
                let (_, _) = or_continue!(casters.get(*caster));
                let (equipped_ability, slot_caster, mana, status, range) =
                    or_continue!(ability_slots.get(*ability_slot));

                if *caster != **slot_caster {
                    info!("caster doesn't have that slot");
                    continue;
                }

                let Some(equipped_ability) = equipped_ability else {
                    info!("slot is empty");
                    continue;
                };

                let (target_team) = or_continue!(abilities.get(equipped_ability.0));

                if mana.progress01() < 1.0 {
                    info!("not enough mana");
                    continue;
                }

                let (caster_allegiance, caster_transform) = or_continue!(units.get(*caster));

                match target {
                    Target::Point(point) => {
                        let distance = caster_transform.translation().xz().distance(point.xz());
                        if let Some(range) = range
                            && distance > range.value()
                        {
                            info!("target is too far");
                            continue;
                        }
                    },
                    Target::Entity(target) => {
                        let (target_allegiance, target_transform) =
                            or_continue!(units.get(*target));

                        let distance = caster_transform
                            .translation()
                            .xz()
                            .distance(target_transform.translation().xz());

                        if let Some(range) = range
                            && distance > range.value()
                        {
                            info!("target is too far");
                            continue;
                        }

                        if !target_team.can_target(*caster_allegiance, *target_allegiance) {
                            info!("ability cannot target that");
                            continue;
                        }
                    },
                }

                // TODO: check and commit casting
            },
            _ => {},
        }
    }
}
