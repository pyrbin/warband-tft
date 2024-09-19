use hexx::Hex;

use crate::{
    board::{Board, Location},
    physics::motor::MovementAction,
    prelude::*,
};

use super::path::{Path, TargetLocation};

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Agent {
    // TODO: navigation currently only supports size: 1 agents
    size: u8,
}

impl Default for Agent {
    fn default() -> Self {
        Self { size: 1 }
    }
}

impl Agent {
    pub fn with_size(size: u8) -> Self {
        assert!(size > 0);
        Self { size }
    }

    pub fn size(&self) -> u8 {
        self.size
    }
}

#[derive(Component, Default, Reflect, Deref, DerefMut, From)]
pub struct DesiredDirection(pub Option<Dir2>);

#[derive(Component, Default, Reflect)]
#[component(storage = "SparseSet")]
pub struct TargetReached;

#[derive(Component, Default, Debug, Clone, Copy, Reflect)]
pub enum TargetReachedCondition {
    #[default]
    Exact,
    Approximate,
    Distance(f32),
    Hexagonal(i32),
}

impl TargetReachedCondition {
    #[inline]
    pub fn has_reached_target(&self, distance_sq: f32, hexagonal_distance: i32) -> bool {
        match self {
            TargetReachedCondition::Distance(distance) => distance_sq.sqrt() < *distance,
            TargetReachedCondition::Exact => distance_sq.is_zero(),
            TargetReachedCondition::Approximate => distance_sq.is_approx_zero(),
            TargetReachedCondition::Hexagonal(distance) => hexagonal_distance <= *distance,
        }
    }
}

pub(super) fn target_reached(
    agents: Query<
        (
            Entity,
            &TargetLocation,
            &TargetReachedCondition,
            &Location,
            &Transform,
            Has<TargetReached>,
        ),
        Changed<Path>,
    >,
    mut commands: Commands,
    board: Res<Board>,
) {
    for (entity, target_location, target_reached_condition, location, transform, target_reached) in
        &agents
    {
        let TargetLocation::Value(target_location) = target_location else {
            return;
        };

        let to_world = |hex: Hex| board.layout.hex_to_world_pos(hex);

        let hexagonal_distance = if let Location::Valid(location) = *location {
            location.distance_to(*target_location)
        } else {
            i32::MAX
        };

        let target_in_world = to_world(*target_location);
        let distance = target_in_world.distance_squared(transform.translation.xz());

        if target_reached_condition.has_reached_target(distance, hexagonal_distance) {
            if !target_reached {
                commands
                    .entity(entity)
                    .insert(TargetReached)
                    .remove::<Path>();
            }
        } else if target_reached {
            commands.entity(entity).remove::<TargetReached>();
        }
    }
}

pub(super) fn navigate(
    mut agents: Query<
        (Entity, &mut DesiredDirection, &Transform, &mut Path),
        Without<TargetReached>,
    >,
    mut commands: Commands,
) {
    for (entity, mut desired_direction, transform, mut path) in &mut agents {
        let translation_xy = transform.translation.xz();

        if path.is_empty() {
            desired_direction.0 = None;
            commands.entity(entity).remove::<Path>();
            continue;
        }

        let next_waypoint = path[0];
        let toward = next_waypoint - translation_xy;
        let distance_to_waypoint = toward.length();
        const REACH_THRESHOLD: f32 = 0.1;

        // TODO: handle overshooting? maybe just use cell-to-cell movement instead of velocity?
        if distance_to_waypoint < REACH_THRESHOLD {
            path.remove(0);
        } else {
            let desired_dir = toward.normalize();
            desired_direction.0 = Some(Dir2::new_unchecked(desired_dir));
        }
    }
}

pub(super) fn moving(
    agents: Query<(Entity, &DesiredDirection), Without<TargetReached>>,
    mut commands: Commands,
) {
    for (entity, desired_direction) in &agents {
        if let Some(desired_direction) = desired_direction.0 {
            commands.trigger_targets(MovementAction::Move(desired_direction), entity);
        }
    }
}

pub(super) fn snap(
    trigger: Trigger<OnRemove, Path>,
    mut agents: Query<(&mut Transform, &Location), With<TargetReached>>,
    board: Res<Board>,
) {
    let entity = trigger.entity();
    let (mut transform, location) = or_return!(agents.get_mut(entity));
    let Location::Valid(location) = location else {
        return;
    };

    let world_pos = board.layout.hex_to_world_pos(*location);
    transform.translation = Vec3::new(world_pos.x, transform.translation.y, world_pos.y);
}

pub(super) fn reset_desired_direction(mut agents: Query<&mut DesiredDirection, Without<Path>>) {
    for mut desired_direction in &mut agents {
        desired_direction.0 = None;
    }
}
