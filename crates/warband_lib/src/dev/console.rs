pub use bevy_dev_console::prelude::custom_log_layer;
use bevy_dev_console::{builtin_parser::Environment, prelude::*, register};

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(DevConsolePlugin);
    app.insert_non_send_resource(environment());
}

fn environment() -> Environment {
    register!(&mut environment => {
        // fn command
    });

    Environment::default()
}
