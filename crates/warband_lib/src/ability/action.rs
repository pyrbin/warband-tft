use core::fmt;

use bevy::ecs::{system::SystemId, world::Command};

use super::{
    event::{AbilityEvent, AbilityEventType},
    Target,
};
use crate::prelude::*;

pub(crate) trait AbilityAction:
    Reflect + FromReflect + GetTypeRegistration + TypePath + Clone + Send + Sync + 'static
{
    type Provider: ActionSystemIdProvider<Self> + FromWorld;
    type Data: Reflect
        + From<Self>
        + FromReflect
        + GetTypeRegistration
        + TypePath
        + Default
        + Clone
        + Send
        + Sync
        + 'static;
}

pub(crate) fn configure_action<T: AbilityAction>(app: &mut App) {
    app_register_types!(T, Action<T>, ActionInput<T>);

    {
        let world = app.world_mut();
        world.init_component::<Action<T>>();
    }

    app.observe(action_trigger::<T>);
}

fn action_trigger<T: AbilityAction>(
    trigger: Trigger<AbilityEvent>,
    action: Query<&Action<T>>,
    transforms: Query<&GlobalTransform>,
    mut commands: Commands,
) {
    let entity = trigger.entity();
    let event = trigger.event();
    let action = or_return_quiet!(action.get(entity));

    let targets = action.targets();
    let point = or_return!(event.target().global_translation(&transforms));

    if targets.contains(Targets::POINT) {
        commands.add(AbilityActionCommand::<T>::new(ActionInput {
            entity,
            target: Target::Point(point),
            event: event.clone(),
            data: action.action().clone().into(),
        }));
    }

    if targets.contains(Targets::ENTITY)
        && let Target::Entity(target) = event.target()
    {
        commands.add(AbilityActionCommand::<T>::new(ActionInput::<T> {
            entity,
            target: Target::Entity(target),
            event: event.clone(),
            data: action.action().clone().into(),
        }));
    }

    if targets.contains(Targets::CASTER) {
        commands.add(AbilityActionCommand::<T>::new(ActionInput::<T> {
            entity,
            target: Target::Entity(event.caster()),
            event: event.clone(),
            data: action.action().clone().into(),
        }));
    }

    // if targets.contains(Targets::RADIUS)
}

bitflags::bitflags! {
    #[derive(Default, Component, Reflect)]
    #[reflect(Component, PartialEq)]
    pub(crate) struct Targets: u8 {
        const ENTITY = 1 << 0;
        const POINT = 1 << 1;
        const CASTER = 1 << 2;
    }
}

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub(crate) enum Prop<T: Component> {
    Ability,
    #[default]
    Parent,
    Caster,
    Target,
    This,
    /// !!! should not be used.
    _Marker(#[reflect(ignore)] PhantomData<T>),
}

#[derive(Component, Clone, Reflect)]
#[reflect(Component)]
pub(crate) struct Action<T: AbilityAction>(pub(crate) Targets, pub(crate) T);

impl<T: AbilityAction> Action<T> {
    pub(crate) fn targets(&self) -> &Targets {
        &self.0
    }

    pub(crate) fn action(&self) -> &T {
        &self.1
    }
}

#[derive(Clone, Reflect)]
pub(crate) struct ActionInput<T: AbilityAction> {
    pub(crate) entity: Entity,
    pub(crate) target: Target,
    pub(crate) event: AbilityEvent,
    pub(crate) data: T::Data,
}

impl<T: AbilityAction> fmt::Debug for ActionInput<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ActionInput")
            .field("entity", &self.entity)
            .field("target", &self.target)
            .field("event", &self.event)
            .finish()
    }
}

impl<T: AbilityAction> fmt::Display for ActionInput<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ActionInput {{ entity: {:?}, target: {:?}, event: {:?} }}",
            self.entity, self.target, self.event
        )
    }
}

pub(crate) trait ActionSystemIdProvider<T: AbilityAction>:
    Resource + Send + Sync + 'static
{
    fn id(&self) -> SystemId<ActionInput<T>>;
}

#[derive(Resource)]
pub(crate) struct ActionSystemId<T: AbilityAction> {
    pub(crate) id: SystemId<ActionInput<T>>,
}

impl<T: AbilityAction> ActionSystemIdProvider<T> for ActionSystemId<T> {
    fn id(&self) -> SystemId<ActionInput<T>> {
        self.id
    }
}

pub(crate) struct AbilityActionCommand<T: AbilityAction> {
    input: ActionInput<T>,
}

impl<T: AbilityAction> AbilityActionCommand<T> {
    pub(crate) fn new(input: ActionInput<T>) -> Self {
        Self { input }
    }
}

impl<T: AbilityAction> Command for AbilityActionCommand<T> {
    fn apply(self, world: &mut World) {
        let system = match world.get_resource::<T::Provider>() {
            Some(system) => system,
            None => {
                world.init_resource::<T::Provider>();
                world
                    .get_resource::<T::Provider>()
                    .expect("it was just inserted")
            },
        };

        if let Err(err) = world.run_system_with_input(system.id(), self.input) {
            error!("error running action system! {}", err)
        }
    }
}
