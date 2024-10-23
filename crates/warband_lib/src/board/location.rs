use hexx::Hex;

use super::Board;
use crate::{navigation::agent::Waypoint, prelude::*};

#[derive(Component, Default, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub enum Location {
    #[default]
    Invalid,
    Valid(Hex),
}

pub(super) fn location(
    mut transforms: Query<
        (&mut Location, &GlobalTransform),
        (
            Or<(Changed<GlobalTransform>, Added<Location>)>,
            Without<Waypoint>, // if entity has a Waypoint, let pathing decide it's Location
        ),
    >,
    board: Res<Board>,
) {
    transforms
        .par_iter_mut()
        .for_each(|(mut location, transform)| {
            let value = to_location(&board, transform);
            if *location != value {
                *location = value;
            }
        });
}

pub(super) fn added(
    trigger: Trigger<OnAdd, Location>,
    mut transforms: Query<&mut Transform>,
    board: Res<Board>,
) {
    let mut transform = or_return!(transforms.get_mut(trigger.entity()));
    let hex: Hex = board.layout.world_pos_to_hex(transform.translation.xz());
    let world_position = board.layout.hex_to_world_pos(hex);
    transform.translation = world_position.x_y(transform.translation.y);
}

pub(super) fn on_board_built(
    mut transforms: Query<(&mut Location, &GlobalTransform)>,
    board: Res<Board>,
) {
    transforms
        .par_iter_mut()
        .for_each(|(mut location, transform)| {
            let value = to_location(&board, transform);
            if *location != value {
                *location = value;
            }
        });
}

#[inline]
fn to_location(board: &Board, transform: &GlobalTransform) -> Location {
    let hex: Hex = board.layout.world_pos_to_hex(transform.translation().xz());
    if board.bounds.is_in_bounds(hex) {
        Location::Valid(hex)
    } else {
        Location::Invalid
    }
}

#[cfg(feature = "dev")]
pub(crate) fn gizmos(
    mut gizmos: Gizmos,
    entities: Query<&Location, Without<super::Cell>>,
    board: Res<Board>,
) {
    use bevy::color::palettes::tailwind::CYAN_500;

    for location in &entities {
        let Location::Valid(hex) = location else {
            continue;
        };

        let position = board.layout.hex_to_world_pos(*hex);

        gizmos.circle(
            position.x0y(),
            Dir3::Y,
            board.layout.hex_size.x.min(board.layout.hex_size.y) / 2.0,
            CYAN_500,
        );
    }
}
