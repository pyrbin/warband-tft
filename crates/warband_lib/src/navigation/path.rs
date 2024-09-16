use bevy::tasks::AsyncComputeTaskPool;
use hexx::{algorithms::a_star, Hex};

use crate::{
    board::{self, Footprint, Location},
    prelude::*,
};
use std::sync::{Arc, RwLock};

#[derive(Component, Clone, Copy, Default, PartialEq, Eq, Hash, Debug, From, Reflect)]
#[reflect(Component)]
pub enum Target {
    #[default]
    None,
    Entity(Entity),
    Cell(Hex),
}

// #FB_TODO: convert to enum to handle Target::None case
#[derive(Component, Clone, Copy, Default, From, Reflect, PartialEq)]
#[reflect(Component)]
pub enum TargetLocation {
    #[default]
    None,
    Value(Hex),
}

pub(super) fn target_location(
    mut with_target_location: Query<(&Target, &mut TargetLocation)>,
    with_location: Query<&Location>,
) {
    with_target_location
        .par_iter_mut()
        .for_each(|(target, mut target_location)| {
            let location: TargetLocation = match target {
                Target::Cell(hex) => TargetLocation::Value(*hex),
                Target::Entity(entity) => {
                    if let Ok(location) = with_location.get(*entity) {
                        if let Location::Valid(hex) = *location {
                            TargetLocation::Value(hex)
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
#[reflect(Component)]
pub struct Path(Vec<Vec2>);

#[derive(Component)]
pub struct FindingPath(Arc<RwLock<(Option<Vec<Hex>>, bool)>>);

pub(super) fn on_changed(
    mut commands: Commands,
    with_target: Query<Entity, With<Target>>,
    footprints: Query<Entity, Changed<Footprint>>,
    obstacles: Res<board::Occupied>,
) {
    if !obstacles.is_changed() || footprints.is_empty() {
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
) {
    for (entity, target, location) in &with_target {
        let Location::Valid(start) = *location else {
            return;
        };

        let TargetLocation::Value(end) = *target else {
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

        AsyncComputeTaskPool::get()
            .spawn(async move {
                let path = a_star(start, end, |_, b| {
                    if occupied.contains_key(&b) { 1 } else { 0 }.into()
                });
                *writer.write().unwrap() = (path, true);
            })
            .detach();

        commands
            .entity(entity)
            .insert(finding)
            .remove::<Dirty<Path>>();
    }
}

pub(super) fn poll(
    mut commands: Commands,
    computing: Query<(Entity, &FindingPath)>,
    board: Res<board::Board>,
) {
    for (entity, task) in &computing {
        let mut task = task.0.write().unwrap();
        if task.1 {
            if let Some(path) = task.0.take() {
                commands
                    .entity(entity)
                    .insert(Path(
                        path.iter()
                            .copied()
                            // #FB_NOTE: skip the first hex, which is the start hex
                            .skip(1)
                            .map(|h| board.layout.hex_to_world_pos(h))
                            .collect(),
                    ))
                    .remove::<FindingPath>();
            } else {
                info!("no path found");
            }
        }
    }
}

#[cfg(feature = "dev")]
pub(crate) fn gizmos(mut gizmos: Gizmos, entities: Query<(&Transform, &Path)>) {
    use bevy::color::palettes::css::PURPLE;
    for (transform, path) in &entities {
        if path.len() <= 0 {
            continue;
        }
        gizmos.line(transform.translation.horizontal(), path[0].x0y(), PURPLE);
        for i in 0..path.len() - 1 {
            let start = path[i];
            let end = path[i + 1];
            gizmos.line(start.x0y(), end.x0y(), PURPLE);
        }
    }
}
