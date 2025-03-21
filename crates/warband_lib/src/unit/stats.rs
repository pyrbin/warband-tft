use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(Health, Movement, Range)>();
}

#[derive(Stat, Component, Default, Reflect, Copy, Clone)]
#[clamp(min = 0)]
pub(crate) struct Health(pub f32);

#[derive(Stat, Component, Default, Reflect, Copy, Clone)]
#[clamp(min = 0, max = 10)]
pub(crate) struct Movement(pub f32);

#[derive(Stat, Component, Default, Reflect, Copy, Clone)]
#[clamp(min = 0, max = 10)]
#[round(round)]
pub(crate) struct Range(pub f32);

fn round(v: f32) -> f32 {
    v.round()
}
