use crate::prelude::*;
use bevy::ecs::{
    component::{ComponentHooks, ComponentId, StorageType},
    world::{Command, DeferredWorld},
};
use enum_dispatch::enum_dispatch;

use super::AbilityTarget;

pub(crate) struct Hook<T: Event, B: Bundle> {
    _marker: std::marker::PhantomData<(T, B)>,
    pub(crate) actions: B,
}

impl<T: Event, B: Bundle> Hook<T, B> {
    pub(crate) fn run(actions: B) -> Self {
        Self {
            _marker: std::marker::PhantomData,
            actions,
        }
    }
}

impl<T: Event, B: Bundle> Component for Hook<T, B> {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;
    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(hook::<T, B>);
    }
}

fn hook<T: Event, B: Bundle>(mut world: DeferredWorld<'_>, entity: Entity, _cid: ComponentId) {
    world.commands().add(EventHookCommand {
        entity,
        _marker: std::marker::PhantomData::<(T, B)>,
    });
}

struct EventHookCommand<T: Event, B: Bundle> {
    entity: Entity,
    _marker: std::marker::PhantomData<(T, B)>,
}

impl<T: Event, B: Bundle> Command for EventHookCommand<T, B> {
    fn apply(self, world: &mut World) {
        let Some(mut entity_mut) = world.get_entity_mut(self.entity) else {
            #[cfg(debug_assertions)]
            panic!("entity not found");

            #[cfg(not(debug_assertions))]
            return;
        };

        let Some(hook) = entity_mut.take::<Hook<T, B>>() else {
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

        hook.actions.get_components(&mut |_, component_ptr| {
            let component_id = component_ids[bundle_component];

            unsafe {
                let child_entity = world
                    .spawn_empty()
                    .insert_by_id(component_id, component_ptr)
                    .id();

                world.entity_mut(self.entity).add_child(child_entity);

                children.push(child_entity);
            }

            bundle_component += 1;
        });

        world
            .entity_mut(self.entity)
            .insert(Hooks::<T>::new(children));
    }
}

#[derive(Component, Reflect)]
pub(crate) struct Hooks<T: Event> {
    _marker: std::marker::PhantomData<T>,
    pub(crate) hooks: SmallVec<[Entity; 8]>,
}

impl<T: Event> Hooks<T> {
    pub(crate) fn new(entities: impl IntoIterator<Item = Entity>) -> Self {
        Self {
            _marker: std::marker::PhantomData,
            hooks: entities.into_iter().collect(),
        }
    }
}

#[derive(Event, Clone, Reflect)]
#[enum_dispatch(AbilityEvent)]
pub(crate) enum AbilityEvents {
    OnCast(OnCast),
    OnTrigger(OnTrigger),
}

#[enum_dispatch]
pub(crate) trait AbilityEvent: Event + Clone + Send + Sync + 'static {
    fn spell(&self) -> Entity;
    fn caster(&self) -> Entity;
    fn target(&self) -> AbilityTarget;
}

#[derive(Event, Clone, Reflect)]
pub(crate) struct OnCast {
    caster: Entity,
    target: AbilityTarget,
    spell: Entity,
}

impl AbilityEvent for OnCast {
    fn spell(&self) -> Entity {
        self.spell
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
    caster: Entity,
    target: AbilityTarget,
    spell: Entity,
}

impl AbilityEvent for OnTrigger {
    fn spell(&self) -> Entity {
        self.spell
    }
    fn caster(&self) -> Entity {
        self.caster
    }
    fn target(&self) -> AbilityTarget {
        self.target
    }
}

pub(crate) trait HookableEvent: Event + Sized {
    fn run<B: Bundle>(actions: B) -> Hook<Self, B>;
}

impl<T: AbilityEvent> HookableEvent for T {
    fn run<B: Bundle>(actions: B) -> Hook<Self, B> {
        Hook::run(actions)
    }
}
