use std::ops::Deref;

use crate::prelude::*;
use bevy::{
    ecs::{
        component::{ComponentHooks, ComponentId, StorageType},
        world::{Command, DeferredWorld},
    },
    prelude::Trigger,
};
use enum_dispatch::enum_dispatch;

use super::AbilityTarget;

pub(super) fn plugin<T: AbilityEventType>(app: &mut App)
where
    T: Into<AbilityEvent>,
{
    app_register_types!(T, Actions<T>);

    app.observe(propagate::<T>);
}

pub(super) fn propagate<T: AbilityEventType + Into<AbilityEvent>>(
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

#[derive(Event, Clone, Reflect)]
#[enum_dispatch(AbilityEventType)]
pub(crate) enum AbilityEvent {
    OnCast(OnCast),
    OnTrigger(OnTrigger),
}

#[enum_dispatch]
pub(crate) trait AbilityEventType:
    Event + Reflect + FromReflect + TypePath + GetTypeRegistration + Clone + Send + Sync + 'static
{
    fn ability(&self) -> Entity;
    fn caster(&self) -> Entity;
    fn target(&self) -> AbilityTarget;
}

#[derive(Event, Clone, Reflect)]
pub(crate) struct OnCast {
    pub(crate) caster: Entity,
    pub(crate) target: AbilityTarget,
    pub(crate) ability: Entity,
}

impl AbilityEventType for OnCast {
    fn ability(&self) -> Entity {
        self.ability
    }
    fn caster(&self) -> Entity {
        self.caster
    }
    fn target(&self) -> AbilityTarget {
        self.target
    }
}

#[derive(Event, Clone, Reflect)]
pub(crate) struct OnTrigger {
    pub(crate) caster: Entity,
    pub(crate) target: AbilityTarget,
    pub(crate) ability: Entity,
    pub(crate) trigger: Entity,
}

impl AbilityEventType for OnTrigger {
    fn ability(&self) -> Entity {
        self.ability
    }
    fn caster(&self) -> Entity {
        self.caster
    }
    fn target(&self) -> AbilityTarget {
        self.target
    }
}

pub(crate) trait AsAbilityEventHook: AbilityEventType + Sized {
    fn actions<B: Bundle>(actions: B) -> AbilityEventHook<Self, B>;
}

impl<T: AbilityEventType> AsAbilityEventHook for T {
    fn actions<B: Bundle>(actions: B) -> AbilityEventHook<Self, B> {
        AbilityEventHook::run(actions)
    }
}

// TODO: impl & use AbilityActionBundle so only Actions(T) can be added
pub(crate) struct AbilityEventHook<T: AbilityEventType, B: Bundle> {
    _marker: std::marker::PhantomData<(T, B)>,
    pub(crate) actions: B,
}

impl<T: AbilityEventType, B: Bundle> AbilityEventHook<T, B> {
    pub(crate) fn run(actions: B) -> Self {
        Self {
            _marker: std::marker::PhantomData,
            actions,
        }
    }
}

impl<T: AbilityEventType, B: Bundle> Component for AbilityEventHook<T, B> {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;
    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(on_ability_event_hook::<T, B>);
    }
}

fn on_ability_event_hook<T: AbilityEventType, B: Bundle>(
    mut world: DeferredWorld<'_>,
    entity: Entity,
    _cid: ComponentId,
) {
    world.commands().add(InitAbilityEventHookCommand {
        entity,
        _marker: std::marker::PhantomData::<(T, B)>,
    });
}

struct InitAbilityEventHookCommand<T: AbilityEventType, B: Bundle> {
    entity: Entity,
    _marker: std::marker::PhantomData<(T, B)>,
}

impl<T: AbilityEventType, B: Bundle> Command for InitAbilityEventHookCommand<T, B> {
    fn apply(self, world: &mut World) {
        let Some(mut entity_mut) = world.get_entity_mut(self.entity) else {
            #[cfg(debug_assertions)]
            panic!("entity not found");

            #[cfg(not(debug_assertions))]
            return;
        };

        let Some(trigger) = entity_mut.take::<AbilityEventHook<T, B>>() else {
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

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(crate) struct Actions<T: AbilityEventType> {
    pub(crate) hooks: SmallVec<[Entity; 8]>,
    #[reflect(ignore)]
    _marker: std::marker::PhantomData<T>,
}

impl<T: AbilityEventType> Deref for Actions<T> {
    type Target = SmallVec<[Entity; 8]>;
    fn deref(&self) -> &Self::Target {
        &self.hooks
    }
}

impl<T: AbilityEventType> Actions<T> {
    pub(crate) fn new(entities: impl IntoIterator<Item = Entity>) -> Self {
        Self {
            _marker: std::marker::PhantomData,
            hooks: entities.into_iter().collect(),
        }
    }
}
