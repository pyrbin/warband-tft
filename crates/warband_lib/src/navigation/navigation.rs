//! TODO: refactor & re-add

use bevy::tasks::AsyncComputeTaskPool;
use hexx::{algorithms::a_star, Hex};

use crate::prelude::*;
use std::sync::{Arc, RwLock};

use super::{Board, Location};

#[derive(Component, Clone, Copy, Default, PartialEq, Eq, Hash, Debug, From, Reflect)]
#[reflect(Component)]
pub enum Target {
    #[default]
    None,
    Entity(Entity),
    Cell(Hex),
}

#[derive(Component, Default, Reflect)]
#[component(storage = "SparseSet")]
pub struct TargetReached;

#[derive(Component, Debug, Clone, Copy, Reflect)]
pub enum TargetReachedCondition {
    Distance(f32),
}

impl TargetReachedCondition {
    #[inline]
    pub fn has_reached_target(&self, target_distance: f32) -> bool {
        match self {
            TargetReachedCondition::Distance(distance) => target_distance < *distance,
        }
    }
}

pub(super) fn reconstruct_path(
    tracked: Query<(Entity, &Location), Changed<Location>>,
    paths: Query<Entity, With<Target>>,
    commands: ParallelCommands,
) {
    if tracked.is_empty() {
        return;
    }

    paths.par_iter().for_each(|entity| {
        commands.command_scope(|mut c| {
            c.entity(entity).insert(Dirty::<Path>::default());
        });
    });
}

#[derive(Component, Deref, DerefMut, Reflect, Default)]
#[reflect(Component)]
pub struct Path(Vec<Vec2>);

#[derive(Component)]
pub struct FindingPath(Arc<RwLock<(Option<Vec<Hex>>, bool)>>);

pub(super) fn compute(
    mut commands: Commands,
    with_target: Query<
        (Entity, &Target, &Location),
        (
            Or<(Changed<Target>, Changed<TargetReached>, With<Dirty<Path>>)>,
            Without<TargetReached>,
        ),
    >,
    with_location: Query<&Location>,
) {
    for (entity, target, location) in &with_target {
        let Location::Valid(start) = *location else {
            return;
        };

        let end = match target {
            Target::Cell(hex) => *hex,
            Target::Entity(entity) => {
                let Ok(location) = with_location.get(*entity) else {
                    return;
                };
                let Location::Valid(hex) = *location else {
                    return;
                };
                hex
            }
            Target::None => return,
        };

        if start == end {
            continue;
        }

        let finding = FindingPath(Arc::new(RwLock::new((None, false))));
        let writer = finding.0.clone();

        AsyncComputeTaskPool::get()
            .spawn(async move {
                let path = a_star(start, end, |_, _| 0.into());
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
    board: Res<Board>,
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

pub(super) fn target_reached(
    commands: ParallelCommands,
    mut agents: Query<(Entity, &Target, &TargetReachedCondition, Has<TargetReached>)>,
    transforms: Query<Ref<GlobalTransform>>,
    board: Res<Board>,
) {
    agents.par_iter_mut().for_each(
        |(entity, target, target_reached_condition, target_reached)| {
            let distance = match target {
                Target::Entity(target_entity) => {
                    let target_transform = or_return!(transforms.get(*target_entity));
                    let agent_transform = or_return!(transforms.get(entity));
                    let distance =
                        target_transform.translation().xz() - agent_transform.translation().xz();
                    distance.length()
                }
                Target::Cell(hex) => {
                    let agent_transform = or_return!(transforms.get(entity));
                    let distance =
                        board.layout.hex_to_world_pos(*hex) - agent_transform.translation().xz();
                    distance.length()
                }
                Target::None => 0.0,
            };

            commands.command_scope(|mut c| {
                if target_reached_condition.has_reached_target(distance) {
                    if !target_reached {
                        c.entity(entity).insert(TargetReached).remove::<Path>();
                    }
                } else if target_reached {
                    c.entity(entity).remove::<TargetReached>();
                }
            });
        },
    );
}

pub(super) fn moving(
    mut agents: Query<(Entity, &mut Transform, &mut Path)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut transform, mut path) in &mut agents {
        let next = path[0];
        let toward = next - transform.translation.xz();
        const SPEED: f32 = 5.0;
        let mut speed = time.delta_seconds() * SPEED;
        if toward.length() < speed {
            speed = toward.length();
            path.remove(0);
            if path.is_empty() {
                commands.entity(entity).remove::<Path>();
            }
        }

        transform.translation += (toward.normalize() * speed).x0y()
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
