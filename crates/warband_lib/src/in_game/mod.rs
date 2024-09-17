use crate::{
    board,
    navigation::{self, agent::Agent},
    player::camera::MainCamera,
    prelude::*,
    stats::stat,
    AppState,
};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::InGame), setup);
    app.add_systems(Update, test_target.run_if(in_state(AppState::InGame)));

    app.add_plugins(stat::plugin::<Health>);
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

    let cube_mesh = meshes.add(Cuboid::default());

    commands.spawn((
        Name::unit("floor"),
        SpatialBundle {
            transform: Transform::from_translation(Vec3::Y * -0.05),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(100.0, 0.1, 100.0),
    ));

    let id = commands
        .spawn((
            Name::unit("Target"),
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.5, 0.5, 0.5)),
                material: materials.add(Color::srgb_u8(124, 144, 255).with_alpha(0.1)),
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                ..default()
            },
            Agent::default(),
            board::Location::default(),
            board::Footprint::default(),
            MoveTo,
        ))
        .id();

    // unit
    let test_id = commands
        .spawn((
            Name::unit("Test"),
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.5, 0.5, 0.5)),
                material: materials.add(Color::srgb_u8(2, 144, 255)),
                transform: Transform::from_xyz(4.0, 0.5, 4.0),
                ..default()
            },
            Agent::default(),
            board::Location::default(),
            board::Footprint::default(),
            navigation::path::Target::Entity(id),
        ))
        .id();

    commands.spawn((
        Name::unit("obstacle"),
        SpatialBundle {
            transform: Transform::from_xyz(-2.0, 2.0, -2.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(4.0, 4.0, 4.0),
        board::Location::default(),
        board::Footprint::default(),
        Health::new(200.0),
    ));
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

    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(distance) = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Dir3::Y)) else {
        return;
    };

    let point = ray.origin + ray.direction * distance;
    let hex = board.layout.world_pos_to_hex(point.xz());
    target.translation = board.layout.hex_to_world_pos(hex).x0y();
}

#[derive(Stat, Component, Reflect, Copy, Clone)]
struct Health(f32);

// impl Stat for Health {
//     fn new(value: f32) -> Self {
//         Self(value)
//     }

//     fn value(&self) -> f32 {
//         self.0
//     }
// }
