use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default());
}

#[derive(PhysicsLayer)]
pub(crate) enum CollisionLayer {
    Player,
    Units,
    Terrain,
    Sensor,
}
