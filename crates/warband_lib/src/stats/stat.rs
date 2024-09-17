use super::{modifier, StatSystems};
use crate::prelude::*;

pub(crate) fn plugin<S: Stat>(app: &mut App)
where
    S: Component + GetTypeRegistration,
{
    app_register_types!(
        S,
        Dirty<S>,
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
        modifier::compute::<S>.in_set(StatSystems::Compute),
    );

    app.add_systems(PostUpdate, dirty_remove::<S>.in_set(StatSystems::Cleanup));
}

pub trait Stat: Reflect + TypePath + Default + Sync + Send + Sized + Copy + 'static {
    /// Creates a new [Stat] with the given value.
    fn new(value: f32) -> Self;

    /// Returns the value of the [Stat].
    fn value(&self) -> f32;

    /// Returns a [modifier::Flat] modifier with the given value of [Stat]
    fn flat(self) -> modifier::Flat<Self> {
        modifier::Flat::from(self)
    }

    /// Returns a [modifier::Additive] modifier with the given value of [Stat]
    fn additive(self) -> modifier::Additive<Self> {
        modifier::Additive::from(self)
    }

    /// Returns a [modifier::Mult] modifier with the given value of [Stat]
    fn mult(self) -> modifier::Mult<Self> {
        modifier::Mult::from(self)
    }

    fn clamp(&mut self) -> Self {
        *self
    }

    fn round(&mut self) -> Self {
        *self
    }
}

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

fn on_remove<S: Stat + Component>(
    trigger: Trigger<OnRemove, S>,
    without_modifies: Query<Entity, Without<modifier::Modifies>>,
    mut commands: Commands,
) {
    let entity = trigger.entity();
    if without_modifies.get(entity).is_ok() {
        commands
            .entity(entity)
            .remove::<modifier::Flat<S>>()
            .remove::<Dirty<S>>();
    }
}

fn dirty_remove<S: Stat + Component>(
    with_dirty: Query<Entity, With<Dirty<S>>>,
    mut commands: Commands,
) {
    for entity in &with_dirty {
        commands.entity(entity).remove::<Dirty<S>>();
    }
}
