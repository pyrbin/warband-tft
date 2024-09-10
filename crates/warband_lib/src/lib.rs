#![feature(let_chains)]
#![feature(if_let_guard)]

mod assets;
#[cfg(feature = "dev")]
mod dev;
mod in_game;
mod prelude;
mod util;
mod version;
use prelude::*;
pub use version::*;
mod board;
mod core;
mod player;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash, Reflect)]
pub enum AppState {
    #[default]
    Loading,
    InGame,
}

pub fn plugin(app: &mut App) {
    app_register_types!(AppState);

    app.init_state::<AppState>().add_plugins((
        core::plugin,
        assets::plugin,
        in_game::plugin,
        board::plugin,
        player::plugin,
        #[cfg(feature = "dev")]
        dev::plugin,
    ));
}
