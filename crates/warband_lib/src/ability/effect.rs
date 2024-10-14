use std::borrow::Cow;

use crate::prelude::*;

use super::AbilityAction;

#[derive(Reflect, Component, Clone, Default, Debug)]
#[reflect(Component, Default, Debug)]
pub(crate) struct EffectId(pub(crate) Cow<'static, str>);

#[derive(Reflect, Component, Clone, Default, Debug)]
#[reflect(Component, Default, Debug)]
pub(crate) struct Effect(pub(crate) EffectId);
impl AbilityAction for Effect {}

#[derive(Component)]
pub(crate) enum Aura {
    Debuff,
    Buff,
}
