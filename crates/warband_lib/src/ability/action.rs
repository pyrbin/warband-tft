use bevy::ecs::system::SystemId;

use crate::prelude::*;

use super::{
    event::{AbilityEvent, AbilityEventType},
    AbilityTarget,
};

pub(super) fn action_plugin<T: AbilityAction>(app: &mut App) {
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
    action_system_id: Res<ActionSystemId<T>>,
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
        commands.run_system_with_input(
            action_system_id.id,
            (AbilityTarget::Point(point), event.clone()),
        );
    }

    if targets.contains(Targets::ENTITY)
        && let AbilityTarget::Entity(entity) = event.target()
    {
        commands.run_system_with_input(
            action_system_id.id,
            (AbilityTarget::Entity(entity), event.clone()),
        );
    }

    if targets.contains(Targets::CASTER) {
        commands.run_system_with_input(
            action_system_id.id,
            (AbilityTarget::Entity(event.caster()), event.clone()),
        );
    }

    // TODO: trigger many (radius)
}

pub(crate) trait AbilityAction: Clone + Send + Sync + 'static {
    fn register(app: &mut App);
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

type ActionInput = (AbilityTarget, AbilityEvent);

#[derive(Resource)]
pub struct ActionSystemId<T: AbilityAction> {
    id: SystemId<ActionInput>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: AbilityAction> ActionSystemId<T> {
    pub(crate) fn new(id: SystemId<ActionInput>) -> Self {
        Self {
            id,
            _marker: std::marker::PhantomData,
        }
    }
}

#[derive(Clone)]
pub(crate) struct Damage;

impl AbilityAction for Damage {
    fn register(app: &mut App) {
        {
            let world = app.world_mut();
            let system_id = world.register_system(damage_action);
            world.insert_resource(ActionSystemId::<Damage>::new(system_id));
        }
    }
}

fn damage_action(event: In<ActionInput>) {}
