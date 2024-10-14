use std::borrow::Cow;

use action::*;

pub mod action;
pub mod area;
pub mod effect;
pub mod events;
pub mod projectile;

use crate::prelude::*;

#[derive(Reflect, Component, Clone, Default, Debug)]
#[reflect(Component, Default, Debug)]
pub(crate) struct AbilityId(pub(crate) Cow<'static, str>);

#[derive(Reflect, Component, Clone, Debug)]
#[reflect(Component, Debug)]
pub(crate) enum AbilityType {
    Area,
    Projectile,
}

#[derive(Component, Default, Clone, Copy, Debug, Reflect)]
pub enum AbilityTarget {
    Point(Vec2),
    Entity(Entity),
    #[default]
    None,
}

bitflags::bitflags! {
    #[derive(Default, Component, Reflect)]
    #[reflect(Component, PartialEq)]
    pub struct Element: u8 {
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

#[derive(Stat, Component, Default, Reflect, Copy, Clone)]
#[clamp(min = 0)]
pub(crate) struct Interval(pub(crate) f32);

#[derive(Stat, Component, Default, Reflect, Copy, Clone)]
#[clamp(min = 0)]
pub(crate) struct Duration(pub(crate) f32);

#[derive(Stat, Component, Default, Reflect, Copy, Clone)]
#[clamp(min = 0)]
pub(crate) struct Radius(pub(crate) f32);

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
