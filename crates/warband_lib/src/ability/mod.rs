use std::borrow::Cow;

use action::*;
use effect::*;

use effect::Interval;
use enum_dispatch::enum_dispatch;

pub mod action;
pub mod effect;

use crate::prelude::*;

fn fireball() -> impl Bundle {
    (
        Name::new("fireball"),
        AbilityType::Projectile,
        School::FIRE,
        Targets::ENTITY | Targets::CASTER,
        TargetTeam::HOSTILE | TargetTeam::FRIENDLY,
        EventTrigger::OnCast((Action(Targets::POINT, AbilityTarget::None),)),
        EventTrigger::OnHit(Effect(Targets::ENTITY, EffectId("ignite".into()))),
    )
}

fn ignite() -> impl Bundle {
    (
        EffectType::Aura,
        AuraType::Debuff,
        School::FIRE,
        Duration(5.0),
        Interval(0.4),
        OnInterval(Action(Targets::ENTITY, AbilityTarget::None)),
    )
}

#[derive(Reflect, Component, Clone, Default, Debug)]
#[reflect(Component, Default, Debug)]
pub(crate) struct AbilityId(pub(crate) Cow<'static, str>);

#[derive(Component, Debug)]
pub enum EventTrigger<B: Bundle> {
    OnHit(B),
    OnCast(B),
}

#[derive(Event, Clone, Reflect)]
#[enum_dispatch(AbilityEvent)]
pub(crate) enum AbilityEvents {
    OnCast(OnCast),
    OnHit(OnHit),
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
    pub struct Targets: u8 {
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
//         Targets::ENTITY,
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
