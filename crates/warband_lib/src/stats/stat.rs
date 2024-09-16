use std::marker::PhantomData;

use bevy::reflect::TypePath;

use super::pool::{self, Current, PoolBundle};
use crate::{
    prelude::*,
    stats::{modifier, pool::DirtyCurrent, StatSystem},
};

// #TODO: Prevent manual editing of Stats. Or make it so the Stat is base value

#[builder]
pub(crate) fn plugin<S: Stat>(clamp: Option<ClampValue>) -> StatPlugin<S>
where
    S: Component + GetTypeRegistration,
{
    StatPlugin {
        clamp_value: clamp.unwrap_or_default(),
        ..Default::default()
    }
}

pub struct StatPlugin<S: Stat>
where
    S: Component + GetTypeRegistration,
{
    pub clamp_value: ClampValue,
    _marker: PhantomData<S>,
}

impl<S: Stat> Plugin for StatPlugin<S>
where
    S: Component + GetTypeRegistration,
{
    fn build(&self, app: &mut App) {
        app_register_types!(Current<S>, Dirty<S>, DirtyCurrent<S>, S);

        app.add_plugins(modifier::plugin::<S, S>().call());

        app.observe(dirty_on_added::<S>);

        app.add_systems(
            PostUpdate,
            (modifier::target_changed::<S>).in_set(StatSystem::Dirty),
        );

        app.add_systems(
            PostUpdate,
            (pool::dirty_current::<S>, dirty_reset::<S>, pool::clamp::<S>)
                .chain()
                .in_set(StatSystem::Reset),
        );

        app.add_systems(
            PostUpdate,
            (dirty_cleanup::<S>, pool::dirty_cleanup::<S>).in_set(StatSystem::Cleanup),
        );

        if !matches!(self.clamp_value, ClampValue::None) {
            let clamp_value = self.clamp_value;
            app.add_systems(
                PostUpdate,
                (move |mut stats: Query<&mut S, Changed<S>>| {
                    for mut stat in &mut stats {
                        let value: f32 = stat.value();
                        let (min, max) = match clamp_value {
                            ClampValue::AboveZero => (0.0, value.max(0.0)),
                            ClampValue::Min(min) => (min, value.max(min)),
                            ClampValue::Max(max) => (value.min(max), max),
                            ClampValue::MinMax(min, max) => (min, max),
                            _ => continue,
                        };
                        *stat.value_mut() = value.clamp(min, max);
                    }
                })
                .in_set(StatSystem::Cleanup),
            );
        }
    }
}

impl<S: Stat> Default for StatPlugin<S>
where
    S: Component + GetTypeRegistration,
{
    fn default() -> Self {
        Self {
            clamp_value: ClampValue::default(),
            _marker: PhantomData,
        }
    }
}

impl<S: Stat> StatPlugin<S>
where
    S: Component + GetTypeRegistration,
{
    #[allow(unused)]
    fn clamp(mut self, value: ClampValue) -> Self {
        self.clamp_value = value;
        self
    }
}

#[derive(Bundle, Default)]
pub struct StatBundle<S: Stat + Component> {
    stat: S,
    base: modifier::Flat<S>,
}

impl<S: Stat + Component + Default> StatBundle<S> {
    pub fn new(value: f32) -> Self {
        Self {
            stat: S::default(),
            base: modifier::Flat(S::new(value)),
        }
    }
}

impl<S: Stat + Component> From<f32> for StatBundle<S> {
    fn from(val: f32) -> Self {
        StatBundle::new(val)
    }
}

pub trait Stat: Reflect + TypePath + Default + Sync + Send + Sized + 'static {
    /// Creates a new [Stat] with the given value.
    fn new(value: f32) -> Self;

    /// Create a [StatBundle<Self>] with the given base stat value.
    fn base(value: f32) -> StatBundle<Self>
    where
        Self: Component,
    {
        StatBundle::new(value)
    }

    /// Create a [PoolBundle<Self>] with the given base stat value & current set to [100%].
    fn pool(value: f32) -> PoolBundle<Self>
    where
        Self: Component,
    {
        PoolBundle::new(value)
    }

    /// Returns the value of the [Stat].
    fn value(&self) -> f32;

    // #TODO: remove value_mut & reset to a &f32 and do an unsafe hack to reset.

    /// Returns a mutable reference to the value of the [Stat].
    /// ### note
    /// Should only be used by the stat systems.
    fn value_mut(&mut self) -> &mut f32;

    /// Resets the [Stat] to its default value.
    /// ### note
    /// Should only be used by the stat systems.
    #[inline]
    fn reset(&mut self) {
        *self.value_mut() = 0.0;
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub enum ClampValue {
    #[default]
    AboveZero,
    None,
    Min(f32),
    Max(f32),
    MinMax(f32, f32),
}

// #TODO: Maybe make it so when a stat is spawned, automatically add a Flat<T> with the base value

fn dirty_on_added<S: Stat + Component>(trigger: Trigger<OnAdd, S>, mut commands: Commands) {
    let entity = trigger.entity();
    commands.entity(entity).insert(Dirty::<S>::default());
}

fn dirty_reset<S: Stat + Component>(mut stats: Query<&mut S, With<Dirty<S>>>) {
    for mut stat in &mut stats {
        stat.reset();
    }
}

fn dirty_cleanup<S: Stat + Component>(
    mut commands: Commands,
    mut stats: Query<Entity, With<Dirty<S>>>,
) {
    for entity in &mut stats {
        commands.entity(entity).remove::<Dirty<S>>();
    }
}
