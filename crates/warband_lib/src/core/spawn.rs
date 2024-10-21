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
    pub input: I,
    _marker: PhantomData<O>,
}

impl<I, O> SpawnSystemCommand<I, O> {
    pub fn new(entity: Entity, input: I) -> Self {
        Self {
            entity,
            input,
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

        if let Err(err) = world.run_system_with_input(system.id, (self.entity, self.input)) {
            error!("error running spawn system! {}", err)
        }
    }
}

pub struct DeferredSpawnSystemCommand<I, O> {
    pub input: I,
    _marker: PhantomData<O>,
}

impl<I, O> DeferredSpawnSystemCommand<I, O> {
    pub fn new(input: I) -> Self {
        Self {
            input,
            _marker: default(),
        }
    }
}

impl<I, O> Command for DeferredSpawnSystemCommand<I, O>
where
    I: Send + 'static,
    O: Send + 'static,
    SpawnSystemId<I, O>: FromWorld,
{
    fn apply(self, world: &mut World) {
        let mut commands = world.commands();
        let entity = commands.spawn_empty().id();
        commands.add(SpawnSystemCommand::new(entity, self.input));
    }
}

pub trait SpawnExtensions<I, O> {
    /// Spawns an entity and add a command to run spawn system for [`I`].
    fn spawn_from(&mut self, input: I) -> EntityCommands;
    /// Like [`Self::spawn_from`], but entity is created when command is applied.
    fn deferred_spawn_from(&mut self, input: I) -> &mut Self;
    /// Adds a command to run spawn system for [`I`] on given entity.
    fn insert_from(&mut self, entity: Entity, input: I) -> EntityCommands;
}

impl<I, O> SpawnExtensions<I, O> for Commands<'_, '_>
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

    fn deferred_spawn_from(&mut self, input: I) -> &mut Self {
        self.add(DeferredSpawnSystemCommand::new(input));
        self
    }

    fn insert_from(&mut self, entity: Entity, input: I) -> EntityCommands {
        self.add(SpawnSystemCommand::new(entity, input));
        self.entity(entity)
    }
}
