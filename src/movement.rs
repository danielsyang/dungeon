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

#[derive(Component, Debug)]
pub enum MovementDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec2,
}

impl Velocity {
    pub fn new(value: Vec2) -> Self {
        Self { value }
    }
}

#[derive(Bundle)]
pub struct MovementBundle {
    pub model: SpriteSheetBundle,
    pub direction: MovementDirection,
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
