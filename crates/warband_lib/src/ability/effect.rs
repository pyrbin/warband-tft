use std::borrow::Cow;

use crate::prelude::*;

#[derive(Reflect, Component, Clone, Default, Debug)]
#[reflect(Component, Default, Debug)]
pub(crate) struct EffectId(pub(crate) Cow<'static, str>);

#[derive(Component)]
pub(crate) enum EffectType {
    Aura,
    Instant,
}

#[derive(Component)]
pub(crate) enum AuraType {
    Debuff,
    Buff,
}

#[derive(Stat, Component, Default, Reflect, Copy, Clone)]
#[clamp(min = 0)]
pub(crate) struct Interval(pub(crate) f32);

#[derive(Component, Default, Reflect, Copy, Clone)]
pub(crate) struct OnInterval<B: Bundle>(pub(crate) B);

#[derive(Stat, Component, Default, Reflect, Copy, Clone)]
#[clamp(min = 0)]
pub(crate) struct Duration(pub(crate) f32);
