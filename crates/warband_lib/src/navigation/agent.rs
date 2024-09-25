use hexx::Hex;

use crate::{
    board::{occupied::Occupant, Board, Location, Occupied},
    physics::motor::{CharacterMotor, MovementAction},
    prelude::*,
};

use super::path::{Path, Target, TargetLocation};

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
    pub fn size(&self) -> u8 {
        self.size
    }
}

#[derive(Component, Default, Reflect)]
#[component(storage = "SparseSet")]
pub struct FollowPath;

#[derive(Component, Default, Reflect)]
#[component(storage = "SparseSet")]
pub struct Arrived;

#[derive(Component, Default, Debug, Clone, Deref, DerefMut, Copy, Reflect)]
pub struct ArrivalThreshold(pub u32);

impl ArrivalThreshold {
    pub fn has_arrived(&self, distance: u32) -> bool {
        distance <= self.0
    }
}

#[derive(Component, Default, Debug, Clone, Copy, Reflect)]
pub struct Waypoint(pub Hex);

#[derive(Event, Debug, Clone, Copy, Reflect)]
pub enum PathingEvent {
    Arrived,
    NeedsUpdate,
}

pub(super) fn pathing(
    mut agents: Query<
        (
            Entity,
            &mut Path,
            &mut Location,
            &TargetLocation,
            &ArrivalThreshold,
        ),
        (
            Without<Waypoint>,
            With<Target>,
            With<Agent>,
            Without<Arrived>,
        ),
    >,
    occupied: Res<Occupied>,
    mut commands: Commands,
) {
    for (entity, mut path, mut location, target_location, arrival) in &mut agents {
        if path.is_empty() {
            commands.trigger_targets(PathingEvent::Arrived, entity);
            continue;
        }

        let TargetLocation::Valid(target_location) = target_location else {
            continue;
        };

        let distance = if let Location::Valid(location) = *location {
            location.unsigned_distance_to(*target_location)
        } else {
            u32::MAX
        };

        if arrival.has_arrived(distance) {
            commands.trigger_targets(PathingEvent::Arrived, entity);
            continue;
        }

        let next_waypoint = path[0];
        path.remove(0);

        if occupied.contains_key(&next_waypoint) {
            commands.trigger_targets(PathingEvent::NeedsUpdate, entity);
            continue;
        }

        *location = Location::Valid(next_waypoint);

        occupied
            .entry(next_waypoint)
            .or_default()
            .insert(Occupant::Agent(entity));

        commands.entity(entity).insert(Waypoint(next_waypoint));
    }
}

pub(super) fn arrived(
    mut agents: Query<
        (Entity, &Location, &TargetLocation, &ArrivalThreshold),
        (
            With<Agent>,
            With<Arrived>,
            Or<(Changed<TargetLocation>, Changed<Location>)>,
        ),
    >,
    mut commands: Commands,
) {
    for (entity, location, target_location, arrival) in &mut agents {
        let TargetLocation::Valid(target_location) = target_location else {
            continue;
        };

        let distance = if let Location::Valid(location) = *location {
            location.unsigned_distance_to(*target_location)
        } else {
            u32::MAX
        };

        if !arrival.has_arrived(distance) {
            commands.entity(entity).remove::<Arrived>();
        }
    }
}

pub(super) fn follow(
    mut agents: Query<
        (Entity, &Waypoint, &GlobalTransform),
        (
            With<Agent>,
            With<FollowPath>,
            With<CharacterMotor>,
            Without<Arrived>,
        ),
    >,
    mut commands: Commands,
    board: Res<Board>,
) {
    for (entity, waypoint, global_transform) in &mut agents {
        let waypoint = board.layout.hex_to_world_pos(waypoint.0).x0y();
        let distance = global_transform
            .translation()
            .horizontal()
            .distance(waypoint);

        // TODO: this might overshoot?
        const DISTANCE_MARGIN: f32 = 0.1;
        if distance < DISTANCE_MARGIN {
            commands.entity(entity).remove::<Waypoint>();
            continue;
        }

        commands.trigger_targets(MovementAction::Toward(waypoint), entity);
    }
}

pub(super) fn on_path_event(trigger: Trigger<PathingEvent>, mut commands: Commands) {
    let entity = trigger.entity();

    let mut entity_commands = or_return!(commands.get_entity(entity));

    match trigger.event() {
        PathingEvent::Arrived => {
            entity_commands
                .remove::<Path>()
                .remove::<Waypoint>()
                .insert(Arrived);
        }
        PathingEvent::NeedsUpdate => {
            entity_commands
                .remove::<Path>()
                .remove::<Waypoint>()
                .remove::<Arrived>()
                .insert(Dirty::<Path>::default());
        }
    }
}

#[cfg(feature = "dev")]
pub(crate) fn gizmos(
    mut gizmos: Gizmos,
    entities: Query<(&GlobalTransform, &Waypoint, &ArrivalThreshold)>,
    board: Res<Board>,
) {
    use bevy::color::palettes::css::LIGHT_CYAN;

    let to_world = |hex: Hex| board.layout.hex_to_world_pos(hex).x0y();

    for (global_transform, waypoint, arrival_range) in &entities {
        gizmos.rect(
            to_world(waypoint.0),
            Quat::from_rotation_x(PI / 2.),
            Vec2::ONE,
            LIGHT_CYAN,
        );
        gizmos.circle(
            global_transform.translation().horizontal(),
            Dir3::Y,
            **arrival_range as f32 * crate::board::HEX_SIZE_RATIO,
            LIGHT_CYAN,
        );
    }
}
