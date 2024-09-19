use avian3d::parry::na::SimdPartialOrd;

use crate::prelude::*;

pub(crate) trait Vec3Ext: Copy {
    fn is_approx_zero(self) -> bool;
    fn horizontal(self) -> Vec3;
}

impl Vec3Ext for Vec3 {
    #[inline]
    fn is_approx_zero(self) -> bool {
        self.length_squared() < 1e-5
    }

    #[inline]
    fn horizontal(self) -> Vec3 {
        Vec3::new(self.x, 0., self.z)
    }
}

pub(crate) trait Vec2Ext: Copy {
    fn is_approx_zero(self) -> bool;
    fn x0y(self) -> Vec3;
    fn x_y(self, y: f32) -> Vec3;
}

impl Vec2Ext for Vec2 {
    #[inline]
    fn is_approx_zero(self) -> bool {
        self.length_squared() < 1e-5
    }

    #[inline]
    fn x0y(self) -> Vec3 {
        Vec3::new(self.x, 0., self.y)
    }

    #[inline]
    fn x_y(self, y: f32) -> Vec3 {
        Vec3::new(self.x, y, self.y)
    }
}

pub(crate) trait F32Ext: Copy {
    fn is_approx_zero(self) -> bool;
    fn squared(self) -> f32;
}

impl F32Ext for f32 {
    #[inline]
    fn is_approx_zero(self) -> bool {
        f32::EPSILON > self.abs()
    }
    #[inline]
    fn squared(self) -> f32 {
        self * self
    }
}

pub(crate) trait Clamp01 {
    fn clamp01(self) -> Self;
}

impl Clamp01 for f32 {
    #[inline]
    fn clamp01(self) -> Self {
        self.simd_clamp(0.0, 1.0)
    }
}

impl Clamp01 for Vec2 {
    #[inline]
    fn clamp01(self) -> Self {
        self.clamp(Vec2::ZERO, Vec2::ONE)
    }
}

impl Clamp01 for Vec3 {
    #[inline]
    fn clamp01(self) -> Self {
        self.clamp(Vec3::ZERO, Vec3::ONE)
    }
}

impl Clamp01 for Vec4 {
    #[inline]
    fn clamp01(self) -> Self {
        self.clamp(Vec4::ZERO, Vec4::ONE)
    }
}

pub(crate) trait IntoTransform {
    /// Convert this type into a `Transform`.
    fn into_transform(self) -> Transform;
}

impl IntoTransform for Vec3 {
    #[inline]
    fn into_transform(self) -> Transform {
        Transform::from_xyz(self.x, self.y, self.z)
    }
}

impl IntoTransform for Quat {
    #[inline]
    fn into_transform(self) -> Transform {
        Transform::from_rotation(self)
    }
}

pub(crate) trait LerpRadius {
    /// Linearly interpolate between two values, but if the distance between them is less than the
    /// radius, return the other value.
    fn lerp_radius(self, other: Self, t: f32, radius: f32) -> Self;
}

impl LerpRadius for f32 {
    #[inline]
    fn lerp_radius(self, other: Self, t: f32, radius: f32) -> Self {
        let mut result = bevy::prelude::FloatExt::lerp(self, other, t);
        if (result - other).abs() < radius {
            result = other;
        }
        result
    }
}

impl LerpRadius for Vec3 {
    #[inline]
    fn lerp_radius(self, other: Self, t: f32, radius: f32) -> Self {
        let mut result = self.lerp(other, t);
        if (result - other).length_squared() < radius {
            result = other;
        }
        result
    }
}

impl LerpRadius for Quat {
    #[inline]
    fn lerp_radius(self, other: Self, t: f32, radius: f32) -> Self {
        let mut result = self.lerp(other, t);
        if (result - other).length_squared() < radius {
            result = other;
        }
        result
    }
}

pub trait Reset: Default {
    fn reset(&mut self);
}

impl<T: Default> Reset for T {
    #[inline]
    fn reset(&mut self) {
        *self = T::default();
    }
}

pub trait Zero: PartialEq + Sized {
    const ZERO: Self;

    fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }
}

impl Zero for Vec3 {
    const ZERO: Self = Self::ZERO;
}

impl Zero for f32 {
    const ZERO: Self = 0.0;
}
