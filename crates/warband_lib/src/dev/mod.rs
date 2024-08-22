use crate::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_editor_pls::EditorPlugin;

pub(super) fn plugin(app: &mut App) {
    {
        app.add_plugins(EditorPlugin::new())
            .insert_resource(default_editor_controls())
            .add_plugins((
                FrameTimeDiagnosticsPlugin,
                LogDiagnosticsPlugin::filtered(vec![]),
                PhysicsDebugPlugin::default(),
            ))
            .insert_gizmo_config(
                PhysicsGizmos {
                    aabb_color: Some(Color::WHITE),
                    ..default()
                },
                GizmoConfig {
                    enabled: false,
                    ..default()
                },
            );
    }
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
