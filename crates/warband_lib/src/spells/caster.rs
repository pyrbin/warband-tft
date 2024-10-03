use crate::prelude::*;

#[derive(Component)]
pub(crate) struct Spells(SmallVec<[Entity; 4]>);

#[derive(Component)]
pub(crate) struct Caster;
