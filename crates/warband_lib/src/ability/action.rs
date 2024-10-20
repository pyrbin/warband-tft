use bevy::ecs::{system::SystemId, world::Command};

use crate::{prelude::*, unit::combat::DamageEvent};

use super::{
    event::{AbilityEvent, AbilityEventType},
    AbilityTarget,
};

pub(super) fn plugin<T: AbilityAction<Data: GetTypeRegistration> + GetTypeRegistration>(
    app: &mut App,
) {
    app_register_types!(T, Action<T>, ActionInput<T>);
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
    let action = or_return_quiet!(action.get(entity));

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
            data: action.action().clone().into(),
        }));
    }

    if targets.contains(Targets::ENTITY)
        && let AbilityTarget::Entity(target) = event.target()
    {
        commands.add(AbilityActionCommand::<T>::new(ActionInput::<T> {
            entity,
            target: AbilityTarget::Entity(target),
            event: event.clone(),
            data: action.action().clone().into(),
        }));
    }

    if targets.contains(Targets::CASTER) {
        commands.add(AbilityActionCommand::<T>::new(ActionInput::<T> {
            entity,
            target: AbilityTarget::Entity(event.caster()),
            event: event.clone(),
            data: action.action().clone().into(),
        }));
    }

    // TODO: trigger many (radius)
}

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
    pub(crate) target: AbilityTarget,
    pub(crate) event: AbilityEvent,
    pub(crate) data: T::Data,
}

pub trait ActionSystemIdProvider<T: AbilityAction>: Resource + Send + Sync + 'static {
    fn id(&self) -> SystemId<ActionInput<T>>;
}

#[derive(Resource)]
pub struct ActionSystemId<T: AbilityAction> {
    id: SystemId<ActionInput<T>>,
}

impl<T: AbilityAction> ActionSystemIdProvider<T> for ActionSystemId<T> {
    fn id(&self) -> SystemId<ActionInput<T>> {
        self.id
    }
}

pub struct AbilityActionCommand<T: AbilityAction> {
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
            }
        };

        if let Err(err) = world.run_system_with_input(system.id(), self.input) {
            error!("error running action system! {}", err)
        }
    }
}

#[derive(AbilityAction, Clone, Default, Reflect)]
#[action(damage)]
pub(crate) struct Damage<T: Stat + Component + GetTypeRegistration> {
    pub(crate) amount: Prop<T>,
    pub(crate) scale: f32,
    pub(crate) can_crit: bool, // demo
}

fn damage<T: Stat + Component + GetTypeRegistration>(
    input: In<ActionInput<Damage<T>>>,
    mut damage_event: EventWriter<crate::unit::combat::DamageEvent>,
) {
    if let AbilityTarget::Entity(entity) = input.target {
        damage_event.send(DamageEvent {
            target: entity,
            source: input.event.ability(),
            damage: input.data.amount.value() * input.data.scale,
        });
    }
}

#[derive(AbilityAction, Clone, Default, Reflect)]
#[action(log)]
pub(crate) struct Log;

fn log(event: In<ActionInput<Log>>) {
    println!("log :)");
}
