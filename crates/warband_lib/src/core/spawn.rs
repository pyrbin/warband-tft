use bevy::ecs::{
    system::{EntityCommands, SystemId},
    world::Command,
};

use crate::prelude::*;
use std::marker::PhantomData;

#[derive(Resource)]
pub struct SpawnSystemId<I, O = ()> {
    pub id: SystemId<(Entity, I), O>,
}

pub struct SpawnSystemCommand<I, O> {
    pub entity: Entity,
    pub arguments: I,
    _marker: PhantomData<O>,
}

impl<I, O> SpawnSystemCommand<I, O> {
    pub fn new(entity: Entity, arguments: I) -> Self {
        Self {
            entity,
            arguments,
            _marker: default(),
        }
    }
}

impl<I, O> Command for SpawnSystemCommand<I, O>
where
    I: Send + 'static,
    O: Send + 'static,
    SpawnSystemId<I, O>: FromWorld,
{
    fn apply(self, world: &mut World) {
        let system = match world.get_resource::<SpawnSystemId<I, O>>() {
            Some(system) => system,
            None => {
                world.init_resource::<SpawnSystemId<I, O>>();
                world
                    .get_resource::<SpawnSystemId<I, O>>()
                    .expect("it was just inserted")
            }
        };

        if let Err(err) = world.run_system_with_input(system.id, (self.entity, self.arguments)) {
            error!("error running spawn system! {}", err)
        }
    }
}

pub trait SpawnExtensions<I, O> {
    fn spawn_from(&mut self, input: I) -> EntityCommands;
}

impl<'w, 's, I, O> SpawnExtensions<I, O> for Commands<'w, 's>
where
    I: Send + 'static,
    O: Send + 'static,
    SpawnSystemId<I, O>: FromWorld,
{
    fn spawn_from(&mut self, input: I) -> EntityCommands {
        let entity = self.spawn_empty().id();
        self.add(SpawnSystemCommand::new(entity, input));
        self.entity(entity)
    }
}
