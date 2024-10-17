use crate::prelude::*;

use super::{event, AbilityTarget};

#[derive(Event)]
pub struct TryAbility {
    pub caster: Entity,
    pub ability: Entity,
    pub target: AbilityTarget,
}

#[derive(Event)]
pub struct CastAbility {
    pub caster: Entity,
    pub ability: Entity,
    pub target: AbilityTarget,
}

pub(super) fn try_ability(
    mut events: EventReader<TryAbility>,
    mut cast_ability: EventWriter<CastAbility>,
) {
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

pub(super) fn cast_ability(mut events: EventReader<CastAbility>, mut commands: Commands) {
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
