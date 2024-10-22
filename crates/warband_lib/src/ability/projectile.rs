use ballistics_math::aim_projectile_straight_fallback;
use bevy::ecs::entity::EntityHashSet;

use crate::{
    physics,
    prelude::*,
    unit::{Allegiance, Unit},
};

use super::Speed;

pub(super) fn plugin(app: &mut App) {
    app_register_types!(
        Projectile,
        ProjectileType,
        EntitiesHit,
        AllegianceFilter,
        Target,
        ProjectileEvent
    );

    app.add_event::<ProjectileEvent>();
    app.add_systems(Update, (trajectory, collisions));
}

#[derive(Bundle, Default)]
pub(crate) struct ProjectileBundle {
    pub projectile: Projectile,
    pub projectile_type: ProjectileType,
    pub projectile_target: Target,
    pub entites_hit: EntitiesHit,
    pub velocity: LinearVelocity,
    pub collider: Collider,
    pub collision_layers: CollisionLayers,
    pub sensor: Sensor,
    pub speed: Speed,
}

#[derive(Component, Default, Reflect)]
pub(crate) struct Projectile;

#[derive(Reflect, Component, Clone, Debug, Default)]
#[reflect(Component, Debug)]
pub(crate) enum ProjectileType {
    Linear,
    #[default]
    Tracking,
}

#[derive(Component, Default, Deref, DerefMut, Reflect)]
pub(crate) struct EntitiesHit(pub EntityHashSet);

#[derive(Component, Reflect, Default, DerefMut, Deref, From)]
pub(crate) struct AllegianceFilter(Allegiance);

#[derive(Event, Reflect, Debug, Clone)]
pub(crate) enum ProjectileEvent {
    Spawned { projectile: Entity },
    Hit { projectile: Entity, target: Entity },
    Destroyed { projectile: Entity },
}

pub(crate) struct TrackingProjectile {
    pub(crate) projectile_target: Target,
    pub(crate) projectile_type: ProjectileType,
    pub(crate) filter: Allegiance,
    pub(crate) radius: f32,
    pub(crate) speed: f32,
    pub(crate) origin: Vec3,
}

#[spawner(TrackingProjectile)]
fn spawn_tracking(
    In((id, args)): In<(Entity, TrackingProjectile)>,
    mut commands: Commands,
    mut projectile_events: EventWriter<ProjectileEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    transforms: Query<&GlobalTransform>,
    gravity: Res<Gravity>,
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
        Target::Entity(entity) => or_return!(global_translation(&transforms, entity)),
        Target::Point(point) => point,
    };

    let velocity = aim_projectile_straight_fallback(target - origin, Vec3::ZERO, speed, gravity.0);

    commands
        .entity(id)
        .insert(ProjectileBundle {
            projectile: Projectile,
            projectile_type,
            projectile_target,
            collider: Collider::sphere(radius),
            velocity: LinearVelocity(velocity),
            speed: Speed(speed),
            collision_layers: CollisionLayers::new(
                [physics::Layer::Projectile],
                [physics::Layer::Units],
            ),
            ..default()
        })
        .insert((
            RigidBody::Kinematic,
            AllegianceFilter(filter),
            Despawn::Delay(30.0),
        ))
        .insert(PbrBundle {
            mesh: meshes.add(Sphere::new(radius)),
            material: materials.add(Color::srgb_u8(124, 144, 255)),
            transform: Transform::from_translation(origin),
            ..default()
        });

    projectile_events.send(ProjectileEvent::Spawned { projectile: id });
}

fn trajectory(
    time: Res<Time>,
    mut projectiles: Query<
        (
            &ProjectileType,
            Option<&GravityScale>,
            &mut LinearVelocity,
            &mut Transform,
            &Target,
            &Speed,
        ),
        With<Projectile>,
    >,
    transforms: Query<(&GlobalTransform, Option<&LinearVelocity>), Without<Projectile>>,
    gravity: Res<Gravity>,
) {
    let delta_time = time.delta_seconds();
    for (
        projectile_type,
        gravity_scale,
        mut linear_velocity,
        transform,
        projectile_target,
        speed,
    ) in &mut projectiles
    {
        let scale = gravity_scale.map(|s| s.0).unwrap_or(1.0);
        linear_velocity.0 += gravity.0 * scale * delta_time;

        if let Target::Entity(entity) = projectile_target
            && matches!(projectile_type, ProjectileType::Tracking)
        {
            let (target_transform, target_vel) = or_return!(transforms.get(*entity));
            let projectile_translation = transform.translation;
            linear_velocity.0 = aim_projectile_straight_fallback(
                target_transform.translation() - projectile_translation,
                target_vel.map(|v| v.0).unwrap_or_default() - linear_velocity.0,
                speed.value(),
                gravity.0,
            );
        }
    }
}

fn collisions(
    mut commands: Commands,
    mut collisions: EventReader<Collision>,
    mut projectile_events: EventWriter<ProjectileEvent>,
    mut projectiles: Query<
        (Entity, &AllegianceFilter, &mut EntitiesHit, &Target),
        (With<Projectile>, Without<Disabled<Projectile>>),
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

        if let Target::Entity(target) = projectile_target
            && unit == *target
        {
            projectile_events.send(ProjectileEvent::Destroyed { projectile });
            commands
                .entity(projectile)
                .insert(Despawn::WaitFrames(2))
                .insert(Disabled::<Projectile>::default());
        }

        entities_hit.insert(unit);

        Ok(())
    };

    for Collision(contacts) in collisions.read() {
        let _ = on_collision(contacts.entity2, contacts.entity1)
            .or(on_collision(contacts.entity1, contacts.entity2));
    }
}

#[cfg(feature = "dev")]
pub(crate) fn gizmos(mut gizmos: Gizmos, entities: Query<(&Target, &GlobalTransform)>) {
    use bevy::color::palettes::css::LIGHT_CYAN;
    for (target, transform) in &entities {
        if let Target::Point(point) = target {
            gizmos.line(transform.translation(), *point, LIGHT_CYAN);
            gizmos.sphere(*point, Quat::IDENTITY, 0.1, LIGHT_CYAN);
        }
    }
}
