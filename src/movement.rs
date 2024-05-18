use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        bundle::Bundle,
        component::Component,
        system::{Query, Res},
    },
    math::Vec2,
    sprite::SpriteSheetBundle,
    time::Time,
    transform::components::Transform,
};

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Debug)]
pub struct MovementDirection {
    pub value: Direction,
}

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec2,
    pub direction: Option<Direction>,
}

impl Velocity {
    pub fn new(value: Vec2) -> Self {
        Self {
            value,
            direction: Some(Direction::Down),
        }
    }
}

#[derive(Bundle)]
pub struct MovementBundle {
    pub model: SpriteSheetBundle,
    pub velocity: Velocity,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position);
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value.extend(0.0) * time.delta_seconds();
    }
}
