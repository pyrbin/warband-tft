use bevy::ecs::{system::RunSystemOnce, world::Command};

use super::stats;
use crate::{assets::FontAssets, player::camera::MainCamera, prelude::*, AppState};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<DamageEvent>()
        .add_event::<DamageNumberEvent>();

    app.add_systems(
        Update,
        (damage_event, damage_number_event).run_if(in_state(AppState::InGame)),
    );
}

#[derive(Event)]
pub struct DamageEvent {
    pub damage: f32,
    pub target: Entity,
    pub source: Entity,
}

fn damage_event(
    mut damage_event: EventReader<DamageEvent>,
    mut damage_number_event: EventWriter<DamageNumberEvent>,
    mut with_health: Query<Pool<stats::Health>>,
) {
    for &DamageEvent {
        damage,
        target,
        source: _,
    } in damage_event.read()
    {
        let mut health = or_continue!(with_health.get_mut(target));
        health -= damage;
        damage_number_event.send(DamageNumberEvent { damage, target });
    }
}

#[derive(Debug, Event)]
pub struct DamageNumberEvent {
    pub damage: f32,
    pub target: Entity,
}

fn damage_number_event(
    mut commands: Commands,
    mut damage: EventReader<DamageNumberEvent>,
    spatial_query: Query<&Transform>,
) {
    if damage.is_empty() {
        return;
    }

    for &DamageNumberEvent { damage, target } in damage.read() {
        let transform = or_continue!(spatial_query.get(target));
        commands.add(SpawnDamageNumber {
            number: damage,
            transform: *transform,
        })
    }
}

#[derive(Debug)]
pub struct SpawnDamageNumber {
    pub number: f32,
    pub transform: Transform,
}

impl Command for SpawnDamageNumber {
    fn apply(self, world: &mut World) {
        world.run_system_once_with(self, spawn_damage_number);
    }
}

fn spawn_damage_number(
    args: In<SpawnDamageNumber>,
    commands: Commands,
    fonts: Res<FontAssets>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = or_return!(camera.iter().next());

    let number = args.number;
    let position_ndc = camera
        .world_to_ndc(camera_transform, args.transform.translation + Vec3::Y * 1.)
        .unwrap_or(Vec3::ZERO);

    // commands.spawn((
    //     Name::ui(format!("damage number {number:?}")),
    //     TextBundle {
    //         text: Text::from_section(
    //             number.to_string(),
    //             TextStyle {
    //                 font: fonts.ia_writer_quattro.clone(),
    //                 font_size: 24.0,
    //                 color: WHITE.into(),
    //                 ..Default::default()
    //             },
    //         ),
    //         style: Style {
    //             position_type: PositionType::Absolute,
    //             left: Val::Percent(50. + 50. * position_ndc.x),
    //             top: Val::Percent(50. - 50. * position_ndc.y),
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     },
    //     Despawn::Delay(0.5),
    // ));
}
