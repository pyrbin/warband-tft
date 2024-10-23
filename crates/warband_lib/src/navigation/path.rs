//! TODO: Should try out NavMesh pathfinding instead of cell-based.

use std::sync::{Arc, RwLock};

use bevy::tasks::AsyncComputeTaskPool;
use hexx::{algorithms::a_star, Hex};

use super::agent::{self};
use crate::{
    board::{
        occupied::Occupant,
        Location,
        {self},
    },
    prelude::*,
};

#[derive(Component, Clone, Copy, Default, PartialEq, Eq, Hash, Debug, From, Reflect)]
pub enum Destination {
    #[default]
    None,
    Entity(Entity),
    Cell(Hex),
}

#[derive(Component, Clone, Copy, Default, From, Reflect, PartialEq)]
pub enum DestinationLocation {
    #[default]
    None,
    Valid(Hex),
}

pub(super) fn target_location(
    mut with_destination: Query<(&Destination, &mut DestinationLocation)>,
    with_location: Query<&Location>,
) {
    with_destination
        .par_iter_mut()
        .for_each(|(destination, mut destination_location)| {
            let location: DestinationLocation = match destination {
                Destination::Cell(hex) => DestinationLocation::Valid(*hex),
                Destination::Entity(entity) => {
                    if let Ok(location) = with_location.get(*entity) {
                        if let Location::Valid(hex) = *location {
                            DestinationLocation::Valid(hex)
                        } else {
                            DestinationLocation::None
                        }
                    } else {
                        DestinationLocation::None
                    }
                },
                Destination::None => DestinationLocation::None,
            };

            if *destination_location != location {
                *destination_location = location;
            }
        });
}

#[derive(Component, Deref, DerefMut, Reflect, Default)]
pub struct Path(Vec<Hex>);

#[derive(Component)]
pub struct FindingPath(Arc<RwLock<(Option<Vec<Hex>>, bool)>>);

pub(super) fn on_occupied_changed(
    mut commands: Commands,
    with_destination: Query<Entity, (With<Destination>, Without<agent::DestinationReached>)>,
    occupied: Res<board::Occupied>,
) {
    if !occupied.is_changed() {
        return;
    }

    for entity in &with_destination {
        commands.entity(entity).insert(Dirty::<Path>::default());
    }
}

pub(super) fn on_destination_changed(
    mut commands: Commands,
    with_destination: Query<Entity, Or<(Changed<Destination>, Changed<DestinationLocation>)>>,
) {
    for entity in &with_destination {
        commands.entity(entity).insert(Dirty::<Path>::default());
    }
}

pub(super) fn compute(
    mut commands: Commands,
    with_destination: Query<(Entity, &DestinationLocation, &Location), With<Dirty<Path>>>,
    occupied: Res<board::Occupied>,
    board: Res<board::Board>,
) {
    const DEFAULT_COST: u32 = 0;
    const DEFAULT_OCCUPANT_COST: u32 = 1;

    for (entity, destination_location, location) in &with_destination {
        let Location::Valid(start) = *location else {
            return;
        };

        let DestinationLocation::Valid(end) = *destination_location else {
            commands.entity(entity).remove::<Path>();
            continue;
        };

        if start == end {
            commands.entity(entity).remove::<Path>();
            continue;
        }

        let finding = FindingPath(Arc::new(RwLock::new((None, false))));
        let writer = finding.0.clone();
        let occupied = occupied.clone();
        let bounds = board.bounds.clone();

        AsyncComputeTaskPool::get()
            .spawn(async move {
                // perf: can be improved, might be good enough tho
                let shortest_path = |ignore_occupants: bool| {
                    a_star(start, end, |_, b| {
                        if !bounds.is_in_bounds(b) {
                            None
                        } else if b == end {
                            Some(DEFAULT_COST)
                        } else if occupied.contains_key(&b) {
                            if let Some(occupants) = occupied.get(&b)
                                && (occupants.contains(&Occupant::Agent(entity))
                                    && occupants.len() == 1)
                            {
                                return Some(DEFAULT_COST);
                            }

                            if ignore_occupants
                                && let Some(occupants) = occupied.get(&b)
                                && !occupants.iter().any(|o| matches!(o, Occupant::Obstacle(_)))
                            {
                                Some(DEFAULT_OCCUPANT_COST)
                            } else {
                                None
                            }
                        } else {
                            Some(DEFAULT_COST)
                        }
                    })
                };

                // 1. find path around occupants.
                // 2. if no path; find path ignoring agent occupants, to get close to the target.
                // 3. determine if new path ignoring occupants is closer than current position
                let mut result_path = shortest_path(false);

                if result_path.is_none() {
                    result_path = shortest_path(true);
                    if let Some(path) = result_path.clone()
                        && let Some((p, _)) = path.iter().rev().copied().find_position(|h| {
                            *h != start && *h != end && !occupied.contains_key(h)
                        })
                    {
                        let index = path.len() - p;
                        let distance = start.distance_to(end);
                        let new_distance = path[index].distance_to(end);
                        if distance < new_distance
                            || (path.len() / 3) as i32 > distance - new_distance
                        {
                            result_path = None;
                        }
                    }
                }

                *writer.write().unwrap() = (result_path, true);
            })
            .detach();

        commands
            .entity(entity)
            .insert(finding)
            .remove::<Dirty<Path>>();
    }
}

pub(super) fn poll(mut commands: Commands, computing: Query<(Entity, &FindingPath)>) {
    for (entity, task) in &computing {
        let mut task = task.0.write().unwrap();
        if task.1 {
            if let Some(path) = task.0.take() {
                let skip = if path.len() < 2 { 0 } else { 1 };
                commands
                    .entity(entity)
                    .insert(Path(path.iter()
                            .copied()
                            .skip(skip) // skip start hex
                            .collect()))
                    .remove::<FindingPath>();
            }
        }
    }
}

#[cfg(feature = "dev")]
pub(crate) fn gizmos(
    mut gizmos: Gizmos,
    entities: Query<(&Transform, &Path)>,
    board: Res<board::Board>,
) {
    use bevy::color::palettes::css::LIGHT_PINK;

    let to_world = |hex: Hex| board.layout.hex_to_world_pos(hex);

    for (transform, path) in &entities {
        if path.len() <= 0 {
            continue;
        }

        let mut segments = vec![transform.translation.horizontal()];
        segments.extend(path.iter().copied().map(|hex| to_world(hex).x0y()));

        for i in 1..segments.len() - 1 {
            gizmos.circle(segments[i], Dir3::Y, 0.1, LIGHT_PINK);
        }

        gizmos.linestrip(segments, LIGHT_PINK);
    }
}
