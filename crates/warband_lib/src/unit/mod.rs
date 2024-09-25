use bevy_spatial::kdtree::KDTree3;
use bevy_vector_shapes::prelude::*;
use stats::Health;

use crate::{player::camera::MainCamera, prelude::*};

pub mod ai;
pub mod stats;

pub(super) fn plugin(app: &mut App) {
    app_register_types!(Unit);

    app.add_plugins(stats::plugin);
    app.add_plugins(ai::plugin);

    app.add_systems(Update, ui_hp);
}

#[derive(Component, Default, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Unit;

pub type UnitTree = KDTree3<Unit>;

#[derive(Component, Reflect, Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[reflect(Component, Hash, PartialEq)]
pub struct Allegiance(u32);

impl Allegiance {
    pub fn is_enemy(&self, other: Self) -> bool {
        self.0 != other.0
    }

    pub fn is_ally(&self, other: Self) -> bool {
        0 != self.0 & other.0
    }
}

bitflags::bitflags! {
    impl Allegiance: u32 {
        /// The team n°1.
        const TEAM_1 = 1 << 0;
        /// The team n°2.
        const TEAM_2 = 1 << 1;
        /// All of the teams.
        const ALL = u32::MAX;
        /// None of the teams.
        const NONE = 0;
    }
}

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
