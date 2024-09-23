use bevy::tasks::AsyncComputeTaskPool;
use hexx::{algorithms::a_star, Hex};

use crate::{
    board::{self, occupied::Occupant, Location},
    prelude::*,
};
use std::sync::{Arc, RwLock};

use super::agent::Arrived;

#[derive(Component, Clone, Copy, Default, PartialEq, Eq, Hash, Debug, From, Reflect)]
pub enum Target {
    #[default]
    None,
    Entity(Entity),
    Cell(Hex),
}

#[derive(Component, Clone, Copy, Default, From, Reflect, PartialEq)]
pub enum TargetLocation {
    #[default]
    None,
    Valid(Hex),
}

pub(super) fn target_location(
    mut with_target_location: Query<(&Target, &mut TargetLocation)>,
    with_location: Query<&Location>,
) {
    with_target_location
        .par_iter_mut()
        .for_each(|(target, mut target_location)| {
            let location: TargetLocation = match target {
                Target::Cell(hex) => TargetLocation::Valid(*hex),
                Target::Entity(entity) => {
                    if let Ok(location) = with_location.get(*entity) {
                        if let Location::Valid(hex) = *location {
                            TargetLocation::Valid(hex)
                        } else {
                            TargetLocation::None
                        }
                    } else {
                        TargetLocation::None
                    }
                }
                Target::None => TargetLocation::None,
            };

            if *target_location != location {
                *target_location = location;
            }
        });
}

#[derive(Component, Deref, DerefMut, Reflect, Default)]
pub struct Path(Vec<Hex>);

#[derive(Component)]
pub struct FindingPath(Arc<RwLock<(Option<Vec<Hex>>, bool)>>);

pub(super) fn on_changed(
    mut commands: Commands,
    with_target: Query<Entity, (With<Target>, Without<Arrived>)>,
    obstacles: Res<board::Occupied>,
) {
    if !obstacles.is_changed() {
        return;
    }

    for entity in &with_target {
        commands.entity(entity).insert(Dirty::<Path>::default());
    }
}

pub(super) fn on_target_changed(
    mut commands: Commands,
    with_target: Query<Entity, Or<(Changed<Target>, Changed<TargetLocation>)>>,
) {
    for entity in &with_target {
        commands.entity(entity).insert(Dirty::<Path>::default());
    }
}

pub(super) fn compute(
    mut commands: Commands,
    with_target: Query<(Entity, &TargetLocation, &Location), With<Dirty<Path>>>,
    occupied: Res<board::Occupied>,
    board: Res<board::Board>,
) {
    const DEFAULT_COST: u32 = 0;
    const DEFAULT_OCCUPANT_COST: u32 = 1;

    for (entity, target, location) in &with_target {
        let Location::Valid(start) = *location else {
            return;
        };

        let TargetLocation::Valid(end) = *target else {
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

                        // TODO: fine tune this
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
                    .insert(Path(
                        path.iter()
                            .copied()
                            .skip(skip) // skip start hex
                            .collect(),
                    ))
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
