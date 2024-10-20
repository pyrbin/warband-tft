use crate::{
    navigation::{
        agent::{self},
        path::{self},
    },
    prelude::*,
};
use bevy_spatial::SpatialAccess;
use big_brain::prelude::*;

use super::{combat::DamageEvent, Allegiance, UnitTree};

pub(super) fn plugin(app: &mut App) {
    app_register_types!(Target);

    app.add_plugins(BigBrainPlugin::new(PreUpdate));

    app.add_systems(PreUpdate, (seek, attack).in_set(BigBrainSet::Actions));
    app.add_systems(
        PreUpdate,
        (seeker_scorer, attack_scorer).in_set(BigBrainSet::Scorers),
    );
}

lazy_static::lazy_static! {
    pub static ref UNIT_THINKER: ThinkerBuilder = Thinker::build()
        .picker(FirstToScore { threshold: 1.0})
        .when(Scorer::<Seek>::default(), Seek { attack_range: 2 })
        .when(Scorer::<Attack>::default(), Attack { damage: 0.5 });
}

#[derive(Debug, Default, Clone, Component, ScorerBuilder)]
pub(crate) struct Scorer<T>(PhantomData<T>)
where
    T: Component + Clone + ActionBuilder + Default;

#[derive(Component, Clone, Copy, Default, PartialEq, Eq, Hash, Debug, From, Reflect)]
pub enum Target {
    #[default]
    None,
    Entity(Entity),
}

#[derive(Component, Default, ActionBuilder, Clone, Debug)]
pub(crate) struct Seek {
    pub attack_range: u32,
}

fn seeker_scorer(
    agents: Query<(Has<agent::DestinationReached>, &Target)>,
    mut actors: Query<(&Actor, &mut Score), With<Scorer<Seek>>>,
) {
    const DEFAULT_SEEK_SCORE: f32 = 1.0;
    for (Actor(actor), mut score) in &mut actors {
        let (destination_reached, target) = or_continue!(agents.get(*actor));
        score.set(if !destination_reached || matches!(target, Target::None) {
            DEFAULT_SEEK_SCORE
        } else {
            0.0
        });
    }
}

fn seek(
    mut units: Query<(
        &Transform,
        &mut Target,
        &mut path::Destination,
        &mut agent::Pathing,
        &mut agent::DestinationRange,
        Has<agent::DestinationReached>,
    )>,
    mut actors: Query<(&Actor, &mut ActionState, &Seek), With<Seek>>,
    unit_allegiance: Query<&Allegiance>,
    kd_tree: Res<UnitTree>,
) {
    const MAX_NEIBOURS: usize = 32;

    for (Actor(actor), mut state, seek) in &mut actors {
        let (
            transform,
            mut target,
            mut destination,
            mut pathing,
            mut destination_range,
            destination_reached,
        ) = or_continue_quiet!(units.get_mut(*actor));

        let allegiance = or_continue!(unit_allegiance.get(*actor));
        let position = transform.translation;

        match *state {
            ActionState::Requested => {
                if destination_reached {
                    *state = ActionState::Cancelled;
                    continue;
                }
                **destination_range = seek.attack_range;
                *state = ActionState::Executing;
                *pathing = agent::Pathing::Active;
            }
            ActionState::Executing => {
                **destination_range = seek.attack_range;
                let Some((_, target_entity)) = kd_tree
                    .k_nearest_neighbour(position, MAX_NEIBOURS)
                    .iter()
                    .filter(|(_, e)| {
                        let Some(other) = e else {
                            return false;
                        };

                        if other == actor {
                            return false;
                        }

                        unit_allegiance
                            .get(*other)
                            .ok()
                            .filter(|a| a.is_enemy(*allegiance))
                            .is_some()
                    })
                    .next()
                    .copied()
                else {
                    *state = ActionState::Failure;
                    continue;
                };

                let Some(target_entity) = target_entity else {
                    *state = ActionState::Failure;
                    continue;
                };

                if *target != Target::Entity(target_entity) {
                    *target = Target::Entity(target_entity);
                }

                if *destination != path::Destination::Entity(target_entity) {
                    *destination = path::Destination::Entity(target_entity);
                }

                if destination_reached {
                    *state = ActionState::Success;
                    continue;
                }
            }
            ActionState::Cancelled => {
                *state = ActionState::Failure;
            }
            ActionState::Failure | ActionState::Success => {
                *pathing = agent::Pathing::Inactive;
            }
            _ => {}
        }
    }
}

#[derive(Component, Default, ActionBuilder, Clone, Debug)]
pub(crate) struct Attack {
    pub damage: f32,
}

fn attack_scorer(
    units: Query<(Has<agent::DestinationReached>, &Target)>,
    mut actors: Query<(&Actor, &mut Score), With<Scorer<Attack>>>,
) {
    const DEFAULT_ATTACK_SCORE: f32 = 1.0;
    for (Actor(actor), mut score) in &mut actors {
        let (destination_reached, target) = or_continue!(units.get(*actor));
        score.set(
            if destination_reached && matches!(target, Target::Entity(_)) {
                DEFAULT_ATTACK_SCORE
            } else {
                0.0
            },
        );
    }
}

fn attack(
    units: Query<(&Target, Has<agent::DestinationReached>)>,
    mut actors: Query<(&Actor, &mut ActionState, &Attack), With<Attack>>,
    mut damage_event: EventWriter<DamageEvent>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();

    for (Actor(actor), mut state, attack) in &mut actors {
        let (target, destination_reached) = or_continue!(units.get(*actor));

        match *state {
            ActionState::Requested => {
                if !destination_reached {
                    *state = ActionState::Cancelled;
                    continue;
                }

                *state = ActionState::Executing;
            }
            ActionState::Executing => {
                let Target::Entity(target_entity) = target else {
                    *state = ActionState::Failure;
                    continue;
                };

                damage_event.send(DamageEvent {
                    source: *actor,
                    target: *target_entity,
                    damage: attack.damage * delta,
                });
            }
            ActionState::Cancelled => {
                *state = ActionState::Failure;
            }
            ActionState::Failure | ActionState::Success => {}
            _ => {}
        }
    }
}
