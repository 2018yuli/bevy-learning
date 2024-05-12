// 导入 Bevy 库的预导入模块
use bevy::prelude::*;

// 定义相机距离常量
const CAMERA_DISTANCE: f32 = 80.0;

// 这里定义了一个名为 CameraPlugin 的公共结构体。该结构体将被用作 Bevy 插件
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

// 使用 commands.spawn() 方法来创建一个新实体，并为其附加一个 Camera3dBundle 组件包
// 这个组件包包括了3D相机所需的所有基础组件
// 设置相机的位置和方向
// - 设置相机的位置。相机被放置在 (0.0, CAMERA_DISTANCE, 0.0)，意味着它位于 Y 轴上，距离原点 CAMERA_DISTANCE
// - 相机朝向原点 (0.0, 0.0, 0.0)，并以 Z 轴的方向为 "up" 方向。这样设置相机是水平观看原点
// - ..default()：这个语法用于自动填充剩余未显式设置的字段为它们的默认值
// 相机的常见属性
/**
 * ### 1. `Transform`
 *  - **作用**：控制相机在3D空间中的位置和朝向。
 *  - **常用方法**：
 *  - `from_xyz(x: f32, y: f32, z: f32)`：设置相机的世界空间坐标。
 *  - `looking_at(target: Vec3, up: Vec3)`：设置相机的朝向，使相机朝向指定的 `target` 点，以 `up` 向量作为上方向。
 *  ### 2. `PerspectiveProjection`
 *  - **作用**：定义一个透视投影相机，用于3D渲染，影响如何将3D场景投影到2D屏幕上。
 *  - **常用属性**：
 *  - `fov`（Field of View，视场）：相机的视角宽度，通常以角度表示，决定了观察者能看到的场景范围。
 *  - `aspect_ratio`：宽高比，影响视野的形状。
 *  - `near`：近裁剪面距离，这是相机可以看到的最近距离。
 *  - `far`：远裁剪面距离，这是相机可以看到的最远距离。
 *  ### 3. `Camera`
 *  - **作用**：标记实体为相机，控制渲染管线如何使用该相机进行渲染。
 *  - **常用属性**：
 *  - `name`：相机的名字，可以用来在多相机设置中标识和选择特定的相机。
 *  ### 4. `VisibleEntities`
 *  - **作用**：存储由当前相机视角可见的实体列表。
 *  - **注意**：这通常由渲染系统自动维护，不需要手动修改。
 *  ### 5. `Frustrum`
 *  - **作用**：存储相机视锥体的信息，视锥体用于决定哪些对象在相机视野中，并因此需要被渲染。
 *  - **注意**：这也是由引擎自动计算和维护的。
 */
fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, CAMERA_DISTANCE, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });
}