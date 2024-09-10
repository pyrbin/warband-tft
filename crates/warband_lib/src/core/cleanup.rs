use bevy::ecs::intern::Interned;

use crate::prelude::*;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Cleanup<T: ScheduleLabel>(pub T);

pub(super) fn plugin<T: ScheduleLabel>(schedule: T) -> CleanupPlugin<T> {
    CleanupPlugin {
        schedule: schedule.intern(),
        _marker: default(),
    }
}

pub(super) struct CleanupPlugin<T: ScheduleLabel> {
    schedule: Interned<dyn ScheduleLabel>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: ScheduleLabel> Plugin for CleanupPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(self.schedule, cleanup::<T>);
    }
}

fn cleanup<T: ScheduleLabel>(
    commands: ParallelCommands,
    entities: Query<Entity, With<Cleanup<T>>>,
) {
    entities.par_iter().for_each(|e| {
        commands.command_scope(|mut c| {
            c.entity(e).despawn_recursive();
        })
    });
}
