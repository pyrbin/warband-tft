use bevy::ecs::system::EntityCommands;

use crate::prelude::*;

pub(crate) trait Spawn {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands;
}

impl Spawn for Commands<'_, '_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

impl Spawn for ChildBuilder<'_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

pub(crate) fn global_translation(
    global_transform_query: &Query<'_, '_, &GlobalTransform>,
    entity: Entity,
) -> Result<Vec3, ()> {
    global_transform_query
        .get(entity)
        .map(|g| g.translation())
        .map_err(|_| ())
}

pub(crate) fn translation(
    global_transform_query: &Query<'_, '_, &Transform>,
    entity: Entity,
) -> Result<Vec3, ()> {
    global_transform_query
        .get(entity)
        .map(|g| g.translation)
        .map_err(|_| ())
}
