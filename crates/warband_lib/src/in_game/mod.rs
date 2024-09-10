use crate::{prelude::*, AppState};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::InGame), setup);
}

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

    // unit
    commands.spawn((
        Name::unit("Test"),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::srgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        crate::board::Location::default(),
    ));
}
