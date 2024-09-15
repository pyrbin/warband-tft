use crate::{assets::FontAssets, prelude::*, AppState};
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
            enabled: true,
            ..default()
        },
    )
    .insert_resource(default_editor_controls())
    .insert_resource(DebugPickingMode::Normal);

    app.add_plugins((perf::plugin, console::plugin));

    app.add_systems(OnExit(AppState::Loading), semver_ui);
    app.add_systems(
        Update,
        (
            crate::board::location::gizmos,
            crate::board::gizmos,
            crate::board::footprint::gizmos,
            crate::navigation::path::gizmos,
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

#[derive(Component)]
struct SemverUi;

fn semver_ui(mut commands: Commands, assets: Res<FontAssets>) {
    commands
        .spawn((
            Name::ui("semver"),
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(9.0),
                    right: Val::Px(16.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::BLACK.with_alpha(0.8)),
                ..default()
            },
            SemverUi,
        ))
        .with_children(|builder| {
            builder.spawn((TextBundle::from_sections([TextSection::new(
                crate::version(),
                TextStyle {
                    font: assets.commit_mono_400.clone(),
                    font_size: 16.0,
                    color: Color::WHITE,
                },
            )]),));
        });
}
