use bevy::ecs::{system::SystemId, world::Command};

use crate::prelude::*;

use super::{
    event::{AbilityEvent, AbilityEventType},
    AbilityTarget,
};

pub(super) fn plugin<T: AbilityAction>(app: &mut App) {
    {
        let world = app.world_mut();
        world.init_component::<Action<T>>();
    }
    app.observe(action::<T>);
}

pub(super) fn action<T: AbilityAction>(
    trigger: Trigger<AbilityEvent>,
    action: Query<&Action<T>>,
    transforms: Query<&GlobalTransform>,
    mut commands: Commands,
) {
    let entity = trigger.entity();
    let event = trigger.event();
    let action = or_return!(action.get(entity));

    let targets = action.targets();
    let point = match event.target() {
        super::AbilityTarget::Point(point) => point,
        super::AbilityTarget::Entity(entity) => {
            or_return!(transforms.get(entity).map(|t| t.translation().xz()))
        }
        super::AbilityTarget::None => {
            return;
        }
    };

    if targets.contains(Targets::POINT) {
        commands.add(AbilityActionCommand::<T>::new(ActionInput {
            entity,
            target: AbilityTarget::Point(point),
            event: event.clone(),
        }));
    }

    if targets.contains(Targets::ENTITY)
        && let AbilityTarget::Entity(entity) = event.target()
    {
        commands.add(AbilityActionCommand::<T>::new(ActionInput {
            entity,
            target: AbilityTarget::Entity(entity),
            event: event.clone(),
        }));
    }

    if targets.contains(Targets::CASTER) {
        commands.add(AbilityActionCommand::<T>::new(ActionInput {
            entity,
            target: AbilityTarget::Entity(event.caster()),
            event: event.clone(),
        }));
    }

    // TODO: trigger many (radius)
}

pub(crate) trait AbilityAction: Clone + Send + Sync + 'static {
    type Provider: ActionSystemIdProvider<Self> + FromWorld;
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

#[derive(Component, Clone)]
pub(crate) struct Action<T: AbilityAction>(pub(crate) Targets, pub(crate) T);

impl<T: AbilityAction> Action<T> {
    pub(crate) fn targets(&self) -> &Targets {
        &self.0
    }

    pub(crate) fn action(&self) -> &T {
        &self.1
    }
}

pub(crate) struct ActionInput {
    pub(crate) entity: Entity,
    pub(crate) target: AbilityTarget,
    pub(crate) event: AbilityEvent,
}

pub trait ActionSystemIdProvider<T: AbilityAction>: Resource + Send + Sync + 'static {
    fn id(&self) -> SystemId<ActionInput>;
}

#[derive(Resource)]
pub struct ActionSystemId<T: AbilityAction> {
    id: SystemId<ActionInput>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: AbilityAction> ActionSystemIdProvider<T> for ActionSystemId<T> {
    fn id(&self) -> SystemId<ActionInput> {
        self.id
    }
}

pub struct AbilityActionCommand<T: AbilityAction> {
    input: ActionInput,
    _marker: PhantomData<T>,
}

impl<T: AbilityAction> AbilityActionCommand<T> {
    pub(crate) fn new(input: ActionInput) -> Self {
        Self {
            input,
            _marker: PhantomData,
        }
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
            }
        };

        if let Err(err) = world.run_system_with_input(system.id(), self.input) {
            error!("error running action system! {}", err)
        }
    }
}

#[derive(Clone)]
pub(crate) struct Damage;

// TODO: proc-macro

impl AbilityAction for Damage {
    type Provider = ActionSystemId<Damage>;
}

impl FromWorld for ActionSystemId<Damage> {
    fn from_world(world: &mut World) -> Self {
        Self {
            id: world.register_system(damage),
            _marker: PhantomData,
        }
    }
}

fn damage(event: In<ActionInput>) {}
