use super::StatSystems;
use crate::prelude::*;

pub(super) fn plugin<M: Modifier<S> + Component, S: Stat + Component>(app: &mut App) {
    app.add_systems(PostUpdate, changed::<M, S>.in_set(StatSystems::Dirty));

    app.observe(removed::<M, S>);

    app.add_systems(
        PostUpdate,
        (add_accumulate::<M, S>, accumulate::<M, S>)
            .chain()
            .in_set(StatSystems::Accumulate),
    );
}

#[derive(Component, Clone, Reflect, From)]
pub enum Modifies {
    Single(Entity),
    Many(SmallVec<[Entity; 8]>),
}

pub trait Modifier<S: Stat>: Default + Send + Sync + 'static {
    fn apply(value: f32, accumulated: f32) -> f32;

    fn value(&self) -> f32;

    fn base() -> f32 {
        0.0
    }
}

#[derive(Component, Default, Reflect, FromReflect, Deref, DerefMut, From)]
#[reflect(from_reflect = false)]
pub struct Flat<S: Stat>(pub S);

impl<S: Stat, M: Stat> Modifier<S> for Flat<M> {
    fn apply(value: f32, accumulated: f32) -> f32 {
        value + accumulated
    }

    fn value(&self) -> f32 {
        self.0.value()
    }
}

#[derive(Component, Default, Reflect, FromReflect, Deref, DerefMut, From)]
#[reflect(from_reflect = false)]
pub struct Additive<S: Stat>(pub S);

impl<S: Stat, M: Stat> Modifier<S> for Additive<M> {
    fn apply(value: f32, accumulated: f32) -> f32 {
        value * accumulated
    }

    fn value(&self) -> f32 {
        self.0.value()
    }

    fn base() -> f32 {
        1.0
    }
}

#[derive(Component, Default, Reflect, FromReflect, Deref, DerefMut, From)]
#[reflect(from_reflect = false)]
pub struct Mult<S: Stat>(pub S);

impl<S: Stat, M: Stat> Modifier<S> for Mult<M> {
    fn apply(value: f32, accumulated: f32) -> f32 {
        value * accumulated
    }

    fn value(&self) -> f32 {
        self.0.value()
    }

    fn base() -> f32 {
        1.0
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[component(storage = "SparseSet")]
pub struct Accumulated<M: Modifier<S>, S: Stat> {
    pub value: f32,
    pub _marker: std::marker::PhantomData<(M, S)>,
}

impl<M: Modifier<S>, S: Stat> Accumulated<M, S> {
    fn new(value: f32) -> Self {
        Self {
            value,
            _marker: std::marker::PhantomData,
        }
    }

    fn add(&mut self, value: f32) {
        self.value += value;
    }

    fn compute(&self, value: f32) -> f32 {
        M::apply(value, self.value)
    }
}

type NonDirtyStatFilter<S> = (With<S>, Without<Dirty<S>>);

/// note: registered per [Stat] in [super::stat::plugin]
pub(super) fn modifies_changed<S: Stat>(
    mut commands: Commands,
    mut stats: Query<Entity, NonDirtyStatFilter<S>>,
    modifiers: Query<(&Modifies, &Previous<Modifies>), Changed<Modifies>>,
) where
    S: Component,
{
    let mut add_dirty_stat = |entity: &Entity| {
        if stats.get_mut(*entity).is_ok()
            && let Some(mut entity_commands) = commands.get_entity(*entity)
        {
            entity_commands.try_insert(Dirty::<S>::default());
        }
    };

    for (modifier, previous) in modifiers.iter() {
        for target in [modifier, previous.get()] {
            match target {
                Modifies::Single(entity) => add_dirty_stat(entity),
                Modifies::Many(entities) => {
                    for entity in entities.iter() {
                        add_dirty_stat(entity)
                    }
                },
            }
        }
    }
}

fn changed<M: Modifier<S>, S: Stat>(
    mut commands: Commands,
    mut stats: Query<Entity, NonDirtyStatFilter<S>>,
    modifiers: Query<(Entity, Option<&Parent>, Option<&Modifies>), Changed<M>>,
    modifier_parents: Query<(Entity, &Modifies)>,
) where
    M: Component + Modifier<S>,
    S: Component,
{
    let mut add_dirty_stat = |entity: Entity| {
        if stats.get_mut(entity).is_ok()
            && let Some(mut entity_commands) = commands.get_entity(entity)
        {
            entity_commands.try_insert(Dirty::<S>::default());
        }
    };

    for (entity, maybe_parent, maybe_target) in &modifiers {
        let modifier_target = maybe_target
            .or(maybe_parent.and_then(|p| modifier_parents.get(p.get()).ok().map(|(_, t)| t)));

        match modifier_target {
            Some(Modifies::Single(entity)) => add_dirty_stat(*entity),
            Some(Modifies::Many(entities)) => {
                for entity in entities.iter() {
                    add_dirty_stat(*entity);
                }
            },
            None => {
                if let Some(parent) = maybe_parent {
                    add_dirty_stat(parent.get())
                }

                add_dirty_stat(entity);
            },
        }
    }
}

fn removed<M: Modifier<S>, S: Stat>(
    trigger: Trigger<OnRemove, M>,
    non_dirty: Query<Entity, NonDirtyStatFilter<S>>,
    modifiers: Query<(Entity, Option<&Parent>, Option<&Modifies>), With<M>>,
    modifier_parents: Query<(Entity, &Modifies)>,
    mut commands: Commands,
) where
    M: Component + Modifier<S>,
    S: Component,
{
    let entity: Entity = trigger.entity();

    let Ok((entity, maybe_parent, maybe_target)) = modifiers.get(entity) else {
        return;
    };

    let mut add_dirty_stat = |entity: &Entity| {
        if let Ok(stat) = non_dirty.get(*entity)
            && let Some(mut entity_commands) = commands.get_entity(stat)
        {
            entity_commands.try_insert(Dirty::<S>::default());
        }
    };

    let modifier_target = maybe_target
        .or(maybe_parent.and_then(|p| modifier_parents.get(p.get()).ok().map(|(_, t)| t)));

    match modifier_target {
        Some(Modifies::Single(entity)) => add_dirty_stat(entity),
        Some(Modifies::Many(entities)) => {
            for entity in entities.iter() {
                add_dirty_stat(entity);
            }
        },
        None => {
            if let Some(parent) = maybe_parent {
                add_dirty_stat(&parent.get());
            }

            add_dirty_stat(&entity);
        },
    }
}

fn add_accumulate<M: Modifier<S>, S: Stat + Component>(
    with_dirty: Query<Entity, With<Dirty<S>>>,
    mut commands: Commands,
) {
    for entity in &with_dirty {
        let accumulated = Accumulated::<M, S>::new(M::base());
        commands.entity(entity).insert(accumulated);
    }
}

fn accumulate<M: Modifier<S>, S: Stat>(
    mut stats: Query<&mut Accumulated<M, S>, With<Dirty<S>>>,
    modifiers: Query<(Entity, &M, Option<&Parent>, Option<&Modifies>)>,
    modifier_parents: Query<(Entity, &Modifies)>,
) where
    M: Component + Modifier<S>,
    S: Component,
{
    for (entity, modifier, maybe_parent, maybe_target) in modifiers.iter() {
        let mut apply_modifier = |entity: &Entity| {
            if let Ok(mut accumulated) = stats.get_mut(*entity) {
                accumulated.add(modifier.value());
            }
        };

        let modifier_target = maybe_target
            .or(maybe_parent.and_then(|p| modifier_parents.get(p.get()).ok().map(|(_, t)| t)));

        match modifier_target {
            Some(Modifies::Single(entity)) => apply_modifier(entity),
            Some(Modifies::Many(entities)) => {
                for entity in entities.iter() {
                    apply_modifier(entity);
                }
            },
            None => {
                if let Some(parent) = maybe_parent {
                    apply_modifier(&parent.get());
                }

                apply_modifier(&entity);
            },
        }
    }
}

/// note: registered per [Stat] in [super::stat::plugin]
pub(super) fn compute<S: Stat + Component>(
    mut stat: Query<
        (
            Entity,
            &mut S,
            &Accumulated<Flat<S>, S>,
            &Accumulated<Additive<S>, S>,
            &Accumulated<Mult<S>, S>,
        ),
        With<Dirty<S>>,
    >,
    mut commands: Commands,
) {
    for (entity, mut stat, flat, additive, multiplicative) in &mut stat {
        const BASE: f32 = 0.0; // TODO: this could be something baked into the Stat

        // formula: ((base + flat) * additive) * multiplicative
        let computed = multiplicative.compute(additive.compute(flat.compute(BASE)));
        let computed = S::clamp(S::round(computed));

        if stat.value() != computed {
            *stat = S::new(computed);
        }

        commands
            .entity(entity)
            .remove::<Accumulated<Flat<S>, S>>()
            .remove::<Accumulated<Additive<S>, S>>()
            .remove::<Accumulated<Mult<S>, S>>();
    }
}
