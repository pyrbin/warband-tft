use bevy_vector_shapes::prelude::*;
use stats::Health;

use crate::{player::camera::MainCamera, prelude::*};

pub mod stats;

pub(super) fn plugin(app: &mut App) {
    app_register_types!(Unit);

    app.add_plugins(stats::plugin);

    app.add_systems(Update, ui_hp);
}

#[derive(Component, Default, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Unit;

fn ui_hp(
    units: Query<(Pool<Health>, &GlobalTransform), With<Unit>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut painter: ShapePainter,
) {
    use bevy::color::palettes::tailwind::GRAY_200;

    let (camera, camera_transform) = or_return!(camera.iter().next());
    let left = camera_transform.left();

    for (hp, transform) in &units {
        let position = transform.translation();
        painter.alignment = Alignment::Billboard;
        // TODO: fix styling & y-position depending on progress state
        painter.transform.translation = position + left * 0.75;
        painter.set_color(GRAY_200);
        painter.corner_radii = Vec4::splat(0.01);
        painter.rect(Vec2::new(0.4, 1.0 * hp.progress01()));
    }
}
