use crate::{camera::GameCameraBundle, primes::Primes, AppState};
use bevy::{log, prelude::*};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        // Setup and cleanup
        app.add_systems(OnEnter(AppState::Loading), setup);
        app.add_systems(OnExit(AppState::Loading), cleanup);
    }
}

fn setup(mut commands: Commands, mut next_state: ResMut<NextState<AppState>>) {
    commands.spawn((GameCameraBundle::default(), StateScoped(AppState::Loading)));

    let primes = Primes::build(5_000_000);
    log::info!("Found {} primes", primes.primes().count());
    commands.insert_resource(primes);

    next_state.set(AppState::Game);
}

fn cleanup(mut _commands: Commands) {}
