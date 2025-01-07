use std::borrow::Cow;

use super::action::Targets;
use crate::prelude::*;

#[derive(Reflect, Clone, Component, Default, Debug, Deref, Hash, PartialEq, Eq, Display, From)]
#[reflect(Component, Default, Debug)]
pub(crate) struct EffectId(Cow<'static, str>);

impl EffectId {
    const fn new(name: &'static str) -> Self {
        Self(Cow::Borrowed(name))
    }
}

#[derive(Reflect, Component, Clone, Default, Debug)]
#[reflect(Component, Default, Debug)]
pub(crate) struct Effect(pub(crate) EffectId);

#[derive(Reflect, Component, Clone, Debug)]
#[reflect(Component, Debug)]
pub(crate) enum EffectType {
    Area,
    Projectile,
}

#[derive(Component)]
pub(crate) enum Aura {
    Debuff,
    Buff,
}

#[derive(Effect)]
#[effect(id = "burn", bundle = burn)]
pub(crate) struct Burn;

#[inline]
fn burn() -> impl EffectBundle {
    (
        Burn,
        EffectType::Ailment,
        Element::Fire,
        Duration(4.0),
        Interval(1.0),
        Actions::<OnTrigger>::run((Action(Targets::ENTITY, Damage { amount: Prop::This }),)),
        Stacking::Override,
    )
}

pub(crate) struct DurationTimer(Timer);

pub(crate) struct IntervalTimer(Timer);

fn handle_effect(effects: Query<(&EffectType, Option<&Duration>, Option<&Interval>)>) {
    for (type, duration, interval) in &effects {

    }

}
