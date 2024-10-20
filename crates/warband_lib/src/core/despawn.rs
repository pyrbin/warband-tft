use crate::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub(crate) enum DespawnSystems {
    Timer,
    Despawn,
}

pub(super) fn plugin(app: &mut App) {
    app_register_types!(Despawn, Despawning);

    app.configure_sets(
        Last,
        (DespawnSystems::Timer, DespawnSystems::Despawn).chain(),
    );

    app.add_systems(
        Last,
        (
            despawn_timer.chain().in_set(DespawnSystems::Timer),
            despawning.in_set(DespawnSystems::Despawn),
        )
            .chain(),
    );
}

#[derive(Component, Default, Reflect, Clone)]
#[reflect(Component)]
#[component(storage = "SparseSet")]
pub enum Despawn {
    /// Despawn immediately.
    #[default]
    Immediate,
    /// Despawn after a delay in seconds.
    Delay(f32),
    /// Despawn after a delay in frames.
    WaitFrames(u32),
}

#[derive(Component, Default, Reflect, Clone)]
pub struct Despawning;

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
                .insert(Despawning);
        }
    }
}

fn despawning(mut commands: Commands, despawning: Query<Entity, Added<Despawning>>) {
    for entity in &despawning {
        commands.entity(entity).despawn_recursive();
    }
}
