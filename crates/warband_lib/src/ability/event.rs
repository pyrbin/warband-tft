use core::fmt;
use std::{fmt::Display, ops::Deref};

use bevy::{
    ecs::{
        component::{ComponentHooks, ComponentId, StorageType},
        world::{Command, DeferredWorld},
    },
    prelude::Trigger,
};
use enum_dispatch::enum_dispatch;

use super::Target;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app_register_types!(OnCast, OnTrigger);

    app.configure::<(OnCast, OnTrigger)>();
}

#[derive(Event, Clone, Reflect, Debug)]
#[enum_dispatch(AbilityEventType)]
pub(crate) enum AbilityEvent {
    OnCast(OnCast),
    OnTrigger(OnTrigger),
}

impl fmt::Display for AbilityEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AbilityEvent::OnCast(event) => write!(f, "{}", event),
            AbilityEvent::OnTrigger(event) => write!(f, "{}", event),
        }
    }
}

#[enum_dispatch]
pub(crate) trait AbilityEventType:
    Event
    + Reflect
    + FromReflect
    + TypePath
    + GetTypeRegistration
    + Display
    + Clone
    + Send
    + Sync
    + 'static
{
    fn ability(&self) -> Entity;
    fn caster(&self) -> Entity;
    fn target(&self) -> Target;
}

pub(crate) fn configure_ability_event<T: AbilityEventType + Into<AbilityEvent>>(app: &mut App) {
    app_register_types!(T, Actions<T>);

    app.observe(propagate::<T>);
}

fn propagate<T: AbilityEventType + Into<AbilityEvent>>(
    trigger: Trigger<T>,
    ability: Query<&Actions<T>>,
    mut commands: Commands,
) {
    let entity = trigger.entity();
    let event = trigger.event();

    if event.ability() != entity {
        return;
    }

    let action = or_return_quiet!(ability.get(entity));
    for entity in action.iter() {
        let event: AbilityEvent = event.clone().into();
        commands.trigger_targets(event, *entity);
    }
}

#[derive(AbilityEvent, Event, Clone, Reflect, Debug)]
pub(crate) struct OnCast {
    #[ability_event(caster)]
    pub(crate) caster: Entity,
    #[ability_event(target)]
    pub(crate) target: Target,
    #[ability_event(ability)]
    pub(crate) ability: Entity,
}

impl fmt::Display for OnCast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OnCast({:?})", self.ability)
    }
}

#[derive(AbilityEvent, Event, Clone, Reflect, Debug)]
pub(crate) struct OnTrigger {
    #[ability_event(caster)]
    pub(crate) caster: Entity,
    #[ability_event(target)]
    pub(crate) target: Target,
    #[ability_event(ability)]
    pub(crate) ability: Entity,
    pub(crate) trigger: Entity,
}

impl fmt::Display for OnTrigger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OnTrigger({:?})", self.trigger)
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(crate) struct Actions<T: AbilityEventType> {
    pub(crate) actions: SmallVec<[Entity; 8]>,
    #[reflect(ignore)]
    _marker: std::marker::PhantomData<T>,
}

impl<T: AbilityEventType> Deref for Actions<T> {
    type Target = SmallVec<[Entity; 8]>;
    fn deref(&self) -> &Self::Target {
        &self.actions
    }
}

impl<T: AbilityEventType> Actions<T> {
    pub(crate) fn new(entities: impl IntoIterator<Item = Entity>) -> Self {
        Self {
            actions: entities.into_iter().collect(),
            _marker: std::marker::PhantomData,
        }
    }
}

#[derive(Clone)]
pub(crate) struct ActionEventBuilder<T: AbilityEventType, B: Bundle> {
    actions: B,
    _marker: std::marker::PhantomData<(T, B)>,
}

impl<T: AbilityEventType, B: Bundle> ActionEventBuilder<T, B> {
    pub(crate) const fn run(actions: B) -> Self {
        Self {
            _marker: std::marker::PhantomData,
            actions,
        }
    }
}

impl<T: AbilityEventType, B: Bundle> Component for ActionEventBuilder<T, B> {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;
    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(action_builder_hook::<T, B>);
    }
}

fn action_builder_hook<T: AbilityEventType, B: Bundle>(
    mut world: DeferredWorld<'_>,
    entity: Entity,
    _cid: ComponentId,
) {
    world.commands().add(ActionEventBuilderCommand {
        entity,
        _marker: std::marker::PhantomData::<(T, B)>,
    });
}

struct ActionEventBuilderCommand<T: AbilityEventType, B: Bundle> {
    entity: Entity,
    _marker: std::marker::PhantomData<(T, B)>,
}

impl<T: AbilityEventType, B: Bundle> Command for ActionEventBuilderCommand<T, B> {
    fn apply(self, world: &mut World) {
        let Some(mut entity_mut) = world.get_entity_mut(self.entity) else {
            #[cfg(debug_assertions)]
            panic!("entity not found");

            #[cfg(not(debug_assertions))]
            return;
        };

        let Some(trigger) = entity_mut.take::<ActionEventBuilder<T, B>>() else {
            #[cfg(debug_assertions)]
            panic!("hook component not found");

            #[cfg(not(debug_assertions))]
            return;
        };

        let mut component_ids = Vec::new();
        B::get_component_ids(world.components(), &mut |component_id| {
            if let Some(component_id) = component_id {
                component_ids.push(component_id);
            }
        });

        let mut children: Vec<Entity> = Vec::new();
        let mut bundle_component: usize = 0;

        trigger.actions.get_components(&mut |_, component_ptr| {
            let component_id = component_ids[bundle_component];

            // SAFETY: if component has been initialized, is the case for any [AbilityAction]
            unsafe {
                let child_entity = world
                    .spawn_empty()
                    .insert(Name::new("action"))
                    .insert_by_id(component_id, component_ptr)
                    .id();

                world.entity_mut(self.entity).add_child(child_entity);

                children.push(child_entity);
            }

            bundle_component += 1;
        });

        world
            .entity_mut(self.entity)
            .insert(Actions::<T>::new(children));
    }
}

pub(crate) trait CreateActionEventBuilder: Sized {
    type Event: AbilityEventType;

    fn run<B: Bundle>(actions: B) -> ActionEventBuilder<Self::Event, B>;
}

impl<T: AbilityEventType> CreateActionEventBuilder for Actions<T> {
    type Event = T;

    fn run<B: Bundle>(actions: B) -> ActionEventBuilder<Self::Event, B> {
        ActionEventBuilder::run(actions)
    }
}
