use crate::{
    navigation::{
        agent::{self, FollowPath},
        path,
    },
    prelude::*,
};
use bevy_spatial::SpatialAccess;
use big_brain::prelude::*;

use super::{Allegiance, UnitTree};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(BigBrainPlugin::new(PreUpdate));

    app.add_systems(PreUpdate, seek.in_set(BigBrainSet::Actions));
    app.add_systems(PreUpdate, seeker_scorer.in_set(BigBrainSet::Scorers));
}

lazy_static::lazy_static! {
    pub static ref UNIT_THINKER: ThinkerBuilder = Thinker::build()
        .picker(Highest)
        .when(Seeker, Seek);
}

#[derive(Debug, Clone, Component, ScorerBuilder)]
pub(crate) struct Seeker;

pub fn seeker_scorer(mut actors: Query<(&Actor, &mut Score), With<Seeker>>) {
    const DEFAULT_SEEK_SCORE: f32 = 1.0;
    for (Actor(actor), mut score) in &mut actors {
        score.set(DEFAULT_SEEK_SCORE);
    }
}

#[derive(Component, ActionBuilder, Clone, Debug)]
pub(crate) struct Seek;

pub(super) fn seek(
    mut commands: Commands,
    mut actors: Query<(&Actor, &mut ActionState, &Transform, &mut path::Target), With<Seek>>,
    units: Query<&Allegiance>,
    kd_tree: Res<UnitTree>,
) {
    const MAX_NEIBOURS: usize = 32;

    for (Actor(actor), mut state, transform, mut target) in &mut actors {
        let allegiance = or_continue!(units.get(*actor));
        let position = transform.translation;

        match *state {
            ActionState::Requested => {
                let Some((_, target_entity)) = kd_tree
                    .k_nearest_neighbour(position, MAX_NEIBOURS)
                    .iter()
                    .skip(0)
                    .filter(|(_, e)| {
                        let Some(other) = e else {
                            return false;
                        };
                        units
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

                *target = path::Target::Entity(target_entity);
                *state = ActionState::Executing;

                commands.entity(*actor).insert(FollowPath);
            }
            ActionState::Cancelled | ActionState::Failure | ActionState::Success => {
                commands.entity(*actor).remove::<agent::FollowPath>();
            }
            _ => {}
        }
    }
}

#[derive(Component, ActionBuilder, Clone, Debug)]
pub(crate) struct Attack;
