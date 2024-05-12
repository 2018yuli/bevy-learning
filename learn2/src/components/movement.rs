use bevy::prelude::*;

// 定义了一个 Velocity 结构体
// Component 特性使得这个结构体可以用作 Bevy 的实体组件系统 (ECS) 的组件
// 而 Debug 特性允许该结构体可以被打印出来，有助于调试
// - pub value: Vec3：Velocity 包含一个公开的 Vec3 类型的字段 value，用于存储速度向量。
#[derive(Component,Debug)]
pub struct Velocity {
    pub value: Vec3,
}

// 这里定义了一个名为 MovementPlugin 的结构体，用于实现插件功能
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position);
    }
}

// 用于更新具有 Velocity 和 Transform 组件的实体的位置
// mut query: Query<(&Velocity, &mut Transform)>：这是一个查询参数，
//  用于检索所有同时具有 Velocity 和 Transform 组件的实体。
// time: Res<Time>：这是资源参数，Time 类型来自 Bevy
// 更新的量是速度乘以自上一帧以来的时间，即 time.delta_seconds()
fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}