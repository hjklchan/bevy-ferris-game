use bevy::prelude::*;
use crate::BASE_MOVEMENT_SPEED;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement);
    }
}

/// # Movement 移动标记组件
#[derive(Component)]
pub struct Movement;

#[derive(Component, Default)]
pub struct Velocity {
    pub value: Vec3,
}

pub fn movement(mut query: Query<(&mut Transform, &Velocity), With<Movement>>, r_time: Res<Time>) {
    let delta = r_time.delta_seconds();
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.value.normalize_or_zero() * BASE_MOVEMENT_SPEED * delta;
    }
}