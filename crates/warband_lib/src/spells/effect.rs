use crate::prelude::*;

pub(crate) fn plugin<T: SpellEffect + Component>(app: &mut App) {
    T::configure(app);
}

pub(crate) trait SpellEffect {
    fn configure(app: &mut App);
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Spell(pub Entity);
