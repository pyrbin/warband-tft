#[cfg(feature = "dev")]
mod dev;
mod in_game;
mod loading;
mod prelude;
mod util;
mod version;
use prelude::*;
pub use version::*;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash, Reflect)]
pub enum AppState {
    #[default]
    Loading,
    InGame,
}

pub fn plugin(app: &mut App) {
    app_register_types!(AppState);
    app.init_state::<AppState>().add_plugins((
        loading::plugin,
        in_game::plugin,
        #[cfg(feature = "dev")]
        dev::plugin,
    ));
}
