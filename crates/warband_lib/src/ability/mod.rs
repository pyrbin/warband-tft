use std::borrow::Cow;

use action::{Action, Targets};
use cast::{cast_ability, try_ability, CastAbility, TryAbility};
use enumflags2::BitFlags;
use event::{AbilityEventType, AsAbilityEventHook, OnCast, OnTrigger};
use projectile::{ProjectileEvent, ProjectileTarget, ProjectileType, TrackingProjectile};
use spawn::SpawnExtensions;

pub mod action;
pub mod area;
pub mod cast;
pub mod effect;
pub mod event;
pub mod projectile;

use crate::{
    for_in_match,
    prelude::*,
    register_stats,
    stats::stat,
    unit::{stats::Health, Allegiance},
};

// TODO:
// - Ability Registration #[ability([id])]
// - Projectile Delivery Trigger
// - Effect Lifecycle
// - Area Delivery Trigger

pub(super) fn example() -> impl Bundle {
    (
        AbilityId("example".into()),
        AbilityType::Projectile,
        ProjectileType::Tracking,
        Element::FIRE,
        TargetTeam::HOSTILE,
        OnTrigger::actions(Action(
            Targets::ENTITY,
            action::Damage::<Health> {
                amount: action::Prop::Target,
                scale: 0.2,
                can_crit: true,
            },
        )),
        Speed(4.0),
        Radius(0.5),
        Damage(10.0),
    )
}

pub(super) fn plugin(app: &mut App) {
    app_register_types!(
        AbilityId,
        AbilityType,
        AbilityTarget,
        Element,
        TargetTeam,
        FromAbility,
        Caster,
        TryAbility,
        CastAbility
    );

    register_stats!(app, Interval, Duration, Radius, Speed);

    app.add_event::<TryAbility>();
    app.add_event::<CastAbility>();

    app.add_plugins(projectile::plugin);
    app.add_plugins((event::plugin::<OnCast>, event::plugin::<OnTrigger>));

    app.add_plugins((
        action::plugin::<action::Damage<Health>>,
        action::plugin::<action::Log>,
    ));

    app.add_systems(Update, (try_ability, cast_ability));

    app.add_systems(Update, projectile_events);
    app.observe(cast);
}

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

#[derive(Stat, Component, Default, Reflect, Copy, Clone)]
#[clamp(min = 0)]
pub(crate) struct Speed(pub(crate) f32);

#[derive(Stat, Component, Default, Reflect, Copy, Clone)]
#[clamp(min = 0)]
pub(crate) struct Damage(pub(crate) f32);

#[derive(Reflect, Component, Clone, Debug, Deref, DerefMut, From)]
#[reflect(Component, Debug)]
pub(crate) struct FromAbility(pub(crate) Entity);

#[derive(Reflect, Component, Clone, Debug, Deref, DerefMut, From)]
#[reflect(Component, Debug)]
pub(crate) struct Caster(pub(crate) Entity);

#[enumflags2::bitflags]
#[derive(Copy, Clone, Debug, PartialEq, Reflect)]
#[reflect(PartialEq)]
#[repr(u8)]
pub(crate) enum Tags {
    Fire = 0b0001,
    Frost,
    Earth,
    Storm,
    Projectile,
    AreaOfEffect,
    Dot,
}

#[derive(Default, Component)]
pub(crate) struct OwnedTags(BitFlags<Tags>);

#[derive(Default, Component)]
pub(crate) struct RequiredTags(BitFlags<Tags>);

type AbilityArguments = (
    Option<&'static Speed>,
    Option<&'static Radius>,
    Option<&'static Duration>,
    Option<&'static Element>,
    Option<&'static ProjectileType>,
    Option<&'static Interval>,
);

type ProjectileArguments = (
    Option<&'static ProjectileType>,
    Option<&'static Speed>,
    Option<&'static Radius>,
);

fn cast(
    trigger: Trigger<event::OnCast>,
    ability: Query<&AbilityType>,
    mut ability_args: Query<AbilityArguments>,
    transforms: Query<&GlobalTransform>,
    mut commands: Commands,
) {
    let entity = trigger.entity();
    let event = trigger.event();

    let ability_type = or_return!(ability.get(entity));
    let caster_position = or_return!(global_translation(&transforms, event.caster()));

    match ability_type {
        AbilityType::Area => {
            todo!("implement area");
        }
        AbilityType::Projectile => {
            let projectile_target = match event.target {
                AbilityTarget::Point(point) => ProjectileTarget::Point(point.x0y()),
                AbilityTarget::Entity(entity) => ProjectileTarget::Entity(entity),
                AbilityTarget::None => return,
            };

            let mut lens = ability_args.transmute_lens::<ProjectileArguments>();
            let projectile_args = lens.query();
            let (projecile_type, speed, radius) = or_return!(projectile_args.get(entity));

            let projecile_type = or_return!(projecile_type);
            let speed = or_return!(speed);
            let radius = or_return!(radius);

            commands
                .spawn_from(TrackingProjectile {
                    projectile_target,
                    projectile_type: projecile_type.clone(),
                    filter: Allegiance::TEAM_2, // TODO: corrent filter
                    radius: radius.value(),
                    speed: speed.value(),
                    origin: caster_position,
                })
                .insert((FromAbility(entity), Name::projectile("example spell")));
        }
    }
}

fn projectile_events(
    mut events: EventReader<ProjectileEvent>,
    mut commands: Commands,
    projectiles: Query<&FromAbility>,
    abilities: Query<&Caster, With<AbilityId>>,
) {
    for_in_match!(events.read(),
        ProjectileEvent::Hit { projectile, target }  => {
            let ability = or_continue!(projectiles.get(*projectile));
            let caster = or_continue!(abilities.get(**ability));
            commands.trigger_targets(
                event::OnTrigger {
                    ability: **ability,
                    target: AbilityTarget::Entity(*target),
                    caster: **caster,
                    trigger: *projectile,
                },
                **ability,
            );
        },
        _ => {}
    );
}
