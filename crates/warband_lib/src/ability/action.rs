use bevy::ecs::system::EntityCommands;
use dyn_clone::DynClone;

use crate::prelude::*;

use super::{events::AbilityEvents, AbilityTarget, Targets};

pub(crate) trait AbilityAction {
    fn apply(event: In<(AbilityTarget, AbilityEvents)>) {}
}

pub(crate) trait AbilityActionCommand {
    fn targets(&self) -> Targets;
    fn actions(&self) -> Box<dyn BundleBox>;
}

// TODO: just allow single action for simplicity.
#[derive(Component, Clone)]
pub(crate) struct Action<B: Bundle + Clone>(pub(crate) Targets, pub(crate) B);

impl<B: Bundle + Clone> AbilityActionCommand for Action<B> {
    fn targets(&self) -> Targets {
        self.0
    }
    fn actions(&self) -> Box<dyn BundleBox> {
        Box::new(self.1.clone())
    }
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
