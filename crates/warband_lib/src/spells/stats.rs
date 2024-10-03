use crate::prelude::*;

#[derive(Stat, Component, Default, Reflect, Copy, Clone)]
#[clamp(min = 0)]
pub(crate) struct Size(pub f32);

#[derive(Stat, Component, Default, Reflect, Copy, Clone)]
#[clamp(min = 0)]
pub(crate) struct Power(pub f32);

#[derive(Stat, Component, Default, Reflect, Copy, Clone)]
#[clamp(min = 0)]
pub(crate) struct Mana(pub f32);

#[derive(Stat, Component, Default, Reflect, Copy, Clone)]
#[clamp(min = 0)]
pub(crate) struct Damage(pub f32);

#[derive(Stat, Component, Default, Reflect, Copy, Clone)]
#[clamp(min = 0)]
pub(crate) struct Coeff<Source: Stat, Target: Stat> {
    #[stat(value)]
    pub value: f32,
    #[reflect(ignore)]
    _marker: std::marker::PhantomData<(Source, Target)>,
}

impl<T: Stat, S: Stat> Coeff<T, S> {
    pub fn new(value: f32) -> Self {
        Self {
            value,
            _marker: std::marker::PhantomData,
        }
    }
}
