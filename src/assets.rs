use crate::{primes::Primes, AssetsState};
use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    ecs::system::RunSystemOnce,
    prelude::*,
    sprite::Mesh2dHandle,
};
use bevy_asset_loader::prelude::*;

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<PrimesAsset>()
            .register_asset_loader(PrimesAssetLoader);

        app.configure_loading_state(
            LoadingStateConfig::new(AssetsState::Loading)
                .load_collection::<GameAssetsCollection>()
                .init_resource::<GameAssets>(),
        );
    }
}

#[derive(Debug, Clone, Asset, TypePath)]
pub struct PrimesAsset(pub Primes);

#[derive(AssetCollection, Resource)]
struct GameAssetsCollection {
    #[asset(path = "primes.bin")]
    primes: Handle<PrimesAsset>,
}

#[derive(Debug, Resource)]
pub struct GameAssets {
    pub circle: Mesh2dHandle,
    pub primes: Primes,
}

impl FromWorld for GameAssets {
    fn from_world(world: &mut World) -> Self {
        world.run_system_once(load_assets)
    }
}

fn load_assets(
    mut meshes: ResMut<Assets<Mesh>>,
    prime_assets: Res<Assets<PrimesAsset>>,
    collection: Res<GameAssetsCollection>,
) -> GameAssets {
    GameAssets {
        circle: meshes.add(RegularPolygon::new(1.0, 16)).into(),
        primes: prime_assets.get(&collection.primes).unwrap().0.clone(),
    }
}

pub struct PrimesAssetLoader;

impl AssetLoader for PrimesAssetLoader {
    type Asset = PrimesAsset;
    type Settings = ();
    type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a (),
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).await?;
        let primes = Primes::from_unchecked(bytemuck::try_cast_slice(&buf)?.to_vec());

        Ok(PrimesAsset(primes))
    }

    fn extensions(&self) -> &[&str] {
        &["bin"]
    }
}
