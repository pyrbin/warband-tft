use crate::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub(crate) enum DespawnSystems {
    Timer,
    Despawn,
}

pub(super) fn plugin(app: &mut App) {
    app_register_types!(Despawn, DespawnInProgress);

    app.configure_sets(
        Last,
        (DespawnSystems::Timer, DespawnSystems::Despawn).chain(),
    );

    app.add_systems(
        Last,
        (
            despawn_timer.in_set(DespawnSystems::Timer),
            apply_deferred,
            despawn.in_set(DespawnSystems::Timer),
        )
            .chain(),
    );
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[component(storage = "SparseSet")]
pub enum Despawn {
    // Despawn immediately.
    #[default]
    Immediate,
    // Despawn after a delay in seconds.
    Delay(f32),
    // Despawn after a delay in frames.
    WaitFrames(u32),
}

#[derive(Component, Default, Reflect)]
pub struct DespawnInProgress;

fn despawn_timer(
    mut commands: Commands,
    mut despawns: Query<(Entity, &mut Despawn)>,
    time: Res<Time>,
) {
    for (entity, mut despawn) in &mut despawns {
        let despawn = match *despawn {
            Despawn::Immediate => true,
            Despawn::Delay(ref mut dur) => {
                *dur -= time.delta_seconds();
                *dur <= 0.0
            }
            Despawn::WaitFrames(ref mut frame) => {
                if *frame == 0 {
                    true
                } else {
                    *frame -= 1;
                    *frame == 0
                }
            }
        };
        if despawn {
            commands
                .entity(entity)
                .remove::<Despawn>()
                .insert(DespawnInProgress);
        }
    }
}

fn despawn(mut commands: Commands, to_despawn: Query<Entity, With<DespawnInProgress>>) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
