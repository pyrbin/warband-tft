use crate::prelude::*;

pub mod motor;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default());
    app.add_plugins(motor::plugin);
}

#[derive(PhysicsLayer)]
pub(crate) enum Layer {
    Units,
    Terrain,
    Projectile,
}
