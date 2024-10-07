use crate::prelude::*;
use enumflags2::BitFlags;

use super::{SpellCastEvent, SpellEvent, SpellImpactEvent, SpellPrepareEvent, Tags};

#[derive(Component, Default, Clone, Deref, DerefMut, From)]
pub(crate) struct Augments(pub HashSet<Entity>);

#[spell_effect]
impl Augments {
    #[on(SpellPrepareEvent)]
    fn on_prepare(
        In(event): In<SpellEvent<SpellPrepareEvent>>,
        with_augments: Query<&Augments>,
        mut commands: Commands,
    ) {
        let event = event.event;
        let augments = or_return!(with_augments.get(event.spell));
        for &augment in augments.0.iter() {
            commands.trigger_targets(event, augment);
        }
    }

    #[on(SpellCastEvent)]
    fn on_cast(
        In(event): In<SpellEvent<SpellCastEvent>>,
        with_augments: Query<&Augments>,
        mut commands: Commands,
    ) {
        let event = event.event;
        let augments = or_return!(with_augments.get(event.spell));
        for &augment in augments.0.iter() {
            commands.trigger_targets(event, augment);
        }
    }

    #[on(SpellImpactEvent)]
    fn on_impact(
        In(event): In<SpellEvent<SpellImpactEvent>>,
        with_augments: Query<&Augments>,
        mut commands: Commands,
    ) {
        let event = event.event;
        let augments = or_return!(with_augments.get(event.spell));
        for &augment in augments.0.iter() {
            commands.trigger_targets(event, augment);
        }
    }
}

#[derive(Default, Component)]
pub(crate) struct RequiredTags(BitFlags<Tags>);
