use bevy::prelude::*;

#[derive(Component,Debug)]
pub struct Velocity {
    pub value: Vec3,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position);
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        // transform.translation.x += velocity.value.x;
        // transform.translation.y += velocity.value.y;
        // transform.translation.z += velocity.value.z;
        transform.translation += velocity.value * time.delta_seconds();
    }
}