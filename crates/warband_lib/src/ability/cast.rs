use super::{event, Target};
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app_register_types!(AbilityCaster, TryAbility, CastAbility);

    app.add_event::<TryAbility>();
    app.add_event::<CastAbility>();

    app.add_systems(Update, (try_ability, cast_ability));
}

#[derive(Component, Reflect)]
pub(crate) struct AbilityCaster;

#[derive(Component, Reflect)]
pub(crate) enum AbilityCasterStatus {
    Idle,
    Casting(Entity),
}

// TODO: convert to comman

#[derive(Event, Reflect)]
pub struct TryAbility {
    pub caster: Entity,
    pub ability: Entity,
    pub target: Target,
}

#[derive(Event, Reflect)]
pub struct CastAbility {
    pub caster: Entity,
    pub ability: Entity,
    pub target: Target,
}

fn try_ability(mut events: EventReader<TryAbility>, mut cast_ability: EventWriter<CastAbility>) {
    for TryAbility {
        caster,
        ability,
        target,
    } in events.read()
    {
        // TODO: check if the caster has enough mana etc.
        cast_ability.send(CastAbility {
            caster: *caster,
            ability: *ability,
            target: *target,
        });
    }
}

fn cast_ability(mut events: EventReader<CastAbility>, mut commands: Commands) {
    for CastAbility {
        caster,
        ability,
        target,
    } in events.read()
    {
        commands.trigger_targets(
            event::OnCast {
                caster: *caster,
                ability: *ability,
                target: *target,
            },
            *ability,
        );
    }
}
