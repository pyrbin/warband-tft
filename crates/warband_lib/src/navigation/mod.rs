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
    Waypoint,
    Pathing,
}

pub(super) fn plugin(app: &mut App) {
    app_register_types!(
        agent::Agent,
        agent::Pathing,
        agent::DestinationReached,
        agent::DestinationRange,
        agent::PathingEvent,
        path::Path,
        path::Destination,
        path::DestinationLocation
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
            AgentSystems::Waypoint,
            AgentSystems::Pathing,
        )
            .chain()
            .run_if(in_state(AppState::InGame)),
    );

    app.add_systems(
        FixedUpdate,
        (
            (
                required_component::<agent::Agent, path::Destination>,
                required_component::<agent::Agent, agent::Pathing>,
                required_component::<path::Destination, path::DestinationLocation>,
                path::target_location,
                (path::on_destination_changed, path::on_occupied_changed),
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
                agent::destination_reached,
                required_component::<agent::Agent, agent::DestinationRange>,
            )
                .in_set(AgentSystems::Maintain),
            agent::waypoint.in_set(AgentSystems::Waypoint),
            agent::pathing.in_set(AgentSystems::Pathing),
        ),
    );
}
