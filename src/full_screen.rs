use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowMode},
};

pub struct FullScreenPlugin;

impl Plugin for FullScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, full_screen);
    }
}

fn full_screen(
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::F11) {
        let mut window = primary_window.single_mut();
        window.mode = match window.mode {
            WindowMode::Windowed => WindowMode::BorderlessFullscreen,
            _ => WindowMode::Windowed,
        };
    }
}
