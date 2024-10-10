use bevy::ecs::system::EntityCommands;
use dyn_clone::DynClone;
use enum_dispatch::enum_dispatch;

use crate::prelude::*;

fn fireball() -> impl Bundle {
    (
        Name::new("fireball"),
        AbilityType::Projectile,
        School::FIRE,
        Targeting::ENTITY | Targeting::CASTER,
        TargetTeam::HOSTILE | TargetTeam::FRIENDLY,
        OnAbilityHit((
            Action(Targeting::POINT, AbilityTarget::None),
            Action(Targeting::POINT, AbilityTarget::None),
        )),
    )
}

#[derive(Component, Debug, Clone, Default)]
pub struct OnAbilityHit<B: Bundle>(pub B);

pub(crate) trait AbilityAction {}

// ActionStatement
pub(crate) trait AbilityActionCommand {
    fn targeting(&self) -> Targeting;
    fn actions(&self) -> Box<dyn BundleBox>; // TODO: Only allow 1 action for now
}

#[derive(Component, Clone)]
pub(crate) struct Action<B: Bundle + Clone>(Targeting, B);

impl<B: Bundle + Clone> AbilityActionCommand for Action<B> {
    fn targeting(&self) -> Targeting {
        self.0
    }
    fn actions(&self) -> Box<dyn BundleBox> {
        Box::new(self.1.clone())
    }
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
pub(crate) struct OnHit {
    caster: Entity,
    target: AbilityTarget,
    spell: Entity,
}

impl AbilityEvent for OnHit {
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
#[enum_dispatch(AbilityEvent)]
pub(crate) enum AbilityEvents {
    OnCast(OnCast),
    OnHit(OnHit),
}

#[derive(Component, Default, Clone, Copy, Debug, Reflect)]
pub enum AbilityTarget {
    Point(Vec2),
    Entity(Entity),
    #[default]
    None,
}

#[derive(Component, Clone, Reflect)]
pub enum AbilityType {
    Projectile,
}

bitflags::bitflags! {
    #[derive(Default, Component, Reflect)]
    #[reflect(Component, PartialEq)]
    pub struct School: u8 {
        const FIRE = 1 << 0;
        const FROST = 1 << 1;
        const EARTH = 1 << 2;
        const STORM = 1 << 3;
    }
}

bitflags::bitflags! {
    #[derive(Default, Component, Reflect)]
    #[reflect(Component, PartialEq)]
    pub struct Targeting: u8 {
        const ENTITY = 1 << 0;
        const POINT = 1 << 1;
        const CASTER = 1 << 2;
    }
}

bitflags::bitflags! {
    #[derive(Default, Component, Reflect)]
    #[reflect(Component, PartialEq)]
    pub struct TargetTeam: u8 {
        const HOSTILE = 1 << 0;
        const FRIENDLY = 1 << 1;
    }
}

pub(crate) trait BundleBox: DynClone + Sync + Send + 'static {
    fn insert(&self, entity_commands: EntityCommands);
    fn spawn(&self, commands: Commands);
}

dyn_clone::clone_trait_object!(BundleBox);

impl<T: Bundle + Clone> BundleBox for T {
    fn insert(&self, mut entity_commands: EntityCommands) {
        entity_commands.insert((*self).clone());
    }
    fn spawn(&self, mut commands: Commands) {
        commands.spawn((*self).clone());
    }
}

#[derive(Component)]
pub struct On<T: AbilityEvent + Clone> {
    _marker: std::marker::PhantomData<T>,
    actions: Box<dyn BundleBox>,
}

impl<T: AbilityEvent> On<T> {
    pub fn new<A: Bundle + Clone>(actions: A) -> Self {
        Self {
            _marker: std::marker::PhantomData,
            actions: Box::new(actions),
        }
    }
}

// #[enum_derive]
// enum AbilityEvent {
//     OnCreate(OnCreateEvent),
// }

// trait AbilityEvent {
//     fn event() -> AbilityEvent;
//     fn spell() -> Entity;
//     fn caster() -> Entity;
//     fn target() -> Entity;
//     fn entity() -> Entity;
// }

// #[ability("fireball")]
// fn fireball() -> impl Bundle {
//     (
//         AbilityType::Projectile,
//         ProjectileType::Tracking,
//         Targeting::ENTITY,
//         School::FIRE,
//         Damage(10.0),
//         Range(1.0),
//         Mana(10.0),
//         Size(1.0),
//         Speed(1.0),
//         On::<AbilityHit>(
//             Action(Targets::TARGET, actions::Damage),
//             Action(Targets::TARGET, actions::Effect("ignite")),
//         ),
//     )
// }

// #[effect("ignite")]
// fn ignite() -> impl Bundle {
//     (
//         EffectType::Aura,
//         AuraType::Debuff,
//         EnableState(Burning),
//         Damage(5.0),
//         Duration(1.5),
//         Interval(0.4),
//         Periodic::<Interval>(Action(Targets::TARGET, actions::Damage)),
//     )
// }
