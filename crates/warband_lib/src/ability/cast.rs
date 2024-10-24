use bevy::ecs::{
    component::{ComponentHooks, ComponentId, StorageType},
    world::{Command, DeferredWorld},
};

use super::{event, AbilityId, Target};
use crate::{
    ability::Caster,
    prelude::*,
    stats::modifier::{Flat, Modifies, Mult},
};

#[derive(Component, Reflect)]
pub(crate) struct AbilityCaster;

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
                    .insert(Modifies::Single(entity))
                    .id();

                let mut slots = or_return!(slots.get_mut(entity));
                slots.0 = container;
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

    world.commands().add(AbilitySlotsCommand {
        entity,
        abilities: ability_slots.abilities,
    });
}

#[derive(Component, Reflect, Clone)]
pub(crate) struct AbilitySlotsCommand {
    entity: Entity,
    abilities: Vec<AbilityId>,
}

impl Command for AbilitySlotsCommand {
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

pub(super) fn total_slot_mana(
    mut ability_slots: Query<&AbilitySlots>,
    mut ability_slots_entity: Query<(&Children, &mut Flat<Mana>)>,
    slot: Query<&Mana, With<AbilitySlot>>,
) {
    for slot_link in &mut ability_slots {
        let (children, mut flat_mana) = or_continue!(ability_slots_entity.get_mut(slot_link.0));
        let mut total_mana: f32 = 0.0;
        for ability in children.iter() {
            let mana = or_continue!(slot.get(*ability));
            total_mana += mana.0;
        }

        if total_mana != flat_mana.value() {
            flat_mana.0 = Mana(total_mana);
        }
    }
}

pub(super) fn slot_mana_modifier(
    caster: Query<(&AbilitySlots, Ref<Mana>), With<AbilityCaster>>,
    ability_slots: Query<(&Children, Ref<Flat<Mana>>)>,
    mut slot: Query<(&mut Mult<Mana>, &Flat<Mana>), With<AbilitySlot>>,
) {
    for (slots_entity, caster_mana) in &caster {
        let (ability_slots, ability_mana) = or_continue!(ability_slots.get(slots_entity.0));
        let mana_diff = caster_mana.value() - ability_mana.value();

        if mana_diff.is_approx_zero() {
            return;
        }

        let total_original_mana: f32 = ability_slots
            .iter()
            .map(|e| or_return!(0.0, slot.get(*e).map(|(_, mana)| mana.value())))
            .sum();

        let modifier = caster_mana.value() / total_original_mana;
        for id in ability_slots.iter() {
            let (mut mult_mana, _) = or_continue!(slot.get_mut(*id));
            if !(modifier - mult_mana.0.value()).is_approx_zero() {
                mult_mana.0 = Mana(modifier);
            }
        }
    }
}

pub(super) fn slot_mana(
    ability: Query<(&Slot, &Mana), (Changed<Mana>, Changed<Slot>)>,
    mut slot: Query<
        &mut Flat<Mana>,
        (
            With<EquippedAbility>,
            Without<AbilityId>,
            Without<AbilitySlot>,
        ),
    >,
) {
    for (slot_id, mana) in &ability {
        let mut slot_mana = or_continue!(slot.get_mut(slot_id.0));
        slot_mana.0 = *mana;
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
}

pub(super) fn ability_slot_events(
    mut events: EventReader<AbilitySlotEvent>,
    mut commands: Commands,
    ability_slots: Query<&AbilitySlots>,
) {
    for event in events.read() {
        if let AbilitySlotEvent::PushAbility { caster, ability } = event {
            let ability = match ability.clone() {
                AbilitySource::Entity(entity) => entity,
                AbilitySource::AbilityId(id) => {
                    commands.spawn_empty().insert(super::Ability(id)).id()
                },
            };

            let ability_slots = or_return!(ability_slots.get(*caster));
            let ability_slot = commands
                .spawn_empty()
                .insert((AbilitySlot, Caster(*caster), EquippedAbility(ability)))
                .add_child(ability)
                .id();

            commands.entity(**ability_slots).add_child(ability_slot);
            commands
                .entity(ability)
                .insert((Slot(ability_slot), Caster(*caster)));
        }
    }
}

#[derive(Stat, Component, Default, Reflect, Copy, Clone)]
#[clamp(min = 0)]
pub(crate) struct Mana(pub(crate) f32);

// TODO: convert to command

#[derive(Event, Reflect)]
pub struct TryAbility {
    pub caster: Entity,
    pub ability: Entity,
    pub target: Target,
}

#[derive(Event, Reflect)]
pub struct CastAbility {
    pub caster: Entity,
    pub ability: Entity,
    pub target: Target,
}

pub(super) fn try_ability(
    mut events: EventReader<TryAbility>,
    mut cast_ability: EventWriter<CastAbility>,
) {
    for TryAbility {
        caster,
        ability,
        target,
    } in events.read()
    {
        // TODO: check if the caster has enough mana etc.
        cast_ability.send(CastAbility {
            caster: *caster,
            ability: *ability,
            target: *target,
        });
    }
}

pub(super) fn cast_ability(mut events: EventReader<CastAbility>, mut commands: Commands) {
    for CastAbility {
        caster,
        ability,
        target,
    } in events.read()
    {
        commands.trigger_targets(
            event::OnCast {
                caster: *caster,
                ability: *ability,
                target: *target,
            },
            *ability,
        );
    }
}
