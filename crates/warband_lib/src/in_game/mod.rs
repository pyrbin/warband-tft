use hexx::{shapes, Hex, HexLayout, PlaneMeshBuilder};

use crate::{prelude::*, AppState};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::InGame), (setup, grid));
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::srgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn((
        Name::new("sun"),
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 5000.0,
                color: Color::WHITE,
                ..default()
            },
            transform: Transform::from_xyz(30., 100., 30.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

const HEX_SIZE: Vec2 = Vec2::splat(1.0);

#[derive(Debug, Resource)]
struct Map {
    layout: HexLayout,
    entities: HashMap<Hex, Entity>,
    highlighted_material: Handle<StandardMaterial>,
    default_material: Handle<StandardMaterial>,
}

fn grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let layout = HexLayout {
        hex_size: HEX_SIZE,
        orientation: hexx::HexOrientation::Pointy,
        ..default()
    };
    // materials
    let default_material = materials.add(Color::Srgba(WHITE));
    let highlighted_material = materials.add(Color::Srgba(YELLOW));
    // mesh
    let mesh = hexagonal_plane(&layout);
    let mesh_handle = meshes.add(mesh);

    let entities = shapes::flat_rectangle([-3, 3, -4, 4])
        .map(|hex| {
            let pos = layout.hex_to_world_pos(hex);
            let id = commands
                .spawn((
                    Name::new(format!("{hex:?}")),
                    PbrBundle {
                        transform: Transform::from_xyz(pos.x, 0.0, pos.y),
                        mesh: mesh_handle.clone(),
                        material: default_material.clone_weak(),
                        ..default()
                    },
                ))
                .id();
            (hex, id)
        })
        .collect();

    commands.insert_resource(Map {
        layout,
        entities,
        highlighted_material,
        default_material,
    });
}

fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = PlaneMeshBuilder::new(hex_layout)
        .facing(hexx::Vec3::Y)
        .with_scale(hexx::Vec3::splat(0.98))
        .center_aligned()
        .build();
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
}
