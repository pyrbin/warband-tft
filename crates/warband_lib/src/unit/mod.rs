use bevy::color::palettes::tailwind::{BLUE_600, GRAY_800, RED_600};
use bevy_spatial::{kdtree::KDTree3, AutomaticUpdate, TransformMode};
use bevy_vector_shapes::prelude::*;
use stats::Health;

use crate::{ability::slot::AbilitySlots, player::camera::MainCamera, prelude::*};

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

    app.add_systems(Update, (ui_hp, ui_mana));
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
    pub const fn enemy(&self) -> Allegiance {
        self.complement()
    }

    pub const fn ally(&self) -> Allegiance {
        *self
    }

    pub const fn is_enemy(&self, other: Self) -> bool {
        !self.is_ally(other)
    }

    pub const fn is_ally(&self, other: Self) -> bool {
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
        const BAR_WIDTH: f32 = 0.3;
        const BG_PAD: f32 = 0.1;

        let position = transform.translation();
        painter.alignment = Alignment::Billboard;
        painter.corner_radii = Vec4::splat(0.0);
        let (height, offset) = progress_bar(BAR_HEIGHT, hp.progress01());

        painter.transform.translation = position + left * LEFT_OFFSET + forward * BG_PAD;
        painter.set_color(GRAY_800);
        painter.rect(Vec2::new(BAR_WIDTH, BAR_HEIGHT));

        painter.transform.translation = position + left * LEFT_OFFSET - offset * up;
        painter.set_color(RED_600);
        painter.rect(Vec2::new(BAR_WIDTH, height));
    }
}

fn ui_mana(
    units: Query<
        (&GlobalTransform, &AbilitySlots, &crate::ability::Mana),
        (With<Unit>, Without<crate::ability::slot::AbilitySlot>),
    >,
    ability_slots: Query<Pool<crate::ability::Mana>, With<crate::ability::slot::AbilitySlot>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut painter: ShapePainter,
) {
    let (_, camera_transform) = or_return!(camera.iter().next());
    let left = camera_transform.left();
    let up = camera_transform.up();
    let forward: Dir3 = camera_transform.forward();

    for (transform, slots, mana) in &units {
        const BAR_HEIGHT: f32 = 0.75;
        const BAR_WIDTH: f32 = 0.15;
        const BG_PAD: f32 = 0.1;
        const LEFT_OFFSET: f32 = 0.75 + BAR_WIDTH + BAR_WIDTH;
        const SLOT_PAD: f32 = 0.075;

        let total_padding = SLOT_PAD * (slots.len() - 1) as f32;
        let total_height = BAR_HEIGHT - total_padding;
        let mut offset_y = if slots.len() > 1 {
            -SLOT_PAD / slots.len() as f32
        } else {
            0.0
        };

        painter.alignment = Alignment::Billboard;
        painter.corner_radii = Vec4::splat(0.0);

        for ability_slot in slots.iter() {
            let slot_mana = or_continue!(ability_slots.get(*ability_slot));
            let percentage = slot_mana.total() / mana.value();

            let total_height = total_height * percentage;
            let self_offset_y = if slots.len() <= 1 {
                Vec3::ZERO
            } else {
                (total_height / 2.0 + if slots.len() > 2 { total_padding } else { 0.0 }) * up
            };

            let slot_offset_y = offset_y * up;
            let total_offet_y = self_offset_y - slot_offset_y;

            let position = transform.translation();
            let (height, offset) = progress_bar(total_height, slot_mana.progress01());

            painter.transform.translation =
                position + left * LEFT_OFFSET + forward * BG_PAD + total_offet_y;
            painter.set_color(GRAY_800);
            painter.rect(Vec2::new(BAR_WIDTH, total_height));

            painter.transform.translation =
                position + left * LEFT_OFFSET - offset * up + total_offet_y;
            painter.set_color(BLUE_600);
            painter.rect(Vec2::new(BAR_WIDTH, height));

            offset_y += total_height + SLOT_PAD;
        }
    }
}

fn progress_bar(value: f32, progress: f32) -> (f32, f32) {
    let height = value * progress;
    (height, (value - height) / 2.0)
}
