use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
    render::camera::ScalingMode,
};

#[derive(Bundle)]
pub struct GameCameraBundle {
    pub camera: Camera2dBundle,
    pub bloom: BloomSettings,
}

impl Default for GameCameraBundle {
    fn default() -> Self {
        GameCameraBundle {
            camera: Camera2dBundle {
                camera: Camera {
                    hdr: true,
                    ..default()
                },
                projection: OrthographicProjection {
                    far: 1000.,
                    near: -1000.,
                    scaling_mode: ScalingMode::FixedVertical(1024.0),
                    ..Default::default()
                },
                tonemapping: Tonemapping::TonyMcMapface,
                ..default()
            },
            bloom: BloomSettings::default(),
        }
    }
}
