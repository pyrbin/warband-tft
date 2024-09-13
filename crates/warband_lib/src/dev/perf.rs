use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParam},
    render::renderer::RenderAdapterInfo,
};
use iyes_perf_ui::{
    entries::{PerfUiFramerateEntries, PerfUiSystemEntries},
    entry::PerfUiEntry,
    prelude::*,
};

use crate::{assets::FontAssets, prelude::*, AppState};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(iyes_perf_ui::PerfUiPlugin);
    app.add_perf_ui_simple_entry::<PerfUiEntryRenderAdapter>();
    app.add_systems(OnExit(AppState::Loading), perf_ui);
}

#[derive(Component, Default)]
pub struct PerfUiEntryRenderAdapter {
    pub sort_key: i32,
}

impl PerfUiEntry for PerfUiEntryRenderAdapter {
    type Value = String;
    type SystemParam = SRes<RenderAdapterInfo>;

    fn label(&self) -> &str {
        "GPU Adapter"
    }

    fn sort_key(&self) -> i32 {
        self.sort_key
    }

    fn value_color(&self, _value: &Self::Value) -> Option<Color> {
        Some(YELLOW.into())
    }

    fn update_value(
        &self,
        render_adapter_info: &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        let render_adapter_info_name = render_adapter_info.name.clone();
        Some(render_adapter_info_name.to_string())
    }

    fn format_value(&self, value: &Self::Value) -> String {
        value.to_string()
    }
}

mod sort_keys {
    pub const RENDER_ADAPTER: i32 = 1000;
    pub const WINDOW_RESOLUTION: i32 = 1001;
}

fn perf_ui(mut commands: Commands, assets: Res<FontAssets>) {
    commands.spawn((
        Name::ui("perf"),
        PerfUiRoot {
            background_color: Color::BLACK.with_alpha(0.8),
            font_label: assets.commit_mono_700.clone(),
            font_value: assets.commit_mono_400.clone(),
            font_highlight: assets.commit_mono_700.clone(),
            ..PerfUiRoot::default()
        },
        (
            PerfUiFramerateEntries::default(),
            PerfUiSystemEntries::default(),
            PerfUiEntryRenderAdapter {
                sort_key: sort_keys::RENDER_ADAPTER,
            },
            PerfUiEntryWindowResolution {
                label: "Window Resolution".into(),
                sort_key: sort_keys::WINDOW_RESOLUTION,
                ..default()
            },
        ),
    ));
}
