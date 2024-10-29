use std::collections::VecDeque;

use super::{
    event,
    slot::{AbilitySlot, AbilitySlots, EquippedAbility},
    AbilityId,
    CanTarget,
    CastTime,
    Caster,
    Mana,
    Target,
};
use crate::{
    prelude::*,
    unit::{stats::Range, Allegiance},
};

pub(super) fn plugin(app: &mut App) {
    app_register_types!(
        AbilityCaster,
        Casting,
        AbilityReadyQueue,
        AbilityCasterEvent
    );

    app.add_event::<AbilityCasterEvent>();
    app.configure::<AbilityCaster>();
    app.add_systems(Update, events);
}

#[derive(Reflect)]
pub(crate) enum CastFrom {
    Slot(Entity),
    FirstReady,
}

#[derive(Event, Reflect)]
pub(crate) enum AbilityCasterEvent {
    Cast {
        caster: Entity,
        ability: CastFrom,
        target: Target,
    },
    CastStart {
        caster: Entity,
        ability_slot: Entity, // TODO: operate on slot_index
        target: Target,
    },
    CastFinished {
        caster: Entity,
        ability_slot: Entity, // TODO: operate on slot_index
        target: Target,
        success: bool,
    },
    Mana {
        caster: Entity,
        delta: f32,
    },
}

#[derive(Component, Reflect, Default, PartialEq)]
pub(crate) enum AbilityCaster {
    #[default]
    Idle,
    Ready,
    Casting(Entity),
}

impl Configure for AbilityCaster {
    fn configure(app: &mut App) {
        app.add_systems(Update, (status, casting, ability_ready_queue));
        app.observe(on_casting);
        app.add_systems(
            Update,
            required_component::<AbilityCaster, AbilityReadyQueue>,
        );
    }
}

#[derive(Component, Reflect)]
pub(crate) struct Casting {
    timer: Timer,
    target: Target,
    ability_slot: Entity,
}

#[derive(Component, Reflect, Default, Deref, DerefMut)]
pub(crate) struct AbilityReadyQueue(VecDeque<Entity>);

fn events(
    mut events: EventReader<AbilityCasterEvent>,
    mut commands: Commands,
    mut casters: Query<(&mut AbilityCaster, &Mana, &AbilitySlots, &AbilityReadyQueue)>,
    mut ability_slots: Query<(Option<&EquippedAbility>, Pool<Mana>, &Caster), With<AbilitySlot>>,
    abilities: Query<(&CanTarget, Option<&Range>, Option<&CastTime>), With<AbilityId>>,
    units: Query<(&Allegiance, &GlobalTransform)>,
) {
    for event in events.read() {
        match event {
            AbilityCasterEvent::Cast {
                caster,
                ability,
                target,
            } => {
                let (mut ability_caster, .., ready_queue) = or_continue!(casters.get_mut(*caster));

                let ability_slot = match ability {
                    CastFrom::Slot(ability_slot) => *ability_slot,
                    CastFrom::FirstReady => {
                        let ability_slot = or_continue!(ready_queue.front());
                        *ability_slot
                    },
                };

                let (equipped_ability, mut mana, slot_caster) =
                    or_continue!(ability_slots.get_mut(ability_slot));

                assert!(
                    slot_caster.0 == *caster,
                    "ability slot doesn't belong to the caster"
                );

                if !matches!(*ability_caster, AbilityCaster::Ready) {
                    info!("caster is not ready");
                    continue;
                }

                let Some(equipped_ability) = equipped_ability else {
                    info!("slot is empty");
                    continue;
                };

                let (can_target, range, cast_time) =
                    or_continue!(abilities.get(equipped_ability.0));
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

                        if !can_target.can_target(*caster_allegiance, *target_allegiance) {
                            info!("ability cannot target that");
                            continue;
                        }
                    },
                }

                mana.set_current(0.0);

                *ability_caster = AbilityCaster::Casting(ability_slot);

                let cast_time = cast_time
                    .map(|cast_time| cast_time.value())
                    .unwrap_or_default();

                commands.entity(*caster).insert(Casting {
                    timer: Timer::from_seconds(cast_time, TimerMode::Once),
                    target: *target,
                    ability_slot,
                });
            },
            AbilityCasterEvent::CastFinished {
                caster,
                ability_slot,
                target,
                success,
            } => {
                let (equipped_ability, .., slot_caster) =
                    or_continue!(ability_slots.get_mut(*ability_slot));

                assert!(
                    slot_caster.0 == *caster,
                    "ability slot doesn't belong to the caster"
                );

                let Some(equipped_ability) = equipped_ability else {
                    info!("slot is empty");
                    continue;
                };

                if !success {
                    info!("casting failed");
                    continue;
                }

                commands.entity(*caster).remove::<Casting>();

                commands.trigger_targets(
                    event::OnCast {
                        caster: *caster,
                        ability: **equipped_ability,
                        target: *target,
                    },
                    **equipped_ability,
                );
            },
            AbilityCasterEvent::Mana { caster, delta } => {
                let (_, caster_mana, slots, _) = or_continue!(casters.get(*caster));
                for slot in slots.iter() {
                    if let Ok((_, mut mana, _)) = ability_slots.get_mut(*slot) {
                        let percentage = mana.total() / caster_mana.value();
                        mana += percentage * delta;
                    }
                }
            },
            _ => {},
        }
    }
}

fn on_casting(
    trigger: Trigger<OnAdd, Casting>,
    caster: Query<&Casting>,
    mut events: EventWriter<AbilityCasterEvent>,
) {
    let entity = trigger.entity();
    let casting = or_return!(caster.get(entity));
    events.send(AbilityCasterEvent::CastStart {
        caster: entity,
        ability_slot: casting.ability_slot,
        target: casting.target,
    });
}

fn casting(
    mut casters: Query<(Entity, &mut Casting), With<AbilityCaster>>,
    time: Res<Time>,
    mut events: EventWriter<AbilityCasterEvent>,
) {
    for (entity, mut casting) in &mut casters {
        if (casting.timer.tick(time.delta())).just_finished() {
            events.send(AbilityCasterEvent::CastFinished {
                caster: entity,
                ability_slot: casting.ability_slot,
                target: casting.target,
                success: true,
            });
        }
    }
}

fn status(mut casters: Query<(&mut AbilityCaster, Option<&Casting>, &AbilityReadyQueue)>) {
    for (mut ability_caster, casting, ready_queue) in &mut casters {
        let new_ability_caster = match (casting, ready_queue) {
            (Some(Casting { ability_slot, .. }), _) => AbilityCaster::Casting(*ability_slot),
            (_, AbilityReadyQueue(que)) if !que.is_empty() => AbilityCaster::Ready,
            _ => AbilityCaster::Idle,
        };

        if *ability_caster != new_ability_caster {
            *ability_caster = new_ability_caster;
        }
    }
}

fn ability_ready_queue(
    mut casters: Query<(&AbilitySlots, &mut AbilityReadyQueue), With<AbilityCaster>>,
    slot: Query<Pool<Mana>, With<AbilitySlot>>,
) {
    for (AbilitySlots(slots), mut ready_queue) in &mut casters {
        ready_queue.clear();

        for entity in slots.iter() {
            let mana = or_continue!(slot.get(*entity));
            if mana.full() {
                ready_queue.push_back(*entity);
            }
        }
    }
}
