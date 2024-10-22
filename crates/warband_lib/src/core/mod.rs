use crate::{prelude::*, AppState};

pub mod camera_driver;
pub mod cleanup;
pub mod despawn;
pub mod name;
pub mod previous;
pub mod required_component;
pub mod spawn;

pub(super) fn plugin(app: &mut App) {
    app_register_types!(Owner);

    app.add_plugins(bevy_mod_picking::DefaultPickingPlugins);
    app.add_plugins((despawn::plugin, camera_driver::plugin(Last)));

    // #FB_TODO: replace with a derive macro?
    app.add_plugins(cleanup::plugin(OnEnter(AppState::InGame)));
    app.add_plugins(cleanup::plugin(OnExit(AppState::InGame)));

    // app.add_plugins(cleanup::plugin(OnEnter(AppState::Loading)));
    // app.add_plugins(cleanup::plugin(OnExit(AppState::Loading)));
}

/// Component to mark own
#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, From)]
pub(crate) struct Owner(pub Entity);

/// Generic component to mark component [`T`] as dirty.
#[derive(Component, Default, Deref, DerefMut, From, Reflect)]
#[component(storage = "SparseSet")]
pub(crate) struct Dirty<T: Component>(#[reflect(ignore)] pub PhantomData<T>);

/// Generic component to mark component [`T`] as disabled.
#[derive(Component, Default, Deref, DerefMut, From, Reflect)]
#[component(storage = "SparseSet")]
pub(crate) struct Disabled<T: Component>(#[reflect(ignore)] pub PhantomData<T>);

#[derive(Component, Clone, Copy, Debug, Reflect)]
#[reflect(Component)]
pub(crate) enum Target {
    Point(Vec3),
    Entity(Entity),
}

impl Target {
    #[inline]
    pub fn global_translation(&self, transforms: &Query<&GlobalTransform>) -> Option<Vec3> {
        match self {
            Self::Point(point) => Some(*point),
            Self::Entity(entity) => global_translation(transforms, *entity).ok(),
        }
    }

    #[inline]
    pub fn translation(&self, transforms: &Query<&Transform>) -> Option<Vec3> {
        match self {
            Self::Point(point) => Some(*point),
            Self::Entity(entity) => translation(transforms, *entity).ok(),
        }
    }
}

impl Default for Target {
    fn default() -> Self {
        Self::Point(Vec3::ZERO)
    }
}
