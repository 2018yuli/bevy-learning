use bevy::prelude::*;
use std::ops::Range;
use rand::prelude::*;

use super::{asset_loading::SceneAssets, collision_detection::Collider, movement::{Acceleration, MovingObjectBundle, Velocity}};

const VELOCITY_SCALAR: f32 = 5.0;
const ACCELERATION_SCALAR: f32 = 1.0;
const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..25.0;
const SPAWN_TIME_SECONDS: f32 = 1.0;
const ROTATE_SPEED: f32 = 2.5;
const RADIUS: f32 = 2.5;

// Asteroid 是一个标记组件，用于在 ECS 查询中标识小行星实体
#[derive(Component, Debug)]
pub struct Asteroid;

// SpawnTimer 是一个资源，包含一个计时器 Timer，用于控制小行星的生成频率
#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer: Timer,
}


pub struct AsteroidPlugin;

// AsteroidPlugin 插件用于初始化 SpawnTimer 资源，
// 并注册三个系统：生成小行星、旋转小行星和处理小行星的碰撞。
impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
        }).add_systems(Update, (spawn_asteroid, rotate_asteroids, handle_asteroid_collisions));
    }
}

// 会周期性地检查计时器，若计时器触发，则生成一个小行星。
// 小行星的位置随机生成于定义的 X 和 Z 范围内，速度和加速度是基于随机单位向量的
fn spawn_asteroid(mut commands: Commands, mut spawn_timer: ResMut<SpawnTimer>, time: Res<Time>, scene_assets: Res<SceneAssets>) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }

    // we need to bring in the rand crate.
    let mut rng = rand::thread_rng();

    let translation = Vec3::new(rng.gen_range(SPAWN_RANGE_X), 0., rng.gen_range(SPAWN_RANGE_Z),);

    let mut random_unit_vector = 
        || Vec3::new(rng.gen_range(-1.0..1.0), 0., rng.gen_range(-1.0..1.0)).normalize_or_zero();
    let velocity = random_unit_vector() * VELOCITY_SCALAR;
    let acceleration = random_unit_vector() * ACCELERATION_SCALAR;

    commands.spawn((
        MovingObjectBundle{
            velocity: Velocity::new(velocity),
            acceleration: Acceleration::new(acceleration),
            collider: Collider::new(RADIUS),
            model: SceneBundle {
                scene: scene_assets.asteroid.clone(),
                transform: Transform::from_translation(translation),
                ..default()
            }
        }, 
        Asteroid,
    ));
}

// 这个系统会遍历所有带有 Asteroid 组件的实体，根据设定的旋转速度更新它们的旋转状态
fn rotate_asteroids(mut query: Query<&mut Transform, With<Asteroid>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotate_local_z(ROTATE_SPEED * time.delta_seconds());
    }
}

// 检测与小行星碰撞的其他实体。
// 如果发现碰撞，则将小行星实体从 ECS 中移除
fn handle_asteroid_collisions(mut commands: Commands, query: Query<(Entity, &Collider), With<Asteroid>>) {
    for (entity, collider) in query.iter() {
        for &collided_entity in collider.colliding_entities.iter() {
            // asteroid collided with another asteroid
            if query.get(collided_entity).is_ok() {
                continue;
            }
            // Despawn the asteroid.
            commands.entity(entity).despawn_recursive();
        }
    }
}