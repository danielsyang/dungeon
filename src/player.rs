use bevy::{
    app::{App, Plugin, PostStartup, Update},
    asset::Assets,
    ecs::{
        component::Component,
        system::{Commands, Query, Res, ResMut},
    },
    input::{keyboard::KeyCode, ButtonInput},
    log::info,
    math::{Vec2, Vec3},
    prelude::{Deref, DerefMut},
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasLayout},
    time::{Time, Timer, TimerMode},
    transform::components::Transform,
    utils::default,
};

use crate::{
    asset_loader::SceneAsset,
    movement::{MovementBundle, Velocity},
};

const PLAYER_SPEED: f32 = 20.0;

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
            .add_systems(Update, animate_sprite_player);
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
                info!("Animation index: {}", atlas.index);
                indices.first
            } else {
                info!("Add Animation index: {}", atlas.index + 1);
                atlas.index + 1
            };
        }
    }
}

fn player_movement(
    mut query: Query<(
        &mut Velocity,
        &mut TextureAtlas,
        &mut PlayerAnimationIndices,
    )>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let (mut velocity, mut atlas, mut indices) = query.single_mut();
    let mut movement_x = 0.0;
    let mut movement_y = 0.0;
    let mut index: Option<usize> = None;

    if keyboard_input.pressed(KeyCode::KeyW) {
        movement_y = PLAYER_SPEED;
        index = Some(12);
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        movement_y = -PLAYER_SPEED;
        index = Some(0);
    } else if keyboard_input.pressed(KeyCode::KeyA) {
        movement_x = -PLAYER_SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        movement_x = PLAYER_SPEED;
        index = Some(6);
    }

    velocity.value = Vec2::new(movement_x, movement_y);

    if let Some(index) = index {
        indices.first = index;
        indices.last = index + 5;

        atlas.index = index;
    }
}
