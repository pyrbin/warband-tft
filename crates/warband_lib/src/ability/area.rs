use crate::prelude::*;

#[derive(Reflect, Component, Clone, Debug)]
#[reflect(Component, Debug)]
pub(crate) enum AreaType {
    Circle,
    Rectangle,
}
