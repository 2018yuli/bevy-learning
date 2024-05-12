use bevy::prelude::*;
use super::movement::Velocity;

// 分别用于设置飞船的初始位置和速度
const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);


// 这里定义了一个名为 SpaceshipBundle 的结构体，并使用 #[derive(Bundle)] 属性
// 在 Bevy 中，Bundle 是一个便捷的方式，用于将多个组件一起添加到实体
// - velocity: Velocity — 用于存储飞船的速度。
// - model: SceneBundle — 用于存储飞船的模型
//  - SceneBundle 是 Bevy 中用于加载和展示 3D 模型的组件集合
// 它使得组件的管理变得更为高效和有组织。通过将相关组件组合在一起，你可以更容易地构建和维护复杂的实体系统
/*
SceneBundle 是一个特别的 Bundle，用于从外部文件（如 .glb 或 .gltf 格式）加载和实例化整个场景。
这包括场景中的所有实体、组件和层次结构，使得它们可以一次性被添加到 Bevy 的实体组件系统（ECS）中
SceneBundle 包含了以下主要组件，这些组件协同工作，以确保场景可以正确加载和渲染：
Scene: 这是场景的主要组件，包含了场景的所有数据。它通常由外部文件加载，例如使用 .glb 格式的3D模型。
    Scene 组件实际上封装了一个或多个实体及其相关的组件（如变换、网格、材质等）。
Transform: 控制场景实体在世界中的位置、旋转和缩放。Transform 是一个通用组件，用于几乎所有需要在3D空间中表现的实体。
GlobalTransform: 自动计算并维护的组件，用于存储相对于世界原点的最终变换矩阵。它通常由引擎内部使用，以优化渲染和交互。
*/
#[derive(Bundle)]
struct SpaceshipBundle {
    velocity: Velocity,
    model: SceneBundle,
}

// 定义 SpaceshipPlugin
pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spaceship);
    }
}

// 这个系统函数用于创建飞船实体，创建了 Bundle 后，你可以在创建实体时使用它：
// Commands 是 Bevy 提供的一个工具，用于执行各种命令，如创建和修改实体。
// AssetServer 资源用于加载外部资源，如 3D 模型
// Transform::from_translation(STARTING_TRANSLATION)
fn spawn_spaceship(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 加载场景文件 #Scene0：
    // .glb 文件：这种文件格式可以包含一个或多个“场景”
    // 这个后缀表示加载文件中索引为 0 的场景。索引通常从 0 开始，所以 Scene0 通常是文件中的第一个场景
    let scene_handle: Handle<Scene> = asset_server.load("spaceship.glb#Scene0");
    commands.spawn(SpaceshipBundle {
        velocity: Velocity {
            value: STARTING_VELOCITY,
        },
        model: SceneBundle {
            scene: scene_handle,
            transform: Transform::from_translation(STARTING_TRANSLATION),
            ..default()
        },
    });
}
