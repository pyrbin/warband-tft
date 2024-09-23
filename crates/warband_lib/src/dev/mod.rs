use crate::prelude::*;
use bevy_editor_pls::EditorPlugin;
use bevy_mod_picking::debug::DebugPickingMode;

mod console;
pub mod gizmos_ext;
mod perf;

// Re-export custom log layer
pub use console::custom_log_layer;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        bevy::diagnostic::FrameTimeDiagnosticsPlugin,
        bevy::diagnostic::EntityCountDiagnosticsPlugin,
        bevy::diagnostic::SystemInformationDiagnosticsPlugin,
        bevy::diagnostic::LogDiagnosticsPlugin::filtered(vec![]),
    ))
    .add_plugins(EditorPlugin::default())
    .add_plugins(PhysicsDebugPlugin::default())
    .insert_gizmo_config(
        PhysicsGizmos {
            aabb_color: Some(Color::WHITE),
            ..default()
        },
        GizmoConfig {
            enabled: false,
            ..default()
        },
    )
    .insert_resource(default_editor_controls())
    .insert_resource(DebugPickingMode::Normal);

    app.add_plugins((perf::plugin, console::plugin));

    app.add_systems(
        Update,
        (
            crate::board::location::gizmos,
            crate::board::gizmos,
            crate::navigation::path::gizmos,
            crate::navigation::agent::gizmos,
        )
            .chain(),
    );
}

fn default_editor_controls() -> bevy_editor_pls::controls::EditorControls {
    use bevy_editor_pls::controls::*;
    let mut editor_controls = EditorControls::default_bindings();
    editor_controls.unbind(Action::PlayPauseEditor);
    editor_controls.insert(
        Action::PlayPauseEditor,
        Binding {
            input: UserInput::Single(Button::Keyboard(KeyCode::KeyQ)),
            conditions: vec![BindingCondition::ListeningForText(false)],
        },
    );
    editor_controls
}
