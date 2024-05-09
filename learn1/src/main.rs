// 这行代码导入了 Bevy 库的预定义内容，使得常用的功能和类型可以直接使用，而无需指定完整的路径
use bevy::prelude::*;

// 这里定义了两个组件：Position 和 Velocity。每个组件都用 #[derive(Component, Debug)] 属性修饰
// Component 属性使得这些结构体可以被 Bevy 作为 ECS（Entity Component System）的组件使用
// Debug 属性允许这些结构体实例可以方便地打印出来，主要用于调试。
#[derive(Component, Debug)]
struct Position {
    x: f32,
    y: f32
}

#[derive(Component, Debug)]
struct Velocity {
    x: f32,
    y: f32
}

// add_systems(Startup, spawn_spaceship) 在应用启动时添加 spawn_spaceship 系统，用于创建初始实体。
// add_systems(Update, (update_postion, print_position)) 在每个更新周期中添加两个系统：
//  update_postion 和 print_position，分别用于更新位置和打印位置信息。
// add_plugins(DefaultPlugins) 添加默认插件集，这通常包括渲染、事件处理、音频管理等标准功能。
// run() 启动应用，这会使应用进入事件循环，直到程序结束。
// 整体上，这段代码展示了 Bevy 引擎的基本使用方式，特别是在使用 ECS 架构方面，通过系统来管理和更新组件数据。
fn main() {
    App::new()
    .add_systems(Startup, spawn_spaceship)
    .add_systems(Update, (update_postion, print_position))
    .add_plugins(DefaultPlugins)
    .run();
}

// spawn_spaceship 系统使用 Commands 参数来创建新的实体，这个实体同时具有 Position 和 Velocity 组件。
fn spawn_spaceship(mut commands: Commands) {
    commands.spawn((Position {x: 0.8, y: 0.0}, Velocity {x: 1.0, y: 1.0}));
}

// update_postion 系统查询所有具有 Velocity 和 Position 组件的实体，并更新它们的位置。
// 这通过将每个实体的速度向量添加到其位置向量来实现。
fn update_postion(mut query: Query<(&Velocity, &mut Position)>) {
    for (velocity, mut position) in query.iter_mut() {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}

// print_position 系统打印出所有实体及其位置的信息。这主要用于调试目的，以便开发者可以看到实体在空间中的分布。
fn print_position(query: Query<(Entity, &Position)>) {
    for (entity, position) in query.iter() {
        info!("Entity {:?} is at position {:?},", entity, position);
    }
}
