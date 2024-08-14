mod instanced;

use crate::{assets::GameAssets, camera::GameCameraBundle, primes::Primes, AppState};
use bevy::{prelude::*, render::view::NoFrustumCulling, sprite::MaterialMesh2dBundle};
use instanced::InstanceMaterialData;
use std::{f32::consts::TAU, time::Duration};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Setup and cleanup
        app.add_systems(OnEnter(AppState::Game), setup);
        app.add_systems(OnExit(AppState::Game), cleanup);

        app.add_systems(
            Update,
            (game_time, zoom, update)
                .chain()
                .run_if(in_state(AppState::Game)),
        );

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

    game_time.elapsed += time.delta().mul_f64(game_time.speed_current);
}

#[derive(Debug, Resource)]
struct Zoom {
    current: f32,
    target: f32,
}

impl Default for Zoom {
    fn default() -> Self {
        Self {
            current: 1.0,
            target: 1.0,
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
        _ if input.just_pressed(KeyCode::Digit1) => 4.0,
        _ if input.just_pressed(KeyCode::Digit2) => 3.0,
        _ if input.just_pressed(KeyCode::Digit3) => 2.0,
        _ if input.just_pressed(KeyCode::Digit4) => 1.0,
        _ if input.just_pressed(KeyCode::Digit5) => 0.0,
        _ if input.just_pressed(KeyCode::Digit6) => -1.0,
        _ if input.just_pressed(KeyCode::Digit7) => -2.0,
        _ if input.just_pressed(KeyCode::Digit8) => -3.0,
        _ if input.just_pressed(KeyCode::Digit9) => -4.0,
        _ if input.just_pressed(KeyCode::Digit0) => -5.0,
        _ => zoom.target,
    };
    zoom.current = f32::lerp(
        zoom.current,
        zoom.target,
        1.0 - f32::exp(f32::ln(0.95) * 60.0 * time.delta_seconds()),
    );

    projection.scale = 1.0 / f32::powf(2.0, zoom.current);
}

fn update(mut gizmos: Gizmos, primes: Res<Primes>, game_time: Res<GameTime>) {
    const RESOLUTION: usize = 6;
    let circle = std::array::from_fn::<_, { RESOLUTION + 1 }, _>(|i| {
        let angle = i as f32 * TAU / RESOLUTION as f32;
        let (x, y) = angle.sin_cos();
        Vec3::new(x, y, 0.0)
    });

    let elapsed_seconds = game_time.elapsed.as_secs_f32();
    // LinearRgba::from(Color::srgb(red, green, blue));

    // for prime in primes.primes() {
    //     let angle = prime as f32 % TAU;
    //     let position = Vec3::new(
    //         prime as f32 * f32::cos(angle - 0.002 * elapsed_seconds) / 50.0,
    //         prime as f32 * f32::sin(angle - 0.002 * elapsed_seconds) / 50.0,
    //         0.0,
    //     );
    //
    //     let radius = 0.2 + 1.0 * (f32::sin(2.0 * elapsed_seconds + prime as f32 * 0.1) + 1.0) / 2.0;
    //     let color = Color::srgb(
    //         1.5 + 0.5 * (f32::sin(1.0 * elapsed_seconds + prime as f32 * 0.0008) + 1.0) / 2.0,
    //         1.5,
    //         1.5 + (1.0 / prime as f32).powf(0.2),
    //     );
    //     gizmos.linestrip(circle.into_iter().map(|pos| position + radius * pos), color);
    // }
}

fn setup(mut commands: Commands, assets: Res<GameAssets>, primes: Res<Primes>) {
    commands.spawn((GameCameraBundle::default(), StateScoped(AppState::Game)));
    commands.init_resource::<GameTime>();
    commands.init_resource::<Zoom>();

    // let elapsed_seconds_f64 = 0.0;
    // // let elapsed_seconds = 0.0;
    //
    // for prime in primes.primes() {
    //     let position = Vec3::new(
    //         (prime as f64 * f64::cos(prime as f64 - 0.002 * elapsed_seconds_f64) / 50.0) as f32,
    //         (prime as f64 * f64::sin(prime as f64 - 0.002 * elapsed_seconds_f64) / 50.0) as f32,
    //         0.0,
    //     );
    //     // let radius = 0.2 + 1.0 * (f32::sin(2.0 * elapsed_seconds + prime as f32 * 0.1) + 1.0) / 2.0;
    //     // let color = Color::srgb(
    //     //     1.5 + 0.5 * (f32::sin(1.0 * elapsed_seconds + prime as f32 * 0.0008) + 1.0) / 2.0,
    //     //     1.5,
    //     //     1.5 + (1.0 / prime as f32).powf(0.2),
    //     // );
    //     // gizmos.linestrip(circle.into_iter().map(|pos| position + radius * pos), color);
    //
    //     commands.spawn((
    //         MaterialMesh2dBundle {
    //             mesh: assets.circle.clone(),
    //             material: assets.circle_material.clone(),
    //             transform: Transform::from_translation(position),
    //             ..default()
    //         },
    //         StateScoped(AppState::Game),
    //         bevy::render::batching::NoAutomaticBatching,
    //     ));
    // }

    commands.spawn((
        assets.circle2.clone(),
        SpatialBundle::INHERITED_IDENTITY,
        InstanceMaterialData::from_iter(primes.primes()),
        NoFrustumCulling,
    ));
}

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<GameTime>();
    commands.remove_resource::<Zoom>();
}
