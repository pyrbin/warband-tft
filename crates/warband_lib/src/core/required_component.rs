use crate::prelude::*;

/// A system that make's sure component [`U`] is present on all entities that have component [`T`].
/// If [`T`] is removed from an entity, [`U`] is also removed.
pub(crate) fn required_component<T: Component, U: Component + Default>(
    commands: ParallelCommands,
    without: Query<Entity, (With<T>, Without<U>)>,
    mut removed: RemovedComponents<T>,
) {
    without.par_iter().for_each(|entity| {
        commands.command_scope(|mut c| {
            c.entity(entity).insert(U::default());
        })
    });

    for entity in &mut removed.read() {
        commands.command_scope(|mut c| {
            if let Some(mut commands) = c.get_entity(entity) {
                commands.remove::<U>();
            }
        });
    }
}
