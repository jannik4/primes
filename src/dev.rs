use bevy::prelude::*;
use iyes_perf_ui::{entries::PerfUiBundle, prelude::*, PerfUiSet};

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            bevy::diagnostic::FrameTimeDiagnosticsPlugin,
            bevy::diagnostic::EntityCountDiagnosticsPlugin,
            iyes_perf_ui::PerfUiPlugin,
        ));
        app.add_systems(Update, toggle.before(PerfUiSet::Setup));
    }
}

fn toggle(
    mut commands: Commands,
    query: Query<Entity, With<PerfUiRoot>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::F12) {
        match query.get_single() {
            Ok(e) => commands.entity(e).despawn_recursive(),
            Err(_) => {
                commands.spawn(PerfUiBundle {
                    root: PerfUiRoot {
                        position: PerfUiPosition::TopLeft,
                        ..default()
                    },
                    ..default()
                });
            }
        }
    }
}
