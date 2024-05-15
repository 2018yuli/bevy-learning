use bevy::{prelude::*, winit::accessibility};

use super::collision_detection::Collider;

// 速度
#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

// 加速
#[derive(Component, Debug)]
pub struct Acceleration {
    pub value: Vec3,
}

impl Acceleration {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

// 可移动物体 Bundle
#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub collider: Collider,
    pub model: SceneBundle,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position);
    }
}

fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (accleration, mut velocity) in query.iter_mut() {
        velocity.value += accleration.value * time.delta_seconds();
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}