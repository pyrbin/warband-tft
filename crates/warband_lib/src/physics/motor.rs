use crate::prelude::*;

#[derive(Component)]
pub struct CharacterMotor;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;
