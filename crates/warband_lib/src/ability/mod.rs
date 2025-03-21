use core::panic;
use std::borrow::Cow;

use bevy::ecs::component::{ComponentHooks, StorageType};
use enumflags2::BitFlags;
use event::AbilityEventType;
use projectile::{ProjectileEvent, ProjectileType, TrackingProjectile};
use spawn::SpawnExtensions;

pub mod action;
pub mod area;
pub mod caster;
// pub mod effect;
pub mod event;
pub mod example;
pub mod projectile;
pub mod slot;

use crate::{prelude::*, unit::Allegiance};

// [ ] Effects
// [/] Ability Registration
// [/] Ability Caster & Ability Slots
// [ ] Area Delivery Trigger
// [x] Projectile Delivery Trigger
// [ ] Instant Delivery Trigger
// [ ] Action Targeting, Proc Rate, ActionParams / ActionTarget Builder
// [ ] Linear Projectile / Tracking Distiction
// [ ] Fire Sound Action
// [ ] Particle Action
// [ ] Try Sprite 3D Projectile
// [ ] More Ability & Effect Events

pub(super) fn plugin(app: &mut App) {
    app_register_types!(
        Ability,
        AbilityId,
        AbilityType,
        Target,
        Element,
        CanTarget,
        FromAbility,
        Caster
    );

    app.init_resource::<AbilityRegistry>();
    app.configure::<(Interval, Duration, Radius, Speed, Mana)>();

    app.add_plugins(example::plugin);
    app.add_plugins(projectile::plugin);
    app.add_plugins(caster::plugin);
    app.add_plugins(slot::plugin);
    app.add_plugins(event::plugin);

    app.add_systems(Update, projectile_events);

    app.observe(cast);
}

pub(crate) fn configure_ability<T: AbilityData>(app: &mut App) {
    app.world_mut()
        .resource_scope(|_, mut abilities: Mut<'_, AbilityRegistry>| {
            let id = T::ID;
            let bundle: Box<dyn BundleBox> = Box::new(T::bundle().clone());
            abilities.0.try_insert(id, bundle).unwrap_or_else(|err| {
                panic!(
                    "Failed to insert ability with ID {:?} into registry",
                    err.entry.key()
                );
            });
        });
}

#[derive(Reflect, Resource, DerefMut, Deref, Default)]
#[reflect(Resource)]
pub(crate) struct AbilityRegistry(HashMap<AbilityId, Box<dyn BundleBox>>);

pub(crate) trait AbilityData {
    const ID: AbilityId;
    fn bundle() -> impl AbilityBundle;
}

pub(crate) trait AbilityBundle: Bundle + Clone {}
impl<T: Bundle + Clone> AbilityBundle for T {}

#[derive(Reflect, Clone, Default, Debug, Deref, Hash, PartialEq, Eq, Display, From)]
#[reflect(Component, Default, Debug)]
pub(crate) struct Ability(pub(crate) AbilityId);

impl Component for Ability {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(
            |mut world: bevy::ecs::world::DeferredWorld<'_>, entity, _| {
                let ability = {
                    let ability = world.get::<Ability>(entity);
                    ability.cloned()
                }
                .expect("ability should exist");
                world.commands().insert_from(entity, ability);
            },
        );
    }
}

#[spawner(Ability)]
fn spawn(
    In((id, args)): In<(Entity, Ability)>,
    mut commands: Commands,
    abilities: Res<AbilityRegistry>,
) {
    let ability_id = args.0;
    if let Some(bundle) = abilities.0.get(&ability_id) {
        {
            let entity_commands = commands.entity(id);
            bundle.insert(entity_commands);
        }
        {
            let mut entity_commands = commands.entity(id);
            let id_str = ability_id.0.clone();
            entity_commands
                .remove::<Ability>()
                .insert(ability_id.clone())
                .insert(Name::ability(id_str));
        }
    } else {
        panic!("tried to spawn non-existing ability {ability_id:?}");
    }
}

#[derive(Reflect, Clone, Component, Default, Debug, Deref, Hash, PartialEq, Eq, Display, From)]
#[reflect(Component, Default, Debug)]
pub(crate) struct AbilityId(Cow<'static, str>);

impl AbilityId {
    const fn new(name: &'static str) -> Self {
        Self(Cow::Borrowed(name))
    }
}

#[derive(Reflect, Component, Clone, Debug)]
#[reflect(Component, Debug)]
pub(crate) enum AbilityType {
    Area,
    Projectile,
}

bitflags::bitflags! {
    #[derive(Default, Component, Reflect)]
    #[reflect(Component, PartialEq)]
    pub(crate) struct Element: u8 {
        const FIRE = 1 << 0;
        const COLD = 1 << 1;
        const NATURE = 1 << 2;
        const STORM = 1 << 3;
    }
}

bitflags::bitflags! {
    #[derive(Default, Component, Reflect)]
    #[reflect(Component, PartialEq)]
    pub(crate) struct CanTarget: u8 {
        const HOSTILE = 1 << 0;
        const FRIENDLY = 1 << 1;
    }
}

impl CanTarget {
    pub(crate) const fn targetable(&self, source: Allegiance) -> Allegiance {
        let mut targetable = Allegiance::NONE;
        if self.contains(CanTarget::HOSTILE) {
            targetable = targetable.union(source.enemy());
        }
        if self.contains(CanTarget::FRIENDLY) {
            targetable = targetable.union(source.ally());
        }
        targetable
    }

    pub(crate) const fn can_target(&self, source: Allegiance, target: Allegiance) -> bool {
        if self.contains(CanTarget::HOSTILE) && source.is_enemy(target) {
            return true;
        }
        if self.contains(CanTarget::FRIENDLY) && source.is_ally(target) {
            return true;
        }
        false
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
pub(crate) struct CastTime(pub(crate) f32);

#[derive(Stat, Component, Default, Reflect, Copy, Clone)]
#[clamp(min = 0)]
pub(crate) struct Radius(pub(crate) f32);

#[derive(Stat, Component, Default, Reflect, Copy, Clone)]
#[clamp(min = 0)]
pub(crate) struct Speed(pub(crate) f32);

#[derive(Stat, Component, Default, Reflect, Copy, Clone)]
#[clamp(min = 0)]
pub(crate) struct Damage(pub(crate) f32);

#[derive(Stat, Component, Default, Reflect, Copy, Clone)]
#[clamp(min = 0)]
pub(crate) struct Mana(pub(crate) f32);

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
        },
        AbilityType::Projectile => {
            let mut lens = ability_args.transmute_lens::<ProjectileArguments>();
            let projectile_args = lens.query();
            let (projecile_type, speed, radius) = or_return!(projectile_args.get(entity));

            let projecile_type = or_return!(projecile_type);
            let speed = or_return!(speed);
            let radius = or_return!(radius);

            commands
                .spawn_from(TrackingProjectile {
                    projectile_target: event.target,
                    projectile_type: projecile_type.clone(),
                    filter: Allegiance::TEAM_2, // TODO: corrent filter
                    radius: radius.value(),
                    speed: speed.value(),
                    origin: caster_position,
                })
                .insert((FromAbility(entity), Name::projectile("example spell")));
        },
    }
}

fn projectile_events(
    mut events: EventReader<ProjectileEvent>,
    mut commands: Commands,
    projectiles: Query<&FromAbility>,
    abilities: Query<&Caster, With<AbilityId>>,
) {
    for event in events.read() {
        if let ProjectileEvent::Hit { projectile, target } = event {
            let ability = or_continue!(projectiles.get(*projectile));
            let caster = or_continue!(abilities.get(**ability));
            commands.trigger_targets(
                event::OnTrigger {
                    ability: **ability,
                    target: Target::Entity(*target),
                    caster: **caster,
                    trigger: *projectile,
                },
                **ability,
            );
        }
    }
}
