use bevy::prelude::*;
use iyes_perf_ui::{entries::PerfUiBundle, prelude::*};

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            bevy::diagnostic::FrameTimeDiagnosticsPlugin,
            bevy::diagnostic::EntityCountDiagnosticsPlugin,
            iyes_perf_ui::PerfUiPlugin,
        ));
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(PerfUiBundle {
        root: PerfUiRoot {
            position: PerfUiPosition::TopLeft,
            ..default()
        },
        ..default()
    });
}
