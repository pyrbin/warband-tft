use bevy_vector_shapes::ShapePlugin;

use crate::{prelude::*, AppState};

mod semver;

pub fn plugin(app: &mut App) {
    app.add_plugins(ShapePlugin::default());
    app.add_systems(OnExit(AppState::Loading), semver::ui);
}
