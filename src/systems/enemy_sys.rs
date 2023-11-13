use crate::prelude::*;
use std::iter::repeat_with;

pub fn move_enemy_random(
    mut query: Query<(&mut Transform, &Speed), With<Enemy>>,
    time: Res<Time>,
){
    let x_sign = if fastrand::bool() {1.} else {-1.};
    let y_sign = if fastrand::bool() {1.} else {-1.};

    let array : [f32;2] = [fastrand::f32() * x_sign , fastrand::f32() * y_sign];
    let direction = Vec2::from_array(array);

    let (mut enemy_transform, enemy_speed) = query.single_mut();
    let new_enemy_pos = enemy_transform.translation + Vec3::from((direction, 0.0)) * enemy_speed.value * time.delta_seconds();
    enemy_transform.translation =  new_enemy_pos;
}