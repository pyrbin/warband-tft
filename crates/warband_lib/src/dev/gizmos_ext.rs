use hexx::{Hex, HexLayout};

use crate::prelude::*;

pub trait GizmosExt {
    fn plane_3d(&mut self, origin: Vec3, normal: Dir3, color: impl Into<Color>);

    fn hex_3d(&mut self, hex: Hex, layout: &HexLayout, color: impl Into<Color>);

    fn hex_scaled_3d(&mut self, hex: Hex, scale: f32, layout: &HexLayout, color: impl Into<Color>);
}

impl<Config, Clear> GizmosExt for Gizmos<'_, '_, Config, Clear>
where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,
{
    fn plane_3d(&mut self, origin: Vec3, normal: Dir3, color: impl Into<Color>) {
        let color = color.into();
        self.arrow(origin, origin + normal.as_vec3(), color);
        let rotation = Quat::from_rotation_arc(Vec3::Z, normal.as_vec3());
        self.grid(origin, rotation, UVec2::splat(100), Vec2::splat(1.0), color);
    }

    fn hex_3d(&mut self, hex: Hex, layout: &HexLayout, color: impl Into<Color>) {
        let color = color.into();
        for [a, b] in layout.all_edge_coordinates(hex) {
            self.line(a.x0y(), b.x0y(), color);
        }
    }

    fn hex_scaled_3d(&mut self, hex: Hex, scale: f32, layout: &HexLayout, color: impl Into<Color>) {
        let color = color.into();
        let center = layout.hex_to_world_pos(hex);
        let scale = 1.0 - scale;
        for [a, b] in layout.all_edge_coordinates(hex) {
            let a_direction_to_center = (center - a).normalize();
            let b_direction_to_center = (center - b).normalize();

            let a = a + a_direction_to_center * (layout.hex_size * scale);
            let b = b + b_direction_to_center * (layout.hex_size * scale);
            self.line(a.x0y(), b.x0y(), color);
        }
    }
}
