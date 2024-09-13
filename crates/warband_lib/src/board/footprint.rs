use hexx::Hex;

use crate::{navigation::agent::Agent, prelude::*};

use super::{occupied::GetPolygon, Board, BoardSettings, Cell, Location};

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub enum Footprint {
    #[default]
    Empty,
    Cells(SmallVec<[Hex; 8]>),
}

impl Footprint {
    pub fn cells(&self) -> impl Iterator<Item = &Hex> {
        match self {
            Footprint::Empty => [].iter(),
            Footprint::Cells(cells) => cells.iter(),
        }
    }
}

pub(super) fn agents(
    mut agents: Query<
        (&mut Footprint, &Location, &Agent),
        Or<(Changed<Location>, Added<Footprint>, Without<Cell>)>,
    >,
    board: Res<Board>,
) {
    agents
        .par_iter_mut()
        .for_each(|(mut footprint, location, agent)| {
            let Location::Valid(hex) = location else {
                *footprint = Footprint::Empty;
                return;
            };
            *footprint = Footprint::Cells(
                hex.range(agent.size().saturating_sub(1) as u32)
                    .filter(|h| board.bounds.is_in_bounds(*h))
                    .collect(),
            );
        });
}

#[inline]
pub(super) fn obstacles(
    mut obstacles: Query<
        (&mut Footprint, &Location, &GlobalTransform, &Collider),
        (
            Or<(Changed<Location>, Changed<ColliderAabb>, Added<Footprint>)>,
            Without<Agent>,
            Without<Cell>,
        ),
    >,
    board: Res<Board>,
    board_settings: Res<BoardSettings>,
) {
    let hexes: Vec<Hex> = board.entities.keys().copied().collect();

    obstacles
        .par_iter_mut()
        .for_each(|(mut footprint, location, global_transform, collider)| {
            let Location::Valid(_) = location else {
                *footprint = Footprint::Empty;
                return;
            };

            let polygon = collider.get_polygon(
                global_transform,
                &board.transform,
                (Dir3::Y, board_settings.upward_shift),
            );
            *footprint = Footprint::Cells(
                hexes
                    .iter()
                    .copied()
                    .filter(|h| point_in_poly2d(board.layout.hex_to_world_pos(*h), &polygon))
                    .collect(),
            );
        });
}

/// ref: https://github.com/Jondolf/barry/blob/main/src/utils/point_in_poly2d.rs
#[inline]
fn point_in_poly2d(pt: Vec2, poly: &[Vec2]) -> bool {
    if poly.is_empty() {
        false
    } else {
        let mut sign = 0.0;

        for i1 in 0..poly.len() {
            let i2 = (i1 + 1) % poly.len();
            let seg_dir = poly[i2] - poly[i1];
            let dpt = pt - poly[i1];
            let perp = dpt.perp_dot(seg_dir);

            if sign.is_zero() {
                sign = perp;
            } else if sign * perp < 0.0 {
                return false;
            }
        }

        true
    }
}

#[cfg(feature = "dev")]
pub(crate) fn gizmos(mut gizmos: Gizmos, footprints: Query<&Footprint>, board: Res<Board>) {
    use bevy::color::palettes::tailwind::GREEN_300;
    for footprint in &footprints {
        for hex in footprint.cells().copied() {
            gizmos.hex_scaled_3d(hex, 0.8, &board.layout, GREEN_300);
        }
    }
}
