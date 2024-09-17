use modifier::Modifies;

use crate::prelude::*;

pub mod modifier;
pub mod stat;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum StatSystems {
    Dirty,
    Accumulate,
    Compute,
    Cleanup,
}

pub(super) fn plugin(app: &mut App) {
    app_register_types!(modifier::Modifies, Previous<modifier::Modifies>);

    app.configure_sets(
        PostUpdate,
        (
            StatSystems::Dirty,
            StatSystems::Accumulate,
            StatSystems::Compute,
            StatSystems::Cleanup,
        )
            .chain(),
    );

    app.add_systems(
        PostUpdate,
        propagate_previous_changed::<Modifies>.in_set(StatSystems::Dirty),
    );
}
