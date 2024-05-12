// 导入 Bevy 库的预导入模块
use bevy::prelude::*;

// 这里定义了一个名为 DebugPlugin 的公共结构体。该结构体没有成员，
// 它的主要作用是作为一个 Bevy 插件的实现基础
pub struct DebugPlugin;

// 这段代码实现了 Bevy 的 Plugin 特性
// 这是 Plugin 特性必须实现的方法，用于设置插件。这个方法接收一个可变引用到 Bevy 的 App 对象，允许插件配置应用程序
// 这行代码在 Bevy 应用的更新阶段（Update）添加了一个系统 print_position。这意味着在每个更新周期，print_position 系统都会被执行。
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_position);
    }
}

// 这是一个系统函数，它使用 Query<(Entity, &Transform)> 参数来查询游戏世界中所有带有 Transform 组件的实体
// Transform 组件通常用于存储实体在游戏世界中的位置、旋转和缩放
// - 遍历查询结果：通过迭代 query.iter() 获取每个实体及其对应的 Transform 组件
// - 打印实体位置：对于每个实体和其变换（位置信息存储在 transform.translation），使用 info! 宏打印实体的 ID 和其位置
fn print_position(query: Query<(Entity, &Transform)>) {
    for (entity, transform) in query.iter() {
        info!(
            "Entity {:?} is at position {:?},", 
            entity, transform.translation
        );
    }
}
