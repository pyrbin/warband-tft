use std::ops::{AddAssign, MulAssign, SubAssign};

use crate::prelude::*;

#[derive(Bundle, Default)]
pub struct PoolBundle<S: Stat + Component> {
    current: Current<S>,
    stat: S,
}

impl<S: Stat + Component> PoolBundle<S> {
    #[allow(unused)]
    pub fn new(value: f32) -> Self {
        Self {
            stat: S::new(value),
            current: Current::<S>::new(value),
        }
    }
}

impl<S: Stat + Component> From<f32> for PoolBundle<S> {
    fn from(val: f32) -> Self {
        PoolBundle::new(val)
    }
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct Pool<S: Stat + Component> {
    pub(super) current: &'static mut Current<S>,
    pub(super) total: &'static S,
}

#[allow(unused)]
impl<S: Stat + Component> PoolReadOnlyItem<'_, S> {
    pub fn total(&self) -> f32 {
        self.total.value()
    }

    pub fn current(&self) -> f32 {
        **self.current
    }

    #[inline]
    pub fn progress01(&self) -> f32 {
        pool_01(self.current(), self.total())
    }
}

#[allow(unused)]
impl<S: Stat + Component> PoolItem<'_, S> {
    pub fn total(&self) -> f32 {
        self.total.value()
    }

    pub fn current(&self) -> f32 {
        **self.current
    }

    /// [0..1]
    #[inline]
    pub fn progress01(&self) -> f32 {
        pool_01(self.current(), self.total())
    }

    #[inline]
    pub fn set_current(&mut self, value: f32) {
        match pool_clamp(value, self.total()) {
            Ok(value) => self.current.0 = value,
            Err(value) => self.current.0 = value,
        };
    }
}

impl<S: Stat + Component> AddAssign<f32> for PoolItem<'_, S> {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        self.set_current(self.current() + rhs);
    }
}

impl<S: Stat + Component> SubAssign<f32> for PoolItem<'_, S> {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        self.add_assign(rhs * -1.0);
    }
}

impl<S: Stat + Component> MulAssign<f32> for PoolItem<'_, S> {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.set_current(self.current() * rhs);
    }
}

#[derive(Component, Debug, Clone, Copy, Reflect, From)]
#[reflect(Component)]
pub struct Current<S: Stat + Component>(pub(super) f32, #[reflect(ignore)] PhantomData<S>);

#[allow(unused)]
impl<S: Stat + Component> Current<S> {
    pub(super) fn new(value: f32) -> Self {
        Self(value, PhantomData)
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

impl<S: Stat + Component> Default for Current<S> {
    fn default() -> Self {
        Self(0.0, PhantomData)
    }
}

impl<S: Stat + Component> std::ops::Deref for Current<S> {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Component, Debug, Clone, Copy, Reflect, From)]
#[reflect(Component)]
pub(super) struct DirtyCurrent<S: Stat + Component> {
    pub(super) last_percentage: f32,
    #[reflect(ignore)]
    _marker: PhantomData<S>,
}

impl<S: Stat + Component> DirtyCurrent<S> {
    pub(super) fn new(percentage: f32) -> Self {
        Self {
            last_percentage: percentage,
            _marker: PhantomData,
        }
    }
}

impl<S: Stat + Component> Default for DirtyCurrent<S> {
    fn default() -> Self {
        Self {
            last_percentage: 0.0,
            _marker: PhantomData,
        }
    }
}

pub(super) fn clamp<S: Stat + Component>(
    mut stats: Query<Pool<S>, (Changed<Current<S>>, Without<Dirty<S>>)>,
) {
    for mut pool in &mut stats {
        if let Err(err) = pool_clamp(pool.current(), pool.total()) {
            pool.current.0 = err;
        }
    }
}

pub(super) fn current<S: Stat + Component>(
    mut commands: Commands,
    mut stats: Query<(Entity, &S, &Current<S>), With<Dirty<S>>>,
) {
    for (entity, stat, current) in &mut stats {
        let p = pool_01(current.value(), stat.value());
        commands.entity(entity).insert(DirtyCurrent::<S>::new(p));
    }
}

pub(super) fn cleanup<S: Stat>(
    mut commands: Commands,
    mut stats: Query<(Entity, Pool<S>, &DirtyCurrent<S>)>,
) where
    S: Component,
{
    for (entity, mut pool, dirty_current) in &mut stats {
        let p = dirty_current.last_percentage;
        pool.current.0 = p * pool.total();
        commands.entity(entity).remove::<DirtyCurrent<S>>();
    }
}

#[inline]
pub(crate) fn pool_clamp(current: f32, total: f32) -> Result<f32, f32> {
    if current < 0.0 || current > total {
        Err(current.clamp(0.0, total))
    } else {
        Ok(current)
    }
}

#[inline]
pub(crate) fn pool_01(current: f32, total: f32) -> f32 {
    const MAX_PROGRESS: f32 = 1.0;

    if current == 0.0 && total == 0.0 {
        return MAX_PROGRESS;
    }

    current / total
}
