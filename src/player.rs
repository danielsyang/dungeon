use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::{
        component::Component,
        query::With,
        system::{Commands, Query, Res},
    },
    input::{keyboard::KeyCode, ButtonInput},
    math::Vec2,
    sprite::SpriteBundle,
    transform::components::Transform,
    utils::default,
};

use crate::{
    asset_loader::SceneAsset,
    movement::{MovementBundle, Velocity},
};

const PLAYER_SPEED: f32 = 20.0;

#[derive(Debug, Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_player)
            .add_systems(Update, player_movement);
    }
}

fn load_player(mut commands: Commands, scene_asset: Res<SceneAsset>) {
    commands.spawn((
        MovementBundle {
            model: SpriteBundle {
                texture: scene_asset.player.clone(),
                transform: Transform::from_xyz(100., 0., 0.),
                ..default()
            },
            velocity: Velocity::new(Vec2::ZERO),
        },
        Player,
    ));
}

fn player_movement(
    mut query: Query<&mut Velocity, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut velocity = query.single_mut();
    let mut movement_x = 0.0;
    let mut movement_y = 0.0;

    if keyboard_input.pressed(KeyCode::KeyW) {
        movement_y = PLAYER_SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        movement_y = -PLAYER_SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyA) {
        movement_x = -PLAYER_SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        movement_x = PLAYER_SPEED;
    }

    velocity.value = Vec2::new(movement_x, movement_y);
}
