use crate::{prelude::*, AppState};
use bevy_inspector_egui::prelude::*;
use bevy_mod_picking::prelude::*;
use hexx::*;

pub mod location;
pub use location::Location;

pub(super) fn plugin(app: &mut App) {
    app_register_types!(Board, BoardSettings, Cell, Location);

    app.init_resource::<Board>().insert_resource(
        BoardSettings::builder()
            .width(10)
            .height(10)
            .hex_size(1.0)
            .build(),
    );

    app.add_systems(
        Update,
        ((
            regenerate,
            location::location.run_if(resource_exists::<Board>),
        )
            .run_if(resource_exists::<BoardSettings>)
            .run_if(in_state(AppState::InGame)),),
    );
}

#[derive(Debug, Resource, Reflect)]
pub struct Board {
    pub entities: HashMap<Hex, Entity>,
    pub layout: HexLayout,
    pub entity: Entity,
    pub bounds: BoardBounds,
    pub highlighted_material: Handle<StandardMaterial>,
    pub default_material: Handle<StandardMaterial>,
}

impl Board {
    pub fn cell(&self, hex: Hex) -> Option<Entity> {
        self.entities.get(&hex).copied()
    }
}

impl FromWorld for Board {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();

        let default_material = materials.add(Color::Srgba(RED));
        let highlighted_material = materials.add(Color::Srgba(YELLOW));

        let entity = world
            .spawn((SpatialBundle { ..default() }, Name::new("board")))
            .id();

        Self {
            entities: HashMap::new(),
            layout: HexLayout::default(),
            entity,
            bounds: BoardBounds::default(),
            highlighted_material,
            default_material,
        }
    }
}

#[derive(Debug, Default, Reflect)]
pub struct BoardBounds {
    half_width: i32,
    half_height: i32,
    orientation: HexOrientation,
}

impl BoardBounds {
    fn new(width: i32, height: i32, orientation: HexOrientation) -> Self {
        let half_width = width / 2;
        let half_height = height / 2;
        Self {
            half_width,
            half_height,
            orientation,
        }
    }

    fn from_board_settings(settings: &BoardSettings) -> Self {
        Self::new(settings.width, settings.height, settings.orientation)
    }

    fn is_in_bounds(&self, hex: Hex) -> bool {
        let (x, y) = axial_to_xy(hex, self.orientation);
        x >= -self.half_width
            && x <= self.half_width
            && y >= -self.half_height
            && y <= self.half_height
    }
}

#[inline]
fn axial_to_xy(hex: Hex, orientation: HexOrientation) -> (i32, i32) {
    let q = hex.x();
    let r = hex.y();
    match orientation {
        HexOrientation::Flat => (q, r + (q - (q & 1)) / 2),
        HexOrientation::Pointy => (q + (r - (r & 1)) / 2, r),
    }
}

#[derive(Builder, Debug, Resource, Default, Reflect, InspectorOptions)]
pub struct BoardSettings {
    #[inspector(min = 1.0, max = 10.0)]
    pub hex_size: f32,
    #[inspector(min = 1, max = 100)]
    pub width: i32,
    #[inspector(min = 1, max = 100)]
    pub height: i32,
    #[builder(default)]
    pub orientation: HexOrientation,
}

impl BoardSettings {
    #[inline]
    pub fn corners(&self) -> [i32; 4] {
        let half_width = self.width / 2;
        let half_height = self.height / 2;
        [-half_width, half_width, -half_height, half_height]
    }
}

#[derive(Debug, Component, Reflect)]
pub struct Cell;

#[derive(Debug, Clone, Copy, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Hovered;

fn regenerate(
    mut commands: Commands,
    board_settings: Res<BoardSettings>,
    mut board: ResMut<Board>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if !board_settings.is_changed() {
        return;
    }

    board.entities.iter_mut().for_each(|(_, entity)| {
        commands.entity(*entity).despawn_recursive();
    });

    let layout = HexLayout {
        hex_size: Vec2::splat(board_settings.hex_size),
        orientation: board_settings.orientation,
        ..default()
    };

    let mesh = circle_column(&layout);
    let mesh_handle = meshes.add(mesh);

    let entities: HashMap<_, _> = shapes::flat_rectangle(board_settings.corners())
        .map(|hex| {
            let pos = layout.hex_to_world_pos(hex);
            let id = commands
                .spawn((
                    Name::new(format!("{hex:?}")),
                    PbrBundle {
                        transform: Transform::from_xyz(pos.x, 0.0, pos.y),
                        mesh: mesh_handle.clone(),
                        material: board.default_material.clone_weak(),
                        ..default()
                    },
                    Cell,
                    Location::default(),
                    PickableBundle::default(),
                    On::<Pointer<Over>>::target_insert(Hovered),
                    On::<Pointer<Out>>::target_remove::<Hovered>(),
                ))
                .id();
            (hex, id)
        })
        .collect();

    commands
        .entity(board.entity)
        .push_children(&entities.values().copied().collect::<Vec<_>>());

    board.entities = entities;
    board.layout = layout;
    board.bounds = BoardBounds::from_board_settings(&board_settings);
}

fn circle_column(hex_layout: &HexLayout) -> Mesh {
    Cylinder {
        radius: hex_layout.hex_size.x * 0.75,
        half_height: f32::EPSILON,
    }
    .into()
}
