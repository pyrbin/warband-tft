use bevy::{
    core_pipeline::prepass::{DepthPrepass, NormalPrepass},
    input::mouse::MouseWheel,
};

use crate::{core::name::NameTags, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app_register_types!(MainCamera, UiCamera);

    app.add_systems(Startup, setup);
    app.add_systems(Update, controls);
}

#[derive(Component, Default, Reflect)]
pub struct MainCamera;

#[derive(Component, Default, Reflect)]
pub struct UiCamera;

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn((
        Name::camera("main"),
        MainCamera,
        Camera3dBundle {
            camera: Camera {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            camera_3d: Camera3d::default(),
            projection: orthographic_fixed_vertical(1.0, 30.0, -100.0, 200.0),
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        DepthPrepass,
        NormalPrepass,
        camera_driver::RigTransform::default(),
        camera_driver::Zoom::with_zoom(30.0),
        camera_driver::YawPitch::with_yaw_pitch(25.0, -45.0),
        camera_driver::Smoothing::default()
            .with_position(0.0)
            .with_rotation(2.0)
            .with_zoom(0.0),
    ));

    // commands.spawn((
    //     Name::new("ui camera"),
    //     UiCamera,
    //     Camera2dBundle { ..default() },
    // ));
}

pub fn orthographic_fixed_vertical(height: f32, scale: f32, near: f32, far: f32) -> Projection {
    OrthographicProjection {
        scale,
        scaling_mode: bevy::render::camera::ScalingMode::FixedVertical(height),
        near,
        far,
        ..default()
    }
    .into()
}

fn controls(
    mut camera: Query<(&mut camera_driver::YawPitch, &mut camera_driver::Zoom), With<MainCamera>>,
    mut scroll: EventReader<MouseWheel>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (mut yaw_pitch, mut zoom) in &mut camera {
        let yaw_input = if input.just_pressed(KeyCode::ArrowLeft) {
            1.0
        } else {
            0.0
        } - if input.just_pressed(KeyCode::ArrowRight) {
            1.0
        } else {
            0.0
        };

        yaw_pitch.rotate_yaw(yaw_input * 90.0);

        let pitch_input = if input.just_pressed(KeyCode::ArrowDown) {
            1.0
        } else {
            0.0
        } - if input.just_pressed(KeyCode::ArrowUp) {
            1.0
        } else {
            0.0
        };

        yaw_pitch.rotate_pitch(pitch_input * 5.0);

        if input.just_pressed(KeyCode::KeyR) {
            yaw_pitch.pitch = -35.0;
            yaw_pitch.yaw = 180.0;
        }

        const MAX_ZOOM: f32 = 100.0;
        const MIN_ZOOM: f32 = 1.0;

        for event in scroll.read() {
            let zoom_scale = zoom.zoom();
            zoom.set_zoom((zoom_scale - event.y).clamp(MIN_ZOOM, MAX_ZOOM));
        }
    }
}
