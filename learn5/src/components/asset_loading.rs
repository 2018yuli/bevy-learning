use bevy::prelude::*;

// 定义了一个名为 SceneAssets 的结构体，用于存储游戏中的关键场景资源
// 这个结构体实现了 Resource 特性，使其可以被注册为 Bevy 应用的全局资源
// 还实现了 Debug 和 Default 特性，分别用于调试输出和提供默认值。
// Handle<Scene>：Handle 是一个指向资源的智能指针，用于异步加载资源
#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub asteroid: Handle<Scene>,
    pub spaceship: Handle<Scene>,
    pub missiles: Handle<Scene>,
}

// 定义了一个插件，用于加载和初始化 SceneAssets 资源
pub struct AssetLoaderPlugin;

// 初始化 SceneAssets 资源：init_resource::<SceneAssets>() 确保 SceneAssets 资源在应用启动前被初始化
// load_assets 注册：add_systems(Startup, load_assets) 启动阶段添加 load_assets 系统，用于加载资源
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

/**
 * 用于实际加载资源
 * scene_assets:        一个可变资源引用，用于存储加载的场景
 * asset_server         资源服务器，用于加载指定的 .glb 文件
 */
fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        asteroid: asset_server.load("Asteroid.glb#Scene0"),
        spaceship: asset_server.load("spaceship.glb#Scene0"),
        missiles: asset_server.load("Missiles.glb#Scene0"),
    }
}