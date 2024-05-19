use bevy::prelude::*;

const DESPAWN_DISTANCE: f32 = 100.0;

// 这里定义了 DespawnPlugin 结构体，用作插件的实现基础。
pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_far_away_entities);
    }
}


//  遍历所有带有 GlobalTransform 组件的实体，并计算每个实体的位置与原点 (Vec3::ZERO) 的距离。
// 如果这个距离大于 DESPAWN_DISTANCE，则使用 Commands 结构体的 entity() 和 despawn_recursive() 方法
// 来销毁该实体及其所有子实体
fn despawn_far_away_entities(mut commands: Commands, query: Query<(Entity, &GlobalTransform)>) {
    for (entity, transform) in query.iter() {
        let distance = transform.translation().distance(Vec3::ZERO);

        // Entity is far away from the camera's viewport.
        if distance > DESPAWN_DISTANCE {
            commands.entity(entity).despawn_recursive();
        }
    }
}
