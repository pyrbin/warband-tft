use crate::{prelude::*, AppState};

pub mod agent;
pub mod path;

#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PathingSystems {
    Maintain,
    Pathfinding,
}

#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AgentSystems {
    Maintain,
    Pathing,
    Follow,
}

pub(super) fn plugin(app: &mut App) {
    app_register_types!(
        agent::Agent,
        agent::Arrived,
        agent::ArrivalThreshold,
        agent::PathingEvent,
        path::Path,
        path::Target,
        path::TargetLocation
    );

    app.configure_sets(
        FixedUpdate, // TODO: maybe run in a slower schedule?
        (PathingSystems::Maintain, PathingSystems::Pathfinding)
            .chain()
            .run_if(in_state(AppState::InGame)),
    );

    app.configure_sets(
        FixedPostUpdate,
        (
            AgentSystems::Maintain,
            AgentSystems::Pathing,
            AgentSystems::Follow,
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
                (path::on_target_changed, path::on_changed),
            )
                .chain()
                .in_set(PathingSystems::Maintain),
            (path::compute, path::poll).in_set(PathingSystems::Pathfinding),
        ),
    );

    app.observe(agent::on_path_event);

    app.add_systems(
        PostUpdate,
        (
            (
                agent::arrived,
                required_component::<agent::Agent, agent::ArrivalThreshold>,
            )
                .in_set(AgentSystems::Maintain),
            agent::pathing.in_set(AgentSystems::Pathing),
            agent::follow.in_set(AgentSystems::Follow),
        ),
    );
}
