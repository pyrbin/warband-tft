use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{config::ConfigureLoadingState, LoadingStateAppExt},
    prelude::LoadingState,
};

use crate::{prelude::*, AppState};

pub fn plugin(app: &mut App) {
    app_register_types!(FontAssets, ModelAssets, ImageAssets);

    app.add_loading_state(
        LoadingState::new(AppState::Loading)
            .load_collection::<FontAssets>()
            .load_collection::<ModelAssets>()
            .load_collection::<ImageAssets>()
            .continue_to_state(AppState::InGame),
    );
}

#[derive(AssetCollection, Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/iAWriterQuattroS-Regular.ttf")]
    pub ia_writer_quattro: Handle<Font>,

    #[asset(path = "fonts/CommitMono-400-Regular.otf")]
    pub commit_mono_400: Handle<Font>,

    #[asset(path = "fonts/CommitMono-700-Regular.otf")]
    pub commit_mono_700: Handle<Font>,
}

#[derive(AssetCollection, Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct ModelAssets {
    #[asset(path = "models/frog.glb#Scene0")]
    pub frog: Handle<Scene>,
}

#[derive(AssetCollection, Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct ImageAssets {
    #[asset(path = "images/bevy.png")]
    pub bevy: Handle<Image>,

    #[asset(path = "images/proto_dark.png")]
    pub proto_dark: Handle<Image>,
}
