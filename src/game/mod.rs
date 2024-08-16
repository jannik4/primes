mod instanced;

use crate::{assets::GameAssets, camera::GameCameraBundle, AppState, Args};
use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
        texture::BevyDefault,
        view::NoFrustumCulling,
    },
};
use bevy_headless_render::{
    components::{HeadlessRenderBundle, HeadlessRenderDestination},
    render_assets::HeadlessRenderSource,
};
use instanced::InstanceMaterialData;
use std::{fs, time::Duration};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Setup and cleanup
        app.add_systems(OnEnter(AppState::Game), setup);
        app.add_systems(OnExit(AppState::Game), cleanup);

        app.add_systems(
            Update,
            (game_time, zoom)
                .run_if(mode_is_run)
                .run_if(in_state(AppState::Game)),
        );
        app.add_systems(
            Update,
            save_screenshot
                .run_if(mode_is_screenshot)
                .run_if(in_state(AppState::Game)),
        );

        app.add_plugins(instanced::InstancedPlugin);
    }
}

fn mode_is_run(args: Res<Args>) -> bool {
    matches!(*args, Args::Run)
}

fn mode_is_screenshot(args: Res<Args>) -> bool {
    matches!(*args, Args::Screenshot { .. })
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

impl Zoom {
    fn scale(&self) -> f32 {
        1.0 / f32::powf(2.0, self.current)
    }
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

    projection.scale = zoom.scale();
}

fn save_screenshot(
    instances: Query<&InstanceMaterialData>,
    mut wait_one_frame: Local<bool>,
    destination: Query<&HeadlessRenderDestination>,
    args: Res<Args>,
    mut app_exit: EventWriter<AppExit>,
) {
    let Args::Screenshot {
        width,
        height,
        game_time,
        game_zoom_exp,
    } = &*args
    else {
        return;
    };

    if instances.iter().any(|instance| !instance.has_rendered()) {
        return;
    }
    if !*wait_one_frame {
        *wait_one_frame = true;
        return;
    }

    let Ok(destination) = destination.get_single() else {
        return;
    };
    let image = destination.0.lock().unwrap();
    if image.data.len() != 4 * (*width as usize * *height as usize) {
        return;
    }

    let image = match image.clone().try_into_dynamic() {
        Ok(image) => image.to_rgba8(),
        Err(e) => panic!("Failed to create image buffer {e:?}"),
    };
    let image_path = format!(
        "./screenshots/primes_{}x{}_{}_{}.png",
        width,
        height,
        game_time.as_millis(),
        game_zoom_exp,
    );

    fs::create_dir_all("./screenshots").unwrap();
    if let Err(e) = image.save(image_path) {
        panic!("Failed to save image: {}", e);
    };

    app_exit.send(AppExit::Success);
}

fn setup(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut images: ResMut<Assets<Image>>,
    headless_render_sources: Option<ResMut<Assets<HeadlessRenderSource>>>,
    args: Res<Args>,
) {
    match &*args {
        Args::Run => {
            commands.spawn((GameCameraBundle::default(), StateScoped(AppState::Game)));
            commands.init_resource::<GameTime>();
            commands.init_resource::<Zoom>();
        }
        Args::Screenshot {
            width,
            height,
            game_time,
            game_zoom_exp,
        } => {
            let game_time = GameTime {
                elapsed: *game_time,
                speed_current: 1.0,
                speed_target: 1.0,
            };
            let zoom = Zoom {
                current: *game_zoom_exp as f32,
                target: *game_zoom_exp as f32,
            };

            let mut image = Image::new_fill(
                Extent3d {
                    width: *width,
                    height: *height,
                    depth_or_array_layers: 1,
                },
                TextureDimension::D2,
                &[0; 4],
                TextureFormat::bevy_default(),
                RenderAssetUsages::default(),
            );
            image.texture_descriptor.usage |= TextureUsages::COPY_SRC
                | TextureUsages::RENDER_ATTACHMENT
                | TextureUsages::TEXTURE_BINDING;
            let image_handle = images.add(image);

            let mut game_camera_bundle = GameCameraBundle::default();
            game_camera_bundle.camera.camera.target = image_handle.clone().into();
            game_camera_bundle.camera.projection.scale = zoom.scale();

            commands.spawn((
                game_camera_bundle,
                HeadlessRenderBundle {
                    source: headless_render_sources
                        .unwrap()
                        .add(HeadlessRenderSource(image_handle)),
                    dest: HeadlessRenderDestination::default(),
                },
                StateScoped(AppState::Game),
            ));

            commands.insert_resource(game_time);
            commands.insert_resource(zoom);
        }
    }

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
