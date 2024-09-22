use hexx::Hex;

use crate::{
    board::{Board, Location},
    physics::motor::{CharacterMotor, MovementAction},
    prelude::*,
};

use super::path::{Path, TargetLocation};

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Agent {
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

#[derive(Component, Default, Reflect)]
#[component(storage = "SparseSet")]
pub struct TargetReached;

#[derive(Component, Default, Debug, Clone, Deref, DerefMut, Copy, Reflect)]
pub struct ArrivalRange(pub i32);

#[derive(Component, Default, Debug, Clone, Copy, Reflect)]
pub struct Waypoint(pub Hex);

// TODO:
// - need to allow updating path while moving to waypoint
// - need to handle collision between objects & agents gracefully
// - need to handle if path is obstructed by an obtacle update

pub(super) fn waypoint(
    mut agents: Query<
        (
            Entity,
            &mut Path,
            &mut Location,
            &TargetLocation,
            &ArrivalRange,
        ),
        Without<Waypoint>,
    >,
    mut commands: Commands,
) {
    for (entity, mut path, mut location, target_location, arrival_range) in &mut agents {
        if path.is_empty() {
            commands
                .entity(entity)
                .remove::<Path>()
                .remove::<Waypoint>()
                .insert(TargetReached);
            continue;
        }

        let TargetLocation::Value(target_location) = target_location else {
            continue;
        };

        let distance = if let Location::Valid(location) = *location {
            location.distance_to(*target_location)
        } else {
            i32::MAX
        };

        if distance <= **arrival_range {
            commands
                .entity(entity)
                .remove::<Path>()
                .remove::<Waypoint>()
                .insert(TargetReached);
            continue;
        }

        let next_waypoint = path[0];
        path.remove(0);

        *location = Location::Valid(next_waypoint);
        commands
            .entity(entity)
            .insert(Waypoint(next_waypoint))
            .remove::<TargetReached>();
    }
}

pub(super) fn moving(
    mut agents: Query<(Entity, &Waypoint, &GlobalTransform), (With<Agent>, With<CharacterMotor>)>,
    mut commands: Commands,
    board: Res<Board>,
) {
    for (entity, waypoint, global_transform) in &mut agents {
        let waypoint = board.layout.hex_to_world_pos(waypoint.0).x0y();
        let distance = global_transform
            .translation()
            .horizontal()
            .distance(waypoint);

        const DISTANCE_MARGIN: f32 = 0.1;
        if distance < DISTANCE_MARGIN {
            commands.entity(entity).remove::<Waypoint>();
            continue;
        }

        commands.trigger_targets(MovementAction::Toward(waypoint), entity);
    }
}

#[cfg(feature = "dev")]
pub(crate) fn gizmos(
    mut gizmos: Gizmos,
    entities: Query<(&GlobalTransform, &Waypoint, &ArrivalRange)>,
    board: Res<Board>,
) {
    use bevy::color::palettes::css::GREEN;

    let to_world = |hex: Hex| board.layout.hex_to_world_pos(hex).x0y();

    for (global_transform, waypoint, arrival_range) in &entities {
        let position = global_transform.translation().horizontal();
        gizmos.circle(to_world(waypoint.0), Dir3::Y, 0.7, GREEN);
        gizmos.circle(
            global_transform.translation().horizontal(),
            Dir3::Y,
            **arrival_range as f32 * 1.44,
            GREEN,
        );

        gizmos.circle(position, Dir3::Y, 0.5, YELLOW);
        gizmos.line(position, position + 1.0 * Vec3::Y, YELLOW);
        gizmos.circle(position + 1.0 * Vec3::Y, Dir3::Y, 0.5, YELLOW);
    }
}
