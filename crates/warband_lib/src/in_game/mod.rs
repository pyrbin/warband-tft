use bevy_mod_picking::prelude::*;

use crate::{
    ability::{
        self,
        cast::{AbilityCaster, TryAbility},
        example::Fireball,
        slot::AbilitySlots,
        AbilityData,
        Mana,
        Radius,
    },
    board,
    navigation::{agent, path},
    physics::motor::{self, Movement},
    player::camera::MainCamera,
    prelude::*,
    stats::modifier::Mult,
    unit,
    AppState,
};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::InGame), setup);
    app.add_systems(
        Update,
        (test_target, test_cast).run_if(in_state(AppState::InGame)),
    );
}

#[derive(Component)]
struct MoveTo;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // light
    commands.spawn((
        Name::light("sun"),
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 5000.0,
                color: Color::WHITE,
                ..default()
            },
            transform: Transform::from_xyz(30., 100., 30.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Cleanup(OnExit(AppState::InGame)),
    ));

    let _ = commands
        .spawn((
            Name::unit("floor"),
            SpatialBundle {
                transform: Transform::from_translation(Vec3::Y * -0.05),
                ..default()
            },
            RigidBody::Static,
            Collider::cuboid(100.0, 0.1, 100.0),
        ))
        .id();

    let caster = commands
        .spawn((
            Name::unit("caster"),
            SpatialBundle {
                transform: Transform::from_translation(Vec3::Y),
                ..default()
            },
            Mana(0.0),
            Radius(0.0),
            Mult(Radius(-1.5)),
            Mult::<Mana>(Mana(0.0)),
            AbilityCaster,
            AbilitySlots::with(Fireball::ID).with(Fireball::ID),
        ))
        .id();

    let _ = commands
        .spawn((
            Name::unit("Target"),
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.5, 0.5, 0.5)),
                material: materials.add(Color::srgb_u8(124, 144, 255).with_alpha(0.1)),
                transform: Transform::from_xyz(5.0, 0.5, 5.0),
                ..default()
            },
            board::Location::default(),
            board::Footprint::default(),
            unit::Unit,
            unit::Allegiance::TEAM_2,
            unit::stats::Health::pool(999.0),
            MoveTo,
            (
                agent::Agent::default(),
                agent::DestinationRange(1),
                motor::CharacterMotorBundle::new(0.5, 0.5),
                path::Destination::None,
            ),
        ))
        .id();

    // unit

    for i in 0..5 {
        let random_color = 255.0 * Vec3::new(rand::random(), rand::random(), rand::random());
        commands.spawn((
            Name::unit("Unit"),
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.5, 0.5, 0.5)),
                material: materials.add(Color::srgb_u8(
                    random_color.x as u8,
                    random_color.y as u8,
                    random_color.z as u8,
                )),
                transform: Transform::from_xyz(i as f32 - 7.0, 0.5, i as f32 - 7.0),
                ..default()
            },
            board::Location::default(),
            board::Footprint::default(),
            (
                agent::Agent::default(),
                agent::DestinationRange(1),
                motor::CharacterMotorBundle::new(0.5, 0.5),
                path::Destination::None,
            ),
            (
                unit::ai::UNIT_THINKER.clone(),
                unit::Allegiance::TEAM_1,
                unit::ai::Target::None,
            ),
            PickableBundle::default(),
            On::<Pointer<Click>>::target_insert(Despawn::Immediate),
            Movement(150.0 + (rand::random::<f32>() * 400.0)),
            unit::Unit,
            unit::stats::Health::pool(100.0),
        ));
    }
}

fn test_cast(
    buttons: Res<ButtonInput<KeyCode>>,
    query: Query<(Entity, &mut ability::Caster, &ability::AbilityId)>,
    mut events: EventWriter<TryAbility>,
    target: Query<Entity, With<MoveTo>>,
    mut caster: Query<&mut Transform>,
) {
    if buttons.just_pressed(KeyCode::Space) {
        let ability = single!(query);
        let target = single!(target);
        let mut transform = or_return!(caster.get_mut(**ability.1));
        // random Vec3 with y being between 2 and 10.
        let position = Vec3::ZERO + Vec3::Y * (rand::random::<f32>() * 10.0);
        transform.translation = position;
        events.send(TryAbility {
            caster: **ability.1,
            ability: ability.0,
            target: Target::Entity(target),
        });
    }
}

fn test_target(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    board: Res<board::Board>,
    mut target: Query<&mut Transform, With<MoveTo>>,
) {
    let window: &Window = single!(windows);
    let (camera, cam_transform) = single!(cameras);
    let mut target = single_mut!(target);

    let Some(ray) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world(cam_transform, p))
    else {
        return;
    };

    if !buttons.just_pressed(MouseButton::Middle) {
        return;
    }

    let Some(distance) = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Dir3::Y)) else {
        return;
    };

    let point = ray.origin + ray.direction * distance;
    let hex = board.layout.world_pos_to_hex(point.xz());
    target.translation = board.layout.hex_to_world_pos(hex).x0y();
}
