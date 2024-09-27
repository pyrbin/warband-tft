use hexx::Hex;

use crate::{
    board::{occupied::Occupant, Board, Location, Occupied},
    physics::motor::{CharacterMotor, MovementAction},
    prelude::*,
};

use super::path::{Destination, DestinationLocation, Path};

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
pub enum Pathing {
    #[default]
    Active,
    Inactive,
}

#[derive(Event, Debug, Clone, Copy, Reflect)]
pub enum PathingEvent {
    DestinationReached,
    NeedsUpdate,
}

#[derive(Component, Default, Reflect)]
#[component(storage = "SparseSet")]
pub struct DestinationReached;

#[derive(Component, Default, Debug, Clone, Deref, DerefMut, Copy, Reflect)]
pub struct DestinationRange(pub u32);

impl DestinationRange {
    pub fn within_range(&self, distance: u32) -> bool {
        distance <= self.0
    }
}

#[derive(Component, Default, Debug, Clone, Copy, Reflect)]
pub struct Waypoint(pub Hex);

pub(super) fn waypoint(
    mut agents: Query<
        (
            Entity,
            &mut Path,
            &mut Location,
            &DestinationLocation,
            &DestinationRange,
        ),
        (
            Without<Waypoint>,
            With<Destination>,
            With<Agent>,
            Without<DestinationReached>,
        ),
    >,
    occupied: Res<Occupied>,
    mut commands: Commands,
) {
    for (entity, mut path, mut location, destination_location, destination_range) in &mut agents {
        if path.is_empty() {
            commands.trigger_targets(PathingEvent::DestinationReached, entity);
            continue;
        }

        let DestinationLocation::Valid(destination_location) = destination_location else {
            continue;
        };

        let distance = if let Location::Valid(location) = *location {
            location.unsigned_distance_to(*destination_location)
        } else {
            u32::MAX
        };

        if destination_range.within_range(distance) {
            commands.trigger_targets(PathingEvent::DestinationReached, entity);
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

pub(super) fn destination_reached(
    mut agents: Query<
        (Entity, &Location, &DestinationLocation, &DestinationRange),
        (
            With<Agent>,
            With<DestinationReached>,
            Or<(Changed<DestinationLocation>, Changed<Location>)>,
        ),
    >,
    mut commands: Commands,
) {
    for (entity, location, destination_location, destination_range) in &mut agents {
        let DestinationLocation::Valid(destination_location) = destination_location else {
            continue;
        };

        let distance = if let Location::Valid(location) = *location {
            location.unsigned_distance_to(*destination_location)
        } else {
            u32::MAX
        };

        if !destination_range.within_range(distance) {
            commands.trigger_targets(PathingEvent::NeedsUpdate, entity);
        }
    }
}

pub(super) fn pathing(
    mut agents: Query<
        (Entity, &Waypoint, &Pathing, &GlobalTransform),
        (
            With<Agent>,
            With<CharacterMotor>,
            Without<DestinationReached>,
        ),
    >,
    mut commands: Commands,
    board: Res<Board>,
) {
    for (entity, waypoint, pathing, global_transform) in &mut agents {
        if matches!(pathing, Pathing::Inactive) {
            continue;
        }

        let waypoint = board.layout.hex_to_world_pos(waypoint.0).x0y();
        let distance = global_transform
            .translation()
            .horizontal()
            .distance(waypoint);

        const DISTANCE_MARGIN: f32 = 0.1;

        if distance < DISTANCE_MARGIN {
            commands.entity(entity).remove::<Waypoint>();
            commands.trigger_targets(MovementAction::Teleport(waypoint), entity);
            continue;
        }

        commands.trigger_targets(MovementAction::Towards(waypoint), entity);
    }
}

pub(super) fn on_path_event(trigger: Trigger<PathingEvent>, mut commands: Commands) {
    let entity = trigger.entity();

    let mut entity_commands = or_return!(commands.get_entity(entity));

    match trigger.event() {
        PathingEvent::DestinationReached => {
            entity_commands
                .remove::<Path>()
                .remove::<Waypoint>()
                .insert(DestinationReached);
        }
        PathingEvent::NeedsUpdate => {
            entity_commands
                .remove::<Path>()
                .remove::<Waypoint>()
                .remove::<DestinationReached>()
                .insert(Dirty::<Path>::default());
        }
    }
}

#[cfg(feature = "dev")]
pub(crate) fn gizmos(
    mut gizmos: Gizmos,
    entities: Query<(&GlobalTransform, &Waypoint, &DestinationRange)>,
    board: Res<Board>,
) {
    use bevy::color::palettes::css::LIGHT_CYAN;

    let to_world = |hex: Hex| board.layout.hex_to_world_pos(hex).x0y();

    for (global_transform, waypoint, destination_range) in &entities {
        gizmos.rect(
            to_world(waypoint.0),
            Quat::from_rotation_x(PI / 2.),
            Vec2::ONE,
            LIGHT_CYAN,
        );
        gizmos.circle(
            global_transform.translation().horizontal(),
            Dir3::Y,
            **destination_range as f32 * crate::board::HEX_SIZE_RATIO,
            LIGHT_CYAN,
        );
    }
}
