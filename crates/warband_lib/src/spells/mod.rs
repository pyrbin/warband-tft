use enumflags2::BitFlags;
use hexx::Hex;
use stats::{Coeff, Damage, Power, Size};

use crate::{prelude::*, unit::stats::Range};

pub mod augment;
pub mod caster;
pub mod effect;
pub mod stats;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(effect::plugin::<Augments>);
}

fn example(mut commands: Commands) {
    commands.spawn((
        SpellType::Projectile(ProjectileType::Homing),
        CastTarget::Entity,
        Element::FIRE,
        Damage(10.0),
        Coeff::<Power, Damage>::new(0.2),
        Range(3.0),
        Size(1.0),
    ));
}

#[derive(Event, Copy, Clone, Debug, Reflect)]
pub(crate) struct SpellPrepareEvent {
    caster: Entity,
    target: SpellTarget,
    spell: Entity,
}

#[derive(Event, Copy, Clone, Debug, Reflect)]
pub(crate) struct SpellCastEvent {
    caster: Entity,
    target: SpellTarget,
    delivery: Option<Entity>,
    spell: Entity,
}

#[derive(Event, Copy, Clone, Debug, Reflect)]
pub(crate) struct SpellImpactEvent {
    caster: Entity,
    target: SpellTarget,
    delivery: Entity,
    spell: Entity,
}

#[derive(Default, Clone, Copy, Debug, Reflect)]
pub enum SpellTarget {
    Location(Vec3),
    Entity(Entity),
    Caster,
    #[default]
    None,
}

#[derive(Component, Default, Clone, Deref, DerefMut, From)]
pub(crate) struct Augments(pub HashSet<Entity>);

#[spell_effect]
impl Augments {
    #[on(SpellCastEvent)]
    fn cast(
        In(event): In<SpellCastEvent>,
        with_augments: Query<&Augments>,
        mut commands: Commands,
    ) {
        let augments = or_return!(with_augments.get(event.spell));
        for &augment in augments.0.iter() {
            commands.trigger_targets(event, augment);
        }
    }

    #[on(SpellImpactEvent)]
    fn impact(
        In(event): In<SpellImpactEvent>,
        with_augments: Query<&Augments>,
        mut commands: Commands,
    ) {
        let augments = or_return!(with_augments.get(event.spell));
        for &augment in augments.0.iter() {
            commands.trigger_targets(event, augment);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
pub(crate) enum SpellType {
    Projectile(ProjectileType),
    Area(AreaType),
    Instant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
pub(crate) enum ProjectileType {
    Homing,
    Collision,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
pub(crate) enum AreaType {
    Circle,
    Square,
    Cone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
pub(crate) enum CastTarget {
    Entity,
    Caster,
    Location(Hex),
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
