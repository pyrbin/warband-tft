use super::{
    action::{Action, ActionInput, Prop, Targets},
    event::{AbilityEventType, Actions, CreateActionEventBuilder, OnCast, OnTrigger},
    projectile::ProjectileType,
    AbilityBundle,
    AbilityType,
    Element,
    Mana,
    Radius,
    Speed,
    Target,
    TargetTeam,
};
use crate::{
    prelude::*,
    unit::{combat::DamageEvent, stats::Health},
};

pub(super) fn plugin(app: &mut App) {
    app.configure::<(Fireball, Damage<Health>, Log)>();
}

#[derive(Ability, Component, Clone, Default, Reflect)]
#[ability(id = "fireball", bundle = fireball)]
pub(crate) struct Fireball;

#[inline]
fn fireball() -> impl AbilityBundle {
    (
        Fireball,
        AbilityType::Projectile,
        ProjectileType::Tracking,
        Element::FIRE,
        TargetTeam::HOSTILE,
        Actions::<OnCast>::run((Action(Targets::ENTITY, Log),)),
        Actions::<OnTrigger>::run((
            Action(
                Targets::ENTITY,
                Damage::<Health> {
                    amount: Prop::Target,
                    scale: 0.2,
                    can_crit: true,
                },
            ),
            Action(Targets::ENTITY, Log),
        )),
        Speed(4.0),
        Radius(0.5),
        Mana(20.0),
        super::Damage(10.0),
    )
}

#[derive(AbilityAction, Clone, Default, Reflect)]
#[ability_action(damage)]
pub(crate) struct Damage<T: Stat + Component + GetTypeRegistration> {
    pub(crate) amount: Prop<T>,
    pub(crate) scale: f32,
    pub(crate) can_crit: bool,
}

fn damage<T: Stat + Component + GetTypeRegistration>(
    input: In<ActionInput<Damage<T>>>,
    mut damage_event: EventWriter<crate::unit::combat::DamageEvent>,
) {
    if let Target::Entity(entity) = input.target {
        damage_event.send(DamageEvent {
            target: entity,
            source: input.event.ability(),
            damage: input.data.amount.value() * input.data.scale,
        });
    }
}

#[derive(AbilityAction, Clone, Default, Reflect, Debug)]
#[ability_action(log)]
pub(crate) struct Log;

fn log(In(event): In<ActionInput<Log>>) {
    info!("{event}");
}
