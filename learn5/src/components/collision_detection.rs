use bevy::{prelude::*, utils::hashbrown::HashMap};

// Collider 组件定义了一个具有半径和正在碰撞的实体列表的碰撞器
// radius 属性表示碰撞检测时的半径。
// colliding_entities 用来存储与该实体碰撞的其他实体的列表。
#[derive(Component, Debug)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entities: Vec<Entity>,
}

// 提供了一个构造函数来创建 Collider 实例
// 同时初始化 colliding_entities 为一个空的向量。
impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            colliding_entities: vec![],
        }
    }
}

// CollisionDetectionPlugin 插件注册了一个名为 collision_detection 的系统
pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_detection);
    }
}

// 检测碰撞：系统遍历所有带有 GlobalTransform 和 Collider 组件的实体，
//  - 计算每对实体之间的距离，如果距离小于两者半径之和，就将它们记录为碰撞
// 更新碰撞：在碰撞检测后，系统将为每个实体更新其 colliding_entities 列表，以包含所有与之碰撞的实体
fn collision_detection(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>) {
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    // First phase: Detect Collisions.
    for (entity_a, transform_a, collider_a) in query.iter() {
        for (entity_b, transform_b, collider_b) in query.iter() {
            if entity_a != entity_b {
                let distance = transform_a.translation().distance(transform_b.translation());
                if distance < collider_a.radius + collider_b.radius {
                    colliding_entities.entry(entity_a).or_insert_with(Vec::new).push(entity_b);
                }
            }
        }
    }

    // Second phase: Update Collisions.
    for (entity, _, mut collider) in query.iter_mut() {
        collider.colliding_entities.clear();
        if let Some(collisions) = colliding_entities.get(&entity) {
            collider.colliding_entities.extend(collisions.iter().copied());
        }
    }
}