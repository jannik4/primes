use crate::AssetsState;
use bevy::{ecs::system::RunSystemOnce, prelude::*, sprite::Mesh2dHandle};
use bevy_asset_loader::prelude::*;

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_loading_state(
            LoadingStateConfig::new(AssetsState::Loading)
                .load_collection::<GameAssetsCollection>()
                .init_resource::<GameAssets>(),
        );
    }
}

#[derive(AssetCollection, Resource)]
struct GameAssetsCollection {}

#[derive(Debug, Resource)]
pub struct GameAssets {
    pub circle: Mesh2dHandle,
}

impl FromWorld for GameAssets {
    fn from_world(world: &mut World) -> Self {
        world.run_system_once(load_assets)
    }
}

fn load_assets(mut meshes: ResMut<Assets<Mesh>>) -> GameAssets {
    GameAssets {
        circle: meshes.add(RegularPolygon::new(1.0, 16)).into(),
    }
}
