use std::borrow::Cow;

use enum_dispatch::enum_dispatch;

use crate::prelude::*;

#[derive(Reflect, Component, Clone, Default, Debug)]
#[reflect(Component, Default, Debug)]
pub(crate) struct EffectId(pub(crate) Cow<'static, str>);

#[derive(Reflect, Component, Clone, Default, Debug)]
#[reflect(Component, Default, Debug)]
pub(crate) struct Effect(pub(crate) EffectId);

#[derive(Component)]
pub(crate) enum Aura {
    Debuff,
    Buff,
}
