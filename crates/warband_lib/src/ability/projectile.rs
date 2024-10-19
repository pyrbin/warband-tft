use ballistics_math::solve_ballistic_arc;
use bevy::ecs::entity::EntityHashSet;
use spawn::SpawnSystemId;

use crate::{
    prelude::*,
    unit::{Allegiance, Unit},
};

use super::Speed;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<ProjectileEvent>();

    app.add_systems(Update, gravity);
}

#[derive(Bundle, Default)]
pub(crate) struct ProjectileBundle {
    pub projectile: Projectile,
    pub projectile_type: ProjectileType,
    pub projectile_target: ProjectileTarget,
    pub entites_hit: EntitiesHit,
    pub velocity: LinearVelocity,
    pub collider: Collider,
    pub collision_layers: CollisionLayers,
    pub sensor: Sensor,
    pub speed: Speed,
}

#[derive(Component, Default)]
pub(crate) struct Projectile;

#[derive(Reflect, Component, Clone, Debug, Default)]
#[reflect(Component, Debug)]
pub(crate) enum ProjectileType {
    Linear,
    #[default]
    Tracking,
}

#[derive(Component)]
pub(crate) struct AbilityTriggerEmitter(pub Entity);

#[derive(Component, Default, Deref, DerefMut)]
pub(crate) struct EntitiesHit(pub EntityHashSet);

#[derive(Component, Default, DerefMut, Deref, From)]
pub(crate) struct AllegianceFilter(Allegiance);

#[derive(Reflect, Component, Clone, Debug)]
pub(crate) enum ProjectileTarget {
    Entity(Entity),
    Point(Vec3),
}

impl Default for ProjectileTarget {
    fn default() -> Self {
        Self::Point(Vec3::ZERO)
    }
}

#[derive(Event, Debug, Clone)]
pub(crate) enum ProjectileEvent {
    Spawned { projectile: Entity },
    Hit { projectile: Entity, target: Entity },
    Destroyed { projectile: Entity },
}

pub(crate) struct TrackingProjectile {
    pub(crate) projectile_target: ProjectileTarget,
    pub(crate) projectile_type: ProjectileType,
    pub(crate) filter: Allegiance,
    pub(crate) radius: f32,
    pub(crate) speed: f32,
    pub(crate) origin: Vec3,
}

// TODO: proc-macro
impl FromWorld for SpawnSystemId<TrackingProjectile> {
    fn from_world(world: &mut World) -> Self {
        Self {
            id: world.register_system(spawn_tracking),
        }
    }
}

fn spawn_tracking(
    In((id, args)): In<(Entity, TrackingProjectile)>,
    mut commands: Commands,
    transforms: Query<&GlobalTransform>,
    gravity: Res<Gravity>,
    time: Res<Time>,
) {
    let TrackingProjectile {
        projectile_target,
        projectile_type,
        filter,
        radius,
        speed,
        origin,
    } = args;

    let target = match projectile_target {
        ProjectileTarget::Entity(entity) => or_return!(global_translation(&transforms, entity)),
        ProjectileTarget::Point(point) => point,
    };

    let gravity = gravity.0.y * time.delta_seconds();
    let result = solve_ballistic_arc(origin, speed, target, gravity);

    let velocity = match result {
        Some((s0, s1)) => {
            if s0.y > s1.y {
                s0
            } else {
                s1
            }
        }
        None => {
            error!("failed to solve ballistic arc");
            return;
        }
    };

    commands
        .entity(id)
        .insert(ProjectileBundle {
            projectile: Projectile,
            projectile_type,
            projectile_target,
            collider: Collider::sphere(radius),
            velocity: LinearVelocity(velocity),
            ..default()
        })
        .insert((RigidBody::Kinematic, AllegianceFilter(filter)));
}

fn gravity(
    time: Res<Time>,
    mut motors: Query<(Option<&GravityScale>, &mut LinearVelocity), With<Projectile>>,
    gravity: Res<Gravity>,
) {
    let delta_time = time.delta_seconds();
    for (gravity_scale, mut linear_velocity) in &mut motors {
        let scale = gravity_scale.map(|s| s.0).unwrap_or(1.0);
        linear_velocity.0 += gravity.0 * scale * delta_time;
    }
}

fn collisions(
    mut commands: Commands,
    mut collisions: EventReader<Collision>,
    mut projectile_events: EventWriter<ProjectileEvent>,
    mut projectiles: Query<
        (
            Entity,
            &AllegianceFilter,
            &mut EntitiesHit,
            &ProjectileTarget,
        ),
        (With<Projectile>, Without<Despawn>),
    >,
    units: Query<(Entity, &Allegiance), With<Unit>>,
) {
    let mut on_collision = |projectile: Entity, unit: Entity| -> Result<(), ()> {
        let (projectile, filter, mut entities_hit, projectile_target) =
            projectiles.get_mut(projectile).map_err(|_| ())?;
        let (unit, allegiance) = units.get(unit).map_err(|_| ())?;

        if entities_hit.contains(&unit) {
            return Err(());
        }

        if !filter.contains(*allegiance) {
            return Err(());
        }

        projectile_events.send(ProjectileEvent::Hit {
            projectile,
            target: unit,
        });

        if let ProjectileTarget::Entity(target) = projectile_target
            && unit == *target
        {
            projectile_events.send(ProjectileEvent::Destroyed { projectile });
            commands.entity(projectile).insert(Despawn::WaitFrames(2));
        }

        entities_hit.insert(unit);

        Ok(())
    };

    for Collision(contacts) in collisions.read() {
        let _ = on_collision(contacts.entity2, contacts.entity1)
            .or(on_collision(contacts.entity1, contacts.entity2));
    }
}

fn tracking_trajectory(
    mut projectile: Query<
        (
            &ProjectileType,
            &ProjectileTarget,
            &mut LinearVelocity,
            &Transform,
            &Speed,
        ),
        With<ProjectileType>,
    >,
    transforms: Query<&GlobalTransform>,
    gravity: Res<Gravity>,
    time: Res<Time>,
) {
    let gravity = gravity.0.y * time.delta_seconds();

    for (projectile_type, projectile_target, mut linvel, transform, speed) in &mut projectile {
        if !matches!(projectile_type, ProjectileType::Tracking) {
            return;
        }

        let target = match projectile_target {
            ProjectileTarget::Entity(entity) => {
                or_return!(global_translation(&transforms, *entity))
            }
            ProjectileTarget::Point(point) => *point,
        };

        let position = transform.translation;
        let result = solve_ballistic_arc(position, speed.value(), target, gravity);
        let velocity = match result {
            Some((s0, s1)) => {
                if s0.y > s1.y {
                    s0
                } else {
                    s1
                }
            }
            None => {
                error!("failed to solve ballistic arc");
                return;
            }
        };

        linvel.0 = velocity;
    }
}
