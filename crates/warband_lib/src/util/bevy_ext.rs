use bevy::ecs::system::EntityCommands;
use dyn_clone::DynClone;

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

#[inline]
pub(crate) fn global_translation(
    global_transform_query: &Query<'_, '_, &GlobalTransform>,
    entity: Entity,
) -> Result<Vec3, ()> {
    global_transform_query
        .get(entity)
        .map(|g| g.translation())
        .map_err(|_| ())
}

#[inline]
pub(crate) fn translation(
    global_transform_query: &Query<'_, '_, &Transform>,
    entity: Entity,
) -> Result<Vec3, ()> {
    global_transform_query
        .get(entity)
        .map(|g| g.translation)
        .map_err(|_| ())
}

pub(crate) trait BundleBox: DynClone + Sync + Send + 'static {
    fn insert(&self, entity_commands: EntityCommands);
    fn spawn(&self, commands: Commands);
}

dyn_clone::clone_trait_object!(BundleBox);

impl<T: Bundle + Clone> BundleBox for T {
    fn insert(&self, mut entity_commands: EntityCommands) {
        entity_commands.insert((*self).clone());
    }
    fn spawn(&self, mut commands: Commands) {
        commands.spawn((*self).clone());
    }
}
