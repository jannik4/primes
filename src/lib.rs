#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod assets;
mod camera;
mod dev;
mod full_screen;
mod game;
mod primes;
mod splash_screen;

use bevy::{
    app::{RunMode, ScheduleRunnerPlugin},
    prelude::*,
    winit::WinitPlugin,
};
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use std::time::Duration;

#[derive(Debug, Resource)]
pub enum Args {
    Run,
    Screenshot {
        width: u32,
        height: u32,
        game_time: Duration,
        game_zoom_exp: i32,
    },
}

impl Args {
    #[cfg(target_arch = "wasm32")]
    pub fn from_env() -> Self {
        Self::Run
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_env() -> Self {
        let mut args = pico_args::Arguments::from_env();
        match args.subcommand().unwrap().as_deref() {
            Some("run") | None => Self::Run,
            Some("screenshot") => Self::Screenshot {
                width: args.value_from_str("--width").unwrap_or(1920),
                height: args.value_from_str("--height").unwrap_or(1080),
                game_time: Duration::from_millis(args.value_from_str("--time").unwrap_or(0)),
                game_zoom_exp: args.value_from_str("--zoom").unwrap_or(0),
            },
            _ => panic!("Invalid subcommand"),
        }
    }
}

pub fn build_app(args: Args) -> App {
    let mut app = App::new();

    let default_plugins = DefaultPlugins.set(AssetPlugin {
        #[cfg(target_arch = "wasm32")]
        meta_check: bevy::asset::AssetMetaCheck::Never,
        ..default()
    });
    let default_plugins = match &args {
        Args::Run => default_plugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }),
        Args::Screenshot { .. } => default_plugins.disable::<WinitPlugin>(),
    };

    app.add_plugins(default_plugins)
        .insert_resource(ClearColor(Color::srgb(0.02, 0.02, 0.02)));

    match &args {
        Args::Run => (),
        Args::Screenshot { .. } => {
            app.add_plugins((
                ScheduleRunnerPlugin {
                    run_mode: RunMode::Loop { wait: None },
                },
                bevy_headless_render::HeadlessRenderPlugin,
            ));
        }
    }

    app.insert_resource(args);

    app.init_state::<AppState>()
        .enable_state_scoped_entities::<AppState>()
        .init_state::<AssetsState>();

    app.add_loading_state({
        LoadingState::new(AssetsState::Loading)
            .continue_to_state(AssetsState::Loaded)
            .on_failure_continue_to_state(AssetsState::Error)
    });

    app.add_plugins((
        assets::GameAssetsPlugin,
        full_screen::FullScreenPlugin,
        splash_screen::SplashScreenPlugin,
        game::GamePlugin,
        dev::DevPlugin,
    ));

    app
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    SplashScreen,
    Game,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AssetsState {
    #[default]
    Loading,
    Loaded,
    Error,
}
