use crate::prelude::*;

use super::{SpellCastEvent, SpellPrepareEvent};

pub(crate) fn plugin<T: SpellEffectConfiguration + Component>(app: &mut App) {
    T::configure(app);
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Spell(pub Entity);

pub(crate) trait SpellEffectConfiguration: Component + 'static {
    fn configure(app: &mut App);
}

#[derive(Component, Clone, Copy, Debug)]
struct DealDamage;

#[spell_effect]
impl DealDamage {
    #[on(SpellPrepareEvent)]
    fn prepare(event: In<SpellPrepareEvent>) {}

    #[on(SpellCastEvent)]
    fn cast(event: In<SpellCastEvent>) {}
}
