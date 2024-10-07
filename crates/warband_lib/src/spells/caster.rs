use bevy::ecs::world::Command;

use crate::prelude::*;

use super::{SpellPrepareEvent, SpellTarget};

#[derive(Component)]
pub(crate) struct Spells(SmallVec<[Entity; 4]>);

#[derive(Component)]
pub(crate) struct Caster;

#[derive(Debug)]
pub(crate) struct CastSpell {
    pub(crate) spell: Entity,
    pub(crate) target: SpellTarget,
    pub(crate) caster: Entity,
}

impl Command for CastSpell {
    fn apply(self, world: &mut World) {
        // TODO: handle mana cost
        // TODO: check requirements?

        let mut commands = world.commands();
        commands.trigger_targets(
            SpellPrepareEvent {
                spell: self.spell,
                target: self.target,
                caster: self.caster,
            },
            self.spell,
        );
    }
}
