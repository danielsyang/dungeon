use bevy::{
    app::{Plugin, Startup},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::system::Commands,
    transform::components::Transform,
};

const CAMERA_DISTANCE: f32 = 10.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(100.0, CAMERA_DISTANCE, 0.0),
        ..Default::default()
    });
}
