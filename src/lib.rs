#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod assets;
mod camera;
mod dev;
mod full_screen;
mod game;
mod loading;
mod primes;
mod splash_screen;

use bevy::prelude::*;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};

pub fn build_app() -> App {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                #[cfg(target_arch = "wasm32")]
                meta_check: bevy::asset::AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    present_mode: bevy::window::PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            }),
    )
    .insert_resource(ClearColor(Color::srgb(0.02, 0.02, 0.02)));

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
        loading::LoadingPlugin,
        game::GamePlugin,
        dev::DevPlugin,
    ));

    app
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    SplashScreen,
    Loading,
    Game,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AssetsState {
    #[default]
    Loading,
    Loaded,
    Error,
}
