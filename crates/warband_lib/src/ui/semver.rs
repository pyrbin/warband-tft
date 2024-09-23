use crate::{assets::FontAssets, prelude::*};

#[derive(Component)]
struct SemverUi;

pub(super) fn ui(mut commands: Commands, assets: Res<FontAssets>) {
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
