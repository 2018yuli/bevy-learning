mod components;

use bevy::prelude::*;
use components::asset_loading::AssetLoaderPlugin;
use components::asteroids::AsteroidPlugin;
use components::collision_detection::CollisionDetectionPlugin;
use components::despawn::DespawnPlugin;
use components::spaceship::SpaceshipPlugin;
use components::movement::MovementPlugin;
use components::debug::DebugPlugin;
use components::camera::CameraPlugin;

fn main() {
    App::new()
    // build-ins
    .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
    .insert_resource(AmbientLight {
        color: Color::default(),
        brightness: 0.95,
    })
    .add_plugins(DefaultPlugins)
    // 用户插件
    .add_plugins(AssetLoaderPlugin)
    .add_plugins(MovementPlugin)
    //.add_plugins(DebugPlugin)
    .add_plugins(SpaceshipPlugin)
    .add_plugins(AsteroidPlugin)
    .add_plugins(CameraPlugin)
    .add_plugins(CollisionDetectionPlugin)
    .add_plugins(DespawnPlugin)
    .run();
}
