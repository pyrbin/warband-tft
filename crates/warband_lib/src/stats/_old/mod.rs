use crate::prelude::*;

pub mod modifier;
pub mod pool;
pub mod stat;
pub mod v2;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum StatSystem {
    Dirty,
    Reset,
    ModifierFlat,
    ModifierMult,
    Cleanup,
}

pub(super) fn plugin(app: &mut App) {
    app.configure_sets(
        PostUpdate,
        (
            StatSystem::Dirty,
            StatSystem::Reset,
            StatSystem::ModifierFlat,
            StatSystem::ModifierMult,
            StatSystem::Cleanup,
        )
            .chain(),
    );

    app.add_systems(
        PostUpdate,
        propagate_previous_changed::<modifier::Modifies>.in_set(StatSystem::Cleanup),
    );
}
