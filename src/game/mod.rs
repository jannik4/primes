mod instanced;

use crate::{assets::GameAssets, camera::GameCameraBundle, AppState};
use bevy::{prelude::*, render::view::NoFrustumCulling};
use instanced::InstanceMaterialData;
use std::time::Duration;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Setup and cleanup
        app.add_systems(OnEnter(AppState::Game), setup);
        app.add_systems(OnExit(AppState::Game), cleanup);

        app.add_systems(Update, (game_time, zoom).run_if(in_state(AppState::Game)));

        app.add_plugins(instanced::InstancedPlugin);
    }
}

#[derive(Debug, Resource)]
struct GameTime {
    elapsed: Duration,
    speed_current: f64,
    speed_target: f64,
}

impl Default for GameTime {
    fn default() -> Self {
        Self {
            elapsed: Duration::ZERO,
            speed_current: 1.0,
            speed_target: 1.0,
        }
    }
}

fn game_time(time: Res<Time>, mut game_time: ResMut<GameTime>, input: Res<ButtonInput<KeyCode>>) {
    let game_time = &mut *game_time;

    game_time.speed_target = match () {
        _ if input.pressed(KeyCode::ShiftLeft) => 16.0,
        _ if input.pressed(KeyCode::ControlLeft) => 4.0,
        _ => 1.0,
    };
    game_time.speed_current = f64::lerp(
        game_time.speed_current,
        game_time.speed_target,
        1.0 - f64::exp(f64::ln(0.95) * 60.0 * time.delta_seconds_f64()),
    );

    if input.pressed(KeyCode::KeyR) {
        game_time.elapsed = Duration::ZERO;
    } else {
        game_time.elapsed += time.delta().mul_f64(game_time.speed_current);
    }
}

#[derive(Debug, Resource)]
struct Zoom {
    current: f32,
    target: f32,
}

impl Default for Zoom {
    fn default() -> Self {
        Self {
            current: 0.0,
            target: 0.0,
        }
    }
}

fn zoom(
    time: Res<Time>,
    mut zoom: ResMut<Zoom>,
    input: Res<ButtonInput<KeyCode>>,
    mut camera: Query<&mut OrthographicProjection>,
) {
    let Ok(mut projection) = camera.get_single_mut() else {
        return;
    };

    zoom.target = match () {
        _ if input.just_pressed(KeyCode::Space) => 0.0,
        _ if input.just_pressed(KeyCode::ArrowUp) => f32::min(10.0, zoom.target + 1.0),
        _ if input.just_pressed(KeyCode::ArrowDown) => f32::max(-6.0, zoom.target - 1.0),
        _ => zoom.target,
    };
    zoom.current = f32::lerp(
        zoom.current,
        zoom.target,
        1.0 - f32::exp(f32::ln(0.95) * 60.0 * time.delta_seconds()),
    );

    projection.scale = 1.0 / f32::powf(2.0, zoom.current);
}

fn setup(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((GameCameraBundle::default(), StateScoped(AppState::Game)));
    commands.init_resource::<GameTime>();
    commands.init_resource::<Zoom>();

    commands.spawn((
        assets.circle.clone(),
        SpatialBundle::INHERITED_IDENTITY,
        InstanceMaterialData::from_iter(assets.primes.primes().iter().copied()),
        NoFrustumCulling,
    ));
}

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<GameTime>();
    commands.remove_resource::<Zoom>();
}
