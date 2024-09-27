use crate::{prelude::*, stats::stat};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(stat::plugin::<Health>);
    app.add_plugins(stat::plugin::<Movement>);
}

#[derive(Stat, Component, Default, Reflect, Copy, Clone)]
#[clamp(min = 0)]
pub(crate) struct Health(pub f32);

#[derive(Stat, Component, Default, Reflect, Copy, Clone)]
#[clamp(min = 0, max = 10)]
pub(crate) struct Movement(pub f32);

#[derive(Stat, Component, Default, Reflect, Copy, Clone)]
#[clamp(min = 0, max = 10)]
pub(crate) struct Range(pub f32);
