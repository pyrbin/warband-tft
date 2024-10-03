use bevy::color::palettes::tailwind::{GRAY_600, RED_600};
use bevy_spatial::{kdtree::KDTree3, AutomaticUpdate, TransformMode};
use bevy_vector_shapes::prelude::*;
use stats::Health;

use crate::{player::camera::MainCamera, prelude::*};

pub mod ai;
pub mod combat;
pub mod stats;

pub(super) fn plugin(app: &mut App) {
    app_register_types!(Unit);
    app.add_plugins(
        AutomaticUpdate::<Unit>::new()
            .with_frequency(Duration::from_secs_f32(0.3))
            .with_transform(TransformMode::GlobalTransform),
    );
    app.add_plugins((stats::plugin, ai::plugin, combat::plugin));

    app.add_systems(Update, ui_hp);
}

#[derive(Component, Default, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Unit;

pub type UnitTree = KDTree3<Unit>;

bitflags::bitflags! {
    #[derive(Default, Component, Reflect)]
    #[reflect(Component, PartialEq)]
    pub struct Allegiance: u32 {
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

impl Allegiance {
    pub fn is_enemy(&self, other: Self) -> bool {
        !self.is_ally(other)
    }

    pub fn is_ally(&self, other: Self) -> bool {
        self.intersects(other)
    }
}

fn ui_hp(
    units: Query<(Pool<Health>, &GlobalTransform), With<Unit>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut painter: ShapePainter,
) {
    let (_, camera_transform) = or_return!(camera.iter().next());
    let left = camera_transform.left();
    let up = camera_transform.up();
    let forward: Dir3 = camera_transform.forward();

    for (hp, transform) in &units {
        const LEFT_OFFSET: f32 = 0.75;
        const BAR_HEIGHT: f32 = 0.75;
        const BG_PAD: f32 = 0.1;

        let position = transform.translation();
        painter.alignment = Alignment::Billboard;
        painter.corner_radii = Vec4::splat(0.0);
        let (height, offset) = progress_bar(BAR_HEIGHT, hp.progress01());

        painter.transform.translation = position + left * LEFT_OFFSET + forward * BG_PAD;
        painter.set_color(GRAY_600);
        painter.rect(Vec2::new(0.3, BAR_HEIGHT));

        painter.transform.translation = position + left * LEFT_OFFSET - offset * up;
        painter.set_color(RED_600);
        painter.rect(Vec2::new(0.3, height));
    }
}

fn progress_bar(value: f32, progress: f32) -> (f32, f32) {
    let height = value * progress;
    (height, (value - height) / 2.0)
}
