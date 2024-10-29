use bevy::ecs::{
    component::{ComponentHooks, ComponentId, StorageType},
    world::{Command, DeferredWorld},
};

use super::{caster::AbilityCaster, AbilityId, Mana};
use crate::{ability::Caster, prelude::*, stats::modifier::Flat};

pub(super) fn plugin(app: &mut App) {
    app_register_types!(
        AbilitySlot,
        AbilitySlots,
        AbilitySlotEvent,
        AbilityFrom,
        EquippedAbility
    );

    app.configure::<AbilitySlot>();
    app.add_event::<AbilitySlotEvent>();

    app.add_systems(Update, ((mana, per_slot_mana).chain(), events));
}

#[derive(Component, Reflect, Deref, DerefMut, Clone, From)]
pub(crate) struct AbilitySlots(pub(crate) SmallVec<[Entity; 4]>);

impl AbilitySlots {
    pub(crate) fn empty() -> Self {
        Self(SmallVec::new())
    }

    pub(crate) fn with(ability: impl Into<AbilityId>) -> AbilitySlotsBuilder {
        AbilitySlotsBuilder::new().with(ability)
    }
}

#[derive(Component, Reflect)]
pub(crate) struct AbilitySlot; // (pub(crate) usize Index)

impl Configure for AbilitySlot {
    fn configure(app: &mut App) {
        app.observe(
            |trigger: Trigger<OnAdd, AbilitySlot>, mut commands: Commands| {
                let entity = trigger.entity();
                commands.entity(entity).insert(Mana::pool(0.0));
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
            event_writer.send(AbilitySlotEvent::Push {
                caster: self.entity,
                ability: AbilityFrom::AbilityId(ability),
            });
        }
    }
}

#[derive(Component, Deref, DerefMut, Reflect)]
pub(crate) struct EquippedAbility(pub Entity);

fn mana(
    mut ability_slots: Query<(&AbilitySlots, &mut Flat<Mana>)>,
    slot: Query<&EquippedAbility, With<AbilitySlot>>,
    ability: Query<&Mana, With<AbilityId>>,
) {
    for (children, mut flat_mana) in &mut ability_slots {
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
    caster: Query<
        (&AbilitySlots, &Mana, &Flat<Mana>),
        (
            With<AbilityCaster>,
            (Without<AbilityId>, Without<AbilitySlot>),
        ),
    >,
    mut slot: Query<(&mut Flat<Mana>, &EquippedAbility), With<AbilitySlot>>,
    ability: Query<&Mana, With<AbilityId>>,
) {
    for (ability_slots, caster_mana, base_mana) in &caster {
        for id in ability_slots.iter() {
            let (mut slot_mana, equipped_ability) = or_continue!(slot.get_mut(*id));
            let mana = or_continue!(ability.get(equipped_ability.0));
            let part = mana.value() / base_mana.value();
            let mana_per_slot = part * caster_mana.value();

            if !(slot_mana.0.value() - mana_per_slot).abs().is_approx_zero() {
                slot_mana.0 = Mana(mana_per_slot);
            }
        }
    }
}

#[derive(Clone, Reflect)]
pub(crate) enum AbilityFrom {
    Entity(Entity),
    AbilityId(AbilityId),
}

#[derive(Event, Reflect)]
pub(crate) enum AbilitySlotEvent {
    AddToSlot {
        ability_slot: Entity, // TODO: operate on slot_index
        ability: AbilityFrom,
    },
    Push {
        caster: Entity,
        ability: AbilityFrom,
    },
    SwapSlots {
        source_slot_index: usize,
        target_slot_index: usize,
    },
}

pub(super) fn events(
    mut reader: EventReader<AbilitySlotEvent>,
    mut commands: Commands,
    mut casters: Query<&mut AbilitySlots, With<AbilityCaster>>,
) {
    for event in reader.read() {
        if let AbilitySlotEvent::Push { caster, ability } = event {
            let ability = match ability.clone() {
                AbilityFrom::Entity(entity) => entity,
                AbilityFrom::AbilityId(id) => {
                    commands.spawn_empty().insert(super::Ability(id)).id()
                },
            };

            let mut ability_slots = or_continue!(casters.get_mut(*caster));
            let ability_slots_len = ability_slots.len();

            let ability_slot = commands
                .spawn_empty()
                .insert((
                    AbilitySlot,
                    Caster(*caster),
                    EquippedAbility(ability),
                    Name::new(format!("ability slot {ability_slots_len:?}")),
                ))
                .add_child(ability)
                .id();

            ability_slots.push(ability_slot);

            commands.entity(*caster).add_child(ability_slot);
            commands.entity(ability).insert(Caster(*caster));
        }
    }
}
