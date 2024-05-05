use bevy::{
    app::{Plugin, Startup},
    asset::{AssetServer, Handle},
    ecs::system::{Res, ResMut, Resource},
    render::texture::Image,
};

#[derive(Debug, Resource, Default)]
pub struct SceneAsset {
    pub player: Handle<Image>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<SceneAsset>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAsset>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAsset {
        player: asset_server.load("player.png"),
    }
}
