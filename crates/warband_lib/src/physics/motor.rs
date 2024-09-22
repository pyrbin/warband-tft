use core::f32;

use crate::prelude::*;

#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MotorSystems {
    Motor,
    Collisions,
}

pub(super) fn plugin(app: &mut App) {
    app_register_types!(
        CharacterMotor,
        MaxSlopeAngle,
        Grounded,
        MovementAction,
        Movement
    );

    app.configure_sets(Update, MotorSystems::Motor);
    app.configure_sets(PostProcessCollisions, MotorSystems::Collisions);

    app.observe(movement);

    app.add_systems(
        Update,
        (grounded, gravity, damping)
            .chain()
            .in_set(MotorSystems::Motor),
    );

    app.add_systems(
        // TODO: Replace with a collider-and-slide
        PostProcessCollisions,
        (collisions).chain().in_set(MotorSystems::Collisions),
    );
}

#[derive(Bundle)]
pub struct CharacterMotorBundle {
    motor: CharacterMotor,
    rigid_body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    gravity: GravityScale,
    max_slope_angle: MaxSlopeAngle,
    damping_factor: DampingFactor,
}

impl CharacterMotorBundle {
    pub fn new(radius: f32, length: f32) -> Self {
        let collider = Collider::capsule(radius, length);
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vec3::ONE * 0.99, 10);

        Self {
            motor: CharacterMotor,
            rigid_body: RigidBody::Kinematic,
            collider,
            ground_caster: ShapeCaster::new(caster_shape, Vec3::ZERO, Quat::default(), Dir3::NEG_Y),
            gravity: GravityScale(1.0),
            max_slope_angle: MaxSlopeAngle(f32::consts::FRAC_PI_2),
            damping_factor: DampingFactor(0.9),
        }
    }
}

#[derive(Component, Reflect)]
pub struct CharacterMotor;

#[derive(Event, Reflect)]
pub enum MovementAction {
    Toward(Vec3),
    Dir(Dir2),
    Jump,
}

#[derive(Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Grounded;

#[derive(Component, Deref, DerefMut, From, Reflect)]
pub struct MaxSlopeAngle(f32);

#[derive(Component, Deref, DerefMut, From, Reflect)]
pub struct DampingFactor(f32);

#[derive(Stat, Component, Default, Reflect, Copy, Clone)]
#[clamp(min = 0.0)]
pub struct Movement(pub f32);

fn grounded(
    mut commands: Commands,
    mut motors: Query<
        (
            Entity,
            &ShapeHits,
            &Rotation,
            Option<&MaxSlopeAngle>,
            Has<Grounded>,
        ),
        (With<CharacterMotor>, Changed<Position>),
    >,
) {
    for (entity, hits, rotation, max_slope_angle, grounded) in &mut motors {
        let is_grounded = hits.iter().any(|hit| {
            if let Some(angle) = max_slope_angle {
                (rotation * -hit.normal2).angle_between(Vec3::Y).abs() <= angle.0
            } else {
                true
            }
        });

        if is_grounded && !grounded {
            commands.entity(entity).insert(Grounded);
        } else if grounded {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

fn gravity(
    time: Res<Time>,
    mut motors: Query<(Option<&GravityScale>, &mut LinearVelocity), With<CharacterMotor>>,
    gravity: Res<Gravity>,
) {
    let delta_time = time.delta_seconds();

    for (gravity_scale, mut linear_velocity) in &mut motors {
        let scale = gravity_scale.map(|s| s.0).unwrap_or(1.0);
        linear_velocity.0 += gravity.0 * scale * delta_time;
    }
}

fn movement(
    trigger: Trigger<MovementAction>,
    time: Res<Time>,
    mut motors: Query<(&Movement, &Transform, &mut LinearVelocity, Has<Grounded>)>,
) {
    let delta_time = time.delta_seconds();
    let entity = trigger.entity();

    let (movement, transform, mut linear_velocity, grounded) = or_return!(motors.get_mut(entity));
    let move_speed = movement.value() * delta_time;
    match trigger.event() {
        MovementAction::Toward(translation) => {
            const ARRIVE_SPEED_MOD: f32 = 0.75;
            let target = move_towards(transform.translation, *translation, move_speed);
            let velocity = (target - transform.translation)
                .horizontal()
                .clamp_length_min(move_speed * ARRIVE_SPEED_MOD);
            linear_velocity.x += velocity.x;
            linear_velocity.z += velocity.z;
        }

        MovementAction::Dir(dir) => {
            linear_velocity.x += dir.x * move_speed;
            linear_velocity.z += dir.y * move_speed;
        }
        MovementAction::Jump => {
            if grounded {
                linear_velocity.0.y = 5.0;
            }
        }
    }
}

#[inline]
fn move_towards(current: Vec3, target: Vec3, max_distance_delta: f32) -> Vec3 {
    let to_vector = target - current;
    let sqdist = to_vector.length_squared();
    if sqdist == 0.0 || sqdist <= max_distance_delta * max_distance_delta {
        return target;
    }
    current + to_vector.normalize() * max_distance_delta
}

fn damping(mut query: Query<(&DampingFactor, &mut LinearVelocity)>) {
    for (damping_factor, mut linear_velocity) in &mut query {
        linear_velocity.x *= damping_factor.0;
        linear_velocity.z *= damping_factor.0;
    }
}

// see: https://github.com/Jondolf/avian/blob/main/crates/avian3d/examples/kinematic_character_3d/plugin.rs
#[allow(clippy::type_complexity)]
fn collisions(
    collisions: Res<Collisions>,
    bodies: Query<&RigidBody>,
    collider_parents: Query<&ColliderParent, Without<Sensor>>,
    mut motors: Query<
        (
            &mut Position,
            &Rotation,
            &mut LinearVelocity,
            Option<&MaxSlopeAngle>,
        ),
        (With<RigidBody>, With<CharacterMotor>),
    >,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();

    // Iterate through collisions and move the kinematic body to resolve penetration
    for contacts in collisions.iter() {
        // Get the rigid body entities of the colliders (colliders could be children)
        let Ok([collider_parent1, collider_parent2]) =
            collider_parents.get_many([contacts.entity1, contacts.entity2])
        else {
            continue;
        };

        // Get the body of the character controller and whether it is the first
        // or second entity in the collision.
        let is_first: bool;

        let character_rb: RigidBody;
        let is_other_dynamic: bool;

        let (mut position, rotation, mut linear_velocity, max_slope_angle) =
            if let Ok(character) = motors.get_mut(collider_parent1.get()) {
                is_first = true;
                character_rb = *bodies.get(collider_parent1.get()).unwrap();
                is_other_dynamic = bodies
                    .get(collider_parent2.get())
                    .is_ok_and(|rb| rb.is_dynamic());
                character
            } else if let Ok(character) = motors.get_mut(collider_parent2.get()) {
                is_first = false;
                character_rb = *bodies.get(collider_parent2.get()).unwrap();
                is_other_dynamic = bodies
                    .get(collider_parent1.get())
                    .is_ok_and(|rb| rb.is_dynamic());
                character
            } else {
                continue;
            };

        // This system only handles collision response for kinematic character controllers.
        if !character_rb.is_kinematic() {
            continue;
        }

        // Iterate through contact manifolds and their contacts.
        // Each contact in a single manifold shares the same contact normal.
        for manifold in contacts.manifolds.iter() {
            let normal = if is_first {
                -manifold.global_normal1(rotation)
            } else {
                -manifold.global_normal2(rotation)
            };

            let mut deepest_penetration: f32 = f32::MIN;

            // Solve each penetrating contact in the manifold.
            for contact in manifold.contacts.iter() {
                if contact.penetration > 0.0 {
                    position.0 += normal * contact.penetration;
                }
                deepest_penetration = deepest_penetration.max(contact.penetration);
            }

            // For now, this system only handles velocity corrections for collisions against static
            // geometry.
            if is_other_dynamic {
                continue;
            }

            // Determine if the slope is climbable or if it's too steep to walk on.
            let slope_angle = normal.angle_between(Vec3::Y);
            let climbable = max_slope_angle.is_some_and(|angle| slope_angle.abs() <= angle.0);

            if deepest_penetration > 0.0 {
                // If the slope is climbable, snap the velocity so that the character
                // up and down the surface smoothly.
                if climbable {
                    // Points in the normal's direction in the XZ plane.
                    let normal_direction_xz =
                        normal.reject_from_normalized(Vec3::Y).normalize_or_zero();

                    // The movement speed along the direction above.
                    let linear_velocity_xz = linear_velocity.dot(normal_direction_xz);

                    // Snap the Y speed based on the speed at which the character is moving
                    // up or down the slope, and how steep the slope is.
                    //
                    // A 2D visualization of the slope, the contact normal, and the velocity
                    // components:
                    //
                    //             ╱
                    //     normal ╱
                    // * ╱
                    // │   *    ╱   velocity_x
                    // │       * - - - - - -
                    // │           *       | velocity_y
                    // │               *   |
                    // *───────────────────*

                    let max_y_speed = -linear_velocity_xz * slope_angle.tan();
                    linear_velocity.y = linear_velocity.y.max(max_y_speed);
                } else {
                    // The character is intersecting an unclimbable object, like a wall.
                    // We want the character to slide along the surface, similarly to
                    // a collide-and-slide algorithm.

                    // Don't apply an impulse if the character is moving away from the surface.
                    if linear_velocity.dot(normal) > 0.0 {
                        continue;
                    }

                    // Slide along the surface, rejecting the velocity along the contact normal.
                    let impulse = linear_velocity.reject_from_normalized(normal);
                    linear_velocity.0 = impulse;
                }
            } else {
                // The character is not yet intersecting the other object,
                // but the narrow phase detected a speculative collision.
                //
                // We need to push back the part of the velocity
                // that would cause penetration within the next frame.

                let normal_speed = linear_velocity.dot(normal);

                // Don't apply an impulse if the character is moving away from the surface.
                if normal_speed > 0.0 {
                    continue;
                }

                // Compute the impulse to apply.
                let impulse_magnitude = normal_speed - (deepest_penetration / delta_time);
                let mut impulse = impulse_magnitude * normal;

                // Apply the impulse differently depending on the slope angle.
                if climbable {
                    // Avoid sliding down slopes.
                    linear_velocity.y -= impulse.y.min(0.0);
                } else {
                    // Avoid climbing up walls.
                    impulse.y = impulse.y.max(0.0);
                    linear_velocity.0 -= impulse;
                }
            }
        }
    }
}
