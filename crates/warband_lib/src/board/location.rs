use hexx::Hex;

use crate::prelude::*;

use super::Board;

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
        Or<(Changed<GlobalTransform>, Added<Location>)>,
    >,
    board: Res<Board>,
) {
    transforms
        .par_iter_mut()
        .for_each(|(mut location, transform)| {
            let hex: Hex = board.layout.world_pos_to_hex(transform.translation().xz());
            let value = if board.bounds.is_in_bounds(hex) {
                Location::Valid(hex)
            } else {
                Location::Invalid
            };

            if *location != value {
                *location = value;
            }
        });
}

#[cfg(feature = "dev")]
pub(crate) fn gizmos(mut gizmos: Gizmos, entities: Query<&Location>, board: Res<Board>) {
    for location in &entities {
        let Location::Valid(hex) = location else {
            continue;
        };

        let position = board.layout.hex_to_world_pos(*hex);

        gizmos.rect(
            position.x0y(),
            Quat::from_rotation_x(PI / 2.),
            board.layout.hex_size,
            YELLOW.with_alpha(1.0),
        );
    }
}
