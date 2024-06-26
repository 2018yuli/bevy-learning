use bevy::{ecs::query, input::keyboard::KeyboardInput, prelude::*};
use super::{asset_loading::SceneAssets, collision_detection::Collider, movement::{Acceleration, MovingObjectBundle, Velocity}};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const SPACESHIP_RADIUS: f32 = 5.0;
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const MISSILE_SPEED: f32 = 50.0;
const MISSILE_FORWARD_SPAWN_SCALAR: f32 = 7.5;
const MISSILE_RADIUS: f32 = 1.0;

// 这里定义了飞船的组件
#[derive(Component, Debug)]
pub struct Spaceship;

// 这里定义了飞船导弹的组件
#[derive(Component, Debug)]
pub struct SpaceshipMissile;

// 定义了一个包含速度和模型的 Bundle
// 方便在创建飞船实体时将这些组件一起添加
#[derive(Bundle)]
struct SpaceshipBundle {
    velocity: Velocity,
    model: SceneBundle,
}

// 注册了初始化飞船的系统和两个控制系统：一个用于飞船的运动控制，另一个用于飞船的武器控制
pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship)
        .add_systems(Update, (spaceship_movement_controls, spaceship_weapon_controls));
    }
}

// 这个系统在游戏启动后创建一个飞船实体，使用从 SceneAssets 资源加载的飞船模型，并设置其初始位置和速度
fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            collider: Collider::new(MISSILE_RADIUS),
            model: SceneBundle {
                scene: scene_assets.spaceship.clone(),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
        },
        Spaceship,
    ));
}

// 这个系统处理飞船的运动，包括前进、后退和旋转。
// 运动是根据按键输入（W, S, A, D, ShiftLeft, ControlLeft）进行的，
// 使用 Bevy 的 Input<KeyCode> 来检测按键状态
fn spaceship_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
    keyboard_input: Res<Input<KeyCode>>, 
    time: Res<Time>) {

    let (mut transform, mut velocity) = query.single_mut();
    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    if keyboard_input.pressed(KeyCode::D) {
        rotation = -SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::A) {
        rotation = SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::S) {
        movement = -SPACESHIP_SPEED;
    } else if keyboard_input.pressed(KeyCode::W) {
        movement = SPACESHIP_SPEED;
    }

    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        roll = -SPACESHIP_ROLL_SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::ControlLeft) {
        roll = SPACESHIP_ROLL_SPEED * time.delta_seconds();
    }

    // Rotate around the Y-axis.
    // Ignores the Z-axis rotation applied below.
    transform.rotate_y(rotation);

    // Rotate around the local Z-axis.
    // The rotation is relative to the current rotation!
    transform.rotate_local_z(roll);

    // Update the spaceship's velocity based on new direction.
    velocity.value = -transform.forward() * movement;
}

// 这个系统允许在按下空格键时发射导弹。导弹的初始位置和速度是基于飞船当前的朝向和位置计算的。
fn spaceship_weapon_controls(mut commands: Commands, query: Query<&Transform, With<Spaceship>>, 
    keyboard_input: Res<Input<KeyCode>>, scene_assets: Res<SceneAssets>) {

    let transform = query.single();
    if keyboard_input.pressed(KeyCode::Space) {
        commands.spawn((
            MovingObjectBundle {
                velocity: Velocity::new(-transform.forward() * MISSILE_SPEED),
                acceleration: Acceleration::new(Vec3::ZERO),
                collider: Collider::new(SPACESHIP_RADIUS),
                model: SceneBundle {
                    scene: scene_assets.missiles.clone(),
                    transform: Transform::from_translation(
                        transform.translation + -transform.forward() * MISSILE_FORWARD_SPAWN_SCALAR,
                    ),
                    ..default()
                },
            },
            SpaceshipMissile,
        ));
    }
}