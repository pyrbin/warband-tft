use bevy_inspector_egui::prelude::*;
use bevy_mod_picking::prelude::*;
use hexx::*;

use crate::{prelude::*, AppState};

pub mod footprint;
pub mod location;
pub mod occupied;

pub use footprint::Footprint;
pub use location::Location;
use occupied::DirtyOccupied;
pub use occupied::Occupied;

pub const HEX_SIZE_RATIO: f32 = 1.44;

#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BoardSystems {
    Build,
    Location,
    Footprint,
    Occupied,
}

pub(super) fn plugin(app: &mut App) {
    app_register_types!(
        Board,
        BoardSettings,
        BoardBounds,
        Cell,
        Location,
        BoardBuiltEvent,
        footprint::Footprint
    );

    app.init_resource::<Board>()
        .init_resource::<Occupied>()
        .init_resource::<DirtyOccupied>()
        .insert_resource(
            BoardSettings::builder()
                .width(10)
                .height(10)
                .hex_size(1.0)
                .build(),
        )
        .add_event::<BoardBuiltEvent>();

    app.observe(location::added);

    app.configure_sets(
        FixedUpdate,
        (
            BoardSystems::Build.run_if(resource_exists::<BoardSettings>),
            BoardSystems::Location,
            BoardSystems::Footprint,
            BoardSystems::Occupied,
        )
            .chain()
            .run_if(resource_exists::<Board>)
            .run_if(in_state(AppState::InGame)),
    );

    app.add_systems(First, occupied::detect_dirty);
    app.add_systems(
        FixedUpdate,
        (
            build.in_set(BoardSystems::Build),
            (
                location::location,
                location::on_board_built.run_if(on_event::<BoardBuiltEvent>()),
            )
                .in_set(BoardSystems::Location),
            footprint::agents.in_set(BoardSystems::Footprint),
            footprint::obstacles.in_set(BoardSystems::Footprint),
            occupied::splat.chain().in_set(BoardSystems::Occupied),
        ),
    );
}

#[derive(Builder, Debug, Resource, Default, Reflect, InspectorOptions)]
#[reflect(Resource, Default)]
pub struct BoardSettings {
    // Size of a single hexagon in world units.
    #[inspector(min = 1.0, max = 10.0)]
    pub hex_size: f32,
    // Width of the board
    #[inspector(min = 1, max = 100)]
    pub width: i32,
    // Height of the board
    #[inspector(min = 1, max = 100)]
    pub height: i32,
    // Orientation of the board.
    #[builder(default)]
    pub orientation: HexOrientation,
    /// Obstacle scale
    #[builder(default)]
    #[inspector(min = 0.0, max = 10.0)]
    pub obstacle_scale: f32,
    /// Upward shift to sample obstacle from the ground
    #[builder(default)]
    #[inspector(min = -5.0, max = 10.0)]
    pub upward_shift: f32,
}

#[derive(Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct Board {
    pub entities: HashMap<Hex, Entity>,
    pub layout: HexLayout,
    pub entity: Entity,
    pub bounds: BoardBounds,
    pub transform: Transform,
}

impl Board {
    pub fn cells(&self) -> impl Iterator<Item = (Hex, Entity)> + '_ {
        self.entities.iter().map(|(hex, entity)| (*hex, *entity))
    }
}

impl FromWorld for Board {
    fn from_world(world: &mut World) -> Self {
        let entity = world
            .spawn((
                Name::unit("board"),
                SpatialBundle { ..default() },
                BoardHolder,
            ))
            .id();

        Self {
            entities: HashMap::new(),
            layout: HexLayout::default(),
            entity,
            bounds: BoardBounds::default(),
            transform: Transform::default(),
        }
    }
}

#[derive(Debug, Default, Clone, Reflect)]
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

    pub(crate) fn is_in_bounds(&self, hex: Hex) -> bool {
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

#[derive(Component, Debug, Reflect)]
pub struct BoardHolder;

#[derive(Component, Debug, Reflect)]
pub struct Cell;

#[derive(Component, Clone, Copy, Default, Debug, Reflect)]
#[reflect(Component)]
pub struct Hovered;

#[derive(Event, Debug, Default, Reflect)]
pub struct BoardBuiltEvent;

fn build(
    mut commands: Commands,
    board_settings: Res<BoardSettings>,
    mut board: ResMut<Board>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut event_writer: EventWriter<BoardBuiltEvent>,
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

    let default_material = materials.add(Color::WHITE);
    let mesh = circle_column(&layout);
    let mesh_handle = meshes.add(mesh);

    let half_width = board_settings.width / 2;
    let half_height = board_settings.height / 2;
    let corners = [-half_width, half_width, -half_height, half_height];

    let entities: HashMap<_, _> = shapes::flat_rectangle(corners)
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
    board.bounds = BoardBounds::new(
        board_settings.width,
        board_settings.height,
        board_settings.orientation,
    );
    board.transform.translation = Vec3::new(board.layout.origin.x, 0.0, board.layout.origin.y);
    board.transform.rotation = Quat::IDENTITY;

    event_writer.send(BoardBuiltEvent);
}

fn circle_column(hex_layout: &HexLayout) -> Mesh {
    Cylinder {
        radius: hex_layout.hex_size.x * 0.75,
        half_height: f32::EPSILON,
    }
    .into()
}

#[cfg(feature = "dev")]
pub(crate) fn gizmos(mut gizmos: Gizmos, board: Res<Board>, occupied: Res<Occupied>) {
    use bevy::color::palettes::tailwind::{RED_500, YELLOW_500};

    let mut occupied_edges = Vec::new();

    for hex in occupied.cells() {
        for [a, b] in board.layout.all_edge_coordinates(hex) {
            gizmos.line(a.x0y(), b.x0y(), RED_500);
            occupied_edges.push((a, b));
        }
    }

    for (hex, _) in board.cells() {
        for [a, b] in board.layout.all_edge_coordinates(hex) {
            if occupied_edges
                .iter()
                .copied()
                .any(|(oa, ob)| oa == a && ob == b)
            {
                continue;
            }

            for [a, b] in board.layout.all_edge_coordinates(hex) {
                gizmos.line(a.x0y(), b.x0y(), YELLOW_500);
            }
        }
    }
}
