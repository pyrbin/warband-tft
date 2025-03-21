use super::{
    modifier::{
        Accumulated,
        Additive,
        Flat,
        Mult,
        {self},
    },
    StatSystems,
};
use crate::{prelude::*, stats::pool};

pub(crate) fn configure_stat<S: Stat>(app: &mut App)
where
    S: Component + GetTypeRegistration,
{
    app_register_types!(
        S,
        Dirty<S>,
        pool::Current<S>,
        pool::DirtyCurrent<S>,
        modifier::Flat<S>,
        modifier::Additive<S>,
        modifier::Mult<S>
    );

    app.add_plugins((
        modifier::plugin::<modifier::Flat<S>, S>,
        modifier::plugin::<modifier::Additive<S>, S>,
        modifier::plugin::<modifier::Mult<S>, S>,
    ));

    app.observe(on_insert::<S>);
    app.observe(on_remove::<S>);

    app.add_systems(
        PostUpdate,
        modifier::modifies_changed::<S>.in_set(StatSystems::Dirty),
    );

    app.add_systems(
        PostUpdate,
        (pool::current::<S>, pool::clamp::<S>)
            .chain()
            .in_set(StatSystems::Reset),
    );

    app.add_systems(
        PostUpdate,
        modifier::compute::<S>.in_set(StatSystems::Compute),
    );

    app.add_systems(
        PostUpdate,
        (dirty_cleanup::<S>, pool::cleanup::<S>).in_set(StatSystems::Cleanup),
    );
}

pub trait Stat:
    Reflect
    + FromReflect
    + GetTypeRegistration
    + Clone
    + TypePath
    + Default
    + Sync
    + Send
    + Sized
    + Copy
    + 'static
{
    /// Creates a new [Stat] with the given value.
    fn new(value: f32) -> Self;

    /// Returns the value of the [Stat].
    fn value(&self) -> f32;

    /// Clamps the value of the [Stat].
    fn clamp(value: f32) -> f32 {
        value
    }

    /// Rounds the value of the [Stat].
    fn round(value: f32) -> f32 {
        value
    }

    /// Returns a [`pool::PoolBundle<Self>`] of [`Self`] with the given value.
    fn pool(value: f32) -> pool::PoolBundle<Self>
    where
        Self: Component,
    {
        pool::PoolBundle::new(value)
    }
}

// TODO: spawn as a child of the parent entity, move to hook
fn on_insert<S: Stat + Component>(
    trigger: Trigger<OnAdd, S>,
    stat: Query<&S>,
    mut commands: Commands,
) {
    let entity = trigger.entity();
    let stat = stat.get(entity).unwrap();

    commands
        .entity(entity)
        .insert(modifier::Flat::<S>::from(*stat))
        .insert(Dirty::<S>::default());
}

// TODO: move to hook
fn on_remove<S: Stat + Component>(
    trigger: Trigger<OnRemove, S>,
    without_modifies: Query<Entity, Without<modifier::Modifies>>,
    mut commands: Commands,
) {
    let entity = trigger.entity();
    if without_modifies.get(entity).is_ok()
        && let Some(mut entity_commands) = commands.get_entity(entity)
    {
        entity_commands
            .remove::<modifier::Flat<S>>()
            .remove::<Accumulated<Flat<S>, S>>()
            .remove::<Accumulated<Additive<S>, S>>()
            .remove::<Accumulated<Mult<S>, S>>()
            .remove::<Dirty<S>>();
    }
}

fn dirty_cleanup<S: Stat + Component>(
    with_dirty: Query<Entity, With<Dirty<S>>>,
    mut commands: Commands,
) {
    for entity in &with_dirty {
        commands.entity(entity).remove::<Dirty<S>>();
    }
}
