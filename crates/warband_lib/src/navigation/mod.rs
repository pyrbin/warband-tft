use crate::{prelude::*, AppState};

pub mod agent;
pub mod path;

#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NavigationSystems {
    Maintain,
    DetectChanges,
    Pathfinding,
}

pub(super) fn plugin(app: &mut App) {
    app_register_types!(agent::Agent, path::Path, path::Target, path::TargetLocation);

    app.configure_sets(
        FixedUpdate,
        (
            NavigationSystems::Maintain,
            NavigationSystems::DetectChanges,
            NavigationSystems::Pathfinding,
        )
            .chain()
            .run_if(in_state(AppState::InGame)),
    );

    app.add_systems(
        FixedUpdate,
        (
            path::target_location.in_set(NavigationSystems::Maintain),
            (path::on_changed, path::on_target_changed).in_set(NavigationSystems::DetectChanges),
            (path::compute, path::poll).in_set(NavigationSystems::Pathfinding),
        ),
    );
}
