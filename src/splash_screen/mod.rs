use crate::{camera::GameCameraBundle, AppState, AssetsState};
use bevy::prelude::*;

pub struct SplashScreenPlugin;

impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        // Setup and cleanup
        app.add_systems(OnEnter(AppState::SplashScreen), setup);
        app.add_systems(OnExit(AppState::SplashScreen), cleanup);

        // Update
        app.add_systems(
            Update,
            splash_screen.run_if(in_state(AppState::SplashScreen)),
        );
    }
}

#[derive(Debug, Resource)]
struct SplashScreen {
    timer: Timer,
    clicked: bool,
}

impl Default for SplashScreen {
    fn default() -> Self {
        Self {
            // TODO: Non-zero duration
            timer: Timer::from_seconds(0.0, TimerMode::Once),
            clicked: false,
        }
    }
}

fn splash_screen(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    assets_state: Res<State<AssetsState>>,
    mut splash_screen: ResMut<SplashScreen>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    splash_screen.timer.tick(time.delta());
    splash_screen.clicked |=
        keyboard_input.just_pressed(KeyCode::Space) || mouse_input.just_pressed(MouseButton::Left);

    // TODO: Handle AssetsState::Error

    if **assets_state == AssetsState::Loaded
        && (splash_screen.timer.finished() || splash_screen.clicked)
    {
        next_state.set(AppState::Loading);
    }
}

fn setup(mut commands: Commands) {
    commands.init_resource::<SplashScreen>();
    commands.spawn((
        GameCameraBundle::default(),
        StateScoped(AppState::SplashScreen),
    ));
}

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<SplashScreen>();
}
