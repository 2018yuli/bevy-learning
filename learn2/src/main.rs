mod components;

use bevy::prelude::*;
use components::spaceship::SpaceshipPlugin;
use components::movement::MovementPlugin;
use components::debug::DebugPlugin;
use components::camera::CameraPlugin;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    // build-ins
    .insert_resource(ClearColor(Color::rgb(0.49, 0.44, 0.46)))
    .insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.75,
    })
    
    // 用户插件
    .add_plugins(MovementPlugin)
    .add_plugins(DebugPlugin)
    .add_plugins(SpaceshipPlugin)
    .add_plugins(CameraPlugin)
    .run();
    
}
