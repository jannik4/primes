use crate::AssetsState;
use bevy::{ecs::system::RunSystemOnce, prelude::*, sprite::Mesh2dHandle};
use bevy_asset_loader::prelude::*;

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_loading_state(
            LoadingStateConfig::new(AssetsState::Loading)
                .load_collection::<AudioAssets>()
                .init_resource::<GameAssets>(),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {}

#[derive(Debug, Resource)]
pub struct GameAssets {
    pub circle: Mesh2dHandle,
    pub circle_material: Handle<ColorMaterial>,

    pub star_mesh: Mesh2dHandle,
    pub planet_mesh: Mesh2dHandle,
    pub space_ship_mesh: Mesh2dHandle,
    pub bullet_mesh: Mesh2dHandle,
    pub explosion_mesh: Mesh2dHandle,
    pub background_mesh: Mesh2dHandle,
    pub health_bar_mesh: Mesh2dHandle,

    pub enemy_space_ship_material: Handle<ColorMaterial>,
    pub enemy_bullet_material: Handle<ColorMaterial>,

    pub player_space_ship_material: Handle<ColorMaterial>,
    pub player_bullet_material: Handle<ColorMaterial>,

    pub home_planet_material: Handle<ColorMaterial>,
    pub background_material: Handle<ColorMaterial>,
    pub health_bar_material_gray: Handle<ColorMaterial>,
    pub health_bar_material_green: Handle<ColorMaterial>,
}

impl FromWorld for GameAssets {
    fn from_world(world: &mut World) -> Self {
        world.run_system_once(load_assets)
    }
}

fn load_assets(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> GameAssets {
    GameAssets {
        circle: meshes.add(RegularPolygon::new(1.2, 6)).into(),
        circle_material: materials.add(Color::srgb(2.0, 1.5, 1.5)),

        star_mesh: meshes.add(Circle::new(16.0)).into(),
        planet_mesh: meshes.add(Circle::new(8.0)).into(),
        space_ship_mesh: meshes.add(Circle::new(8.0)).into(),
        bullet_mesh: meshes.add(Rectangle::new(6.0, 2.0)).into(),
        explosion_mesh: meshes.add(Rectangle::new(3.0, 1.5)).into(),
        background_mesh: meshes.add(Circle::new(8.0)).into(),
        health_bar_mesh: meshes.add(Capsule2d::new(4.0, 200.0)).into(),

        enemy_space_ship_material: materials.add(Color::srgb(1.4, 0.6, 0.6)),
        enemy_bullet_material: materials.add(Color::srgb(2.0, 0.0, 0.0)),

        player_space_ship_material: materials.add(Color::srgb(0.6, 0.6, 1.4)),
        player_bullet_material: materials.add(Color::srgb(0.0, 0.0, 2.0)),

        home_planet_material: materials.add(Color::srgb(0.2, 0.5, 2.0)),
        background_material: materials.add(Color::srgb(6.0, 6.0, 6.0)),
        health_bar_material_gray: materials.add(Color::srgb(0.5, 0.5, 0.5)),
        health_bar_material_green: materials.add(Color::srgb(0.0, 1.5, 0.0)),
    }
}
