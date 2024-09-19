use modifier::Modifies;

use crate::prelude::*;

pub mod modifier;
pub mod pool;
pub mod stat;

/// # Example
/// ```
/// #[derive(Stat, Default, Component, Reflect, Copy, Clone)]
/// #[clamp(clamp_0_100)]
/// #[round(round_i32)]
/// struct Health {
///     #[stat(value)]
///     value: f32,
/// }
///
/// fn clamp_0_100(value: f32) -> f32 {
///     value.clamp(0.0, 100.0)
/// }
///
/// fn round_i32(value: f32) -> f32 {
///     value.round()
/// }
/// ```

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum StatSystems {
    Dirty,
    Reset,
    Accumulate,
    Compute,
    Cleanup,
}

pub(super) fn plugin(app: &mut App) {
    app_register_types!(modifier::Modifies, Previous<modifier::Modifies>);

    app.configure_sets(
        PostUpdate,
        (
            StatSystems::Dirty,
            StatSystems::Reset,
            StatSystems::Accumulate,
            StatSystems::Compute,
            StatSystems::Cleanup,
        )
            .chain(),
    );

    app.add_systems(
        PostUpdate,
        propagate_previous_changed::<Modifies>.in_set(StatSystems::Dirty),
    );
}
