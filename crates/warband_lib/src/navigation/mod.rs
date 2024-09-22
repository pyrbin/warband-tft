use crate::{prelude::*, AppState};

pub mod agent;
pub mod path;

#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NavigationSystems {
    Maintain,
    DetectChanges,
    Pathfinding,
}

#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AgentSystems {
    Maintain,
    TargetReached,
    Movement,
}

pub(super) fn plugin(app: &mut App) {
    app_register_types!(
        agent::Agent,
        agent::TargetReached,
        agent::ArrivalRange,
        path::Path,
        path::Target,
        path::TargetLocation
    );

    // #FB_TODO: run in a slow schedule, like 16fps
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

    app.configure_sets(
        PostUpdate,
        (
            AgentSystems::Maintain,
            AgentSystems::TargetReached,
            AgentSystems::Movement,
        )
            .chain()
            .run_if(in_state(AppState::InGame)),
    );

    app.add_systems(
        FixedUpdate,
        (
            (
                required_component::<path::Target, path::TargetLocation>,
                path::target_location,
            )
                .in_set(NavigationSystems::Maintain),
            (path::on_changed, path::on_target_changed).in_set(NavigationSystems::DetectChanges),
            (path::compute, path::poll).in_set(NavigationSystems::Pathfinding),
        ),
    );

    app.add_systems(
        PostUpdate,
        (
            required_component::<agent::Agent, agent::ArrivalRange>.in_set(AgentSystems::Maintain),
            agent::waypoint.in_set(AgentSystems::TargetReached),
            agent::moving.in_set(AgentSystems::Movement),
        ),
    );
}
