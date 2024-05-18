use bevy::{
    app::{App, Plugin, PostStartup, Update},
    asset::Assets,
    ecs::{
        component::Component,
        system::{Commands, Query, Res, ResMut},
    },
    input::{keyboard::KeyCode, ButtonInput},
    math::{Vec2, Vec3},
    prelude::{Deref, DerefMut},
    sprite::{Sprite, SpriteSheetBundle, TextureAtlas, TextureAtlasLayout},
    time::{Time, Timer, TimerMode},
    transform::components::Transform,
    utils::default,
};

use crate::{
    asset_loader::SceneAsset,
    movement::{Direction, MovementBundle, Velocity},
};

const PLAYER_SPEED: f32 = 40.0;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct PlayerAnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Debug, Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, load_player)
            .add_systems(Update, player_movement)
            .add_systems(Update, animate_sprite_player)
            .add_systems(Update, player_direction)
            .add_systems(Update, player_idle);
    }
}

fn load_player(
    mut commands: Commands,
    scene_asset: Res<SceneAsset>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(Vec2::new(48.0, 48.0), 6, 10, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = PlayerAnimationIndices { first: 0, last: 5 };

    commands.spawn((
        MovementBundle {
            model: SpriteSheetBundle {
                texture: scene_asset.player.clone(),
                atlas: TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
                // transform: Transform::from_xyz(100., 0., 0.),
                transform: Transform::from_scale(Vec3::splat(2.0)),
                ..default()
            },
            velocity: Velocity::new(Vec2::ZERO),
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player,
    ));
}

fn animate_sprite_player(
    time: Res<Time>,
    mut query: Query<(
        &PlayerAnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlas,
    )>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

fn player_movement(mut query: Query<&mut Velocity>, keyboard_input: Res<ButtonInput<KeyCode>>) {
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

fn player_direction(
    mut query: Query<(
        &mut Velocity,
        &mut TextureAtlas,
        &mut PlayerAnimationIndices,
        &mut Sprite,
    )>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let (mut velocity, mut atlas, mut indices, mut sprite) = query.single_mut();

    if keyboard_input.just_pressed(KeyCode::KeyW) && velocity.direction != Some(Direction::Up) {
        indices.first = 30;
        atlas.index = 30;
        indices.last = 35;
        velocity.direction = Some(Direction::Up);
    }

    if keyboard_input.just_pressed(KeyCode::KeyS) && velocity.direction != Some(Direction::Down) {
        indices.first = 18;
        atlas.index = 18;
        indices.last = 23;
        velocity.direction = Some(Direction::Down);
    }

    if keyboard_input.just_pressed(KeyCode::KeyA) && velocity.direction != Some(Direction::Left) {
        indices.first = 24;
        atlas.index = 24;
        indices.last = 29;
        sprite.flip_x = true;
        velocity.direction = Some(Direction::Left);
    }

    if keyboard_input.just_pressed(KeyCode::KeyD) && velocity.direction != Some(Direction::Right) {
        indices.first = 24;
        atlas.index = 24;
        indices.last = 29;
        sprite.flip_x = false;
        velocity.direction = Some(Direction::Right);
    }
}

fn player_idle(
    mut query: Query<(
        &mut Velocity,
        &mut TextureAtlas,
        &mut PlayerAnimationIndices,
        &mut Sprite,
    )>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let (mut velocity, mut atlas, mut indices, mut sprite) = query.single_mut();
    if keyboard_input.just_released(KeyCode::KeyW) {
        indices.first = 12;
        atlas.index = 12;
        indices.last = 17;
    }

    if keyboard_input.just_released(KeyCode::KeyS) {
        indices.first = 0;
        atlas.index = 0;
        indices.last = 5;
    }

    if keyboard_input.just_released(KeyCode::KeyA) {
        indices.first = 6;
        atlas.index = 6;
        indices.last = 11;
        sprite.flip_x = true;
    }

    if keyboard_input.just_released(KeyCode::KeyD) {
        indices.first = 6;
        atlas.index = 6;
        indices.last = 11;
        sprite.flip_x = false;
    }

    velocity.direction = None;
}
