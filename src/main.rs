mod asset_loader;
mod camera;
mod movement;
mod player;

use asset_loader::AssetLoaderPlugin;
use bevy::{
    app::App,
    pbr::AmbientLight,
    render::{camera::ClearColor, color::Color},
    DefaultPlugins,
};
use camera::CameraPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(255.1, 255.0, 255.0)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.35,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(MovementPlugin)
        .run();
}
