use crate::prelude::*;
pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Speed), With<Player>>,
    time: Res<Time>,
){
    let mut direction = Vec2::default();

    if keyboard_input.pressed(KeyCode::Left){
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Right){
        direction.x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up){
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Down){
        direction.y -= 1.0;
    }

    let (mut player_transform, player_speed) = query.single_mut();
    let new_player_pos = player_transform.translation + Vec3::from((direction, 0.0)) * player_speed.value * time.delta_seconds();
    player_transform.translation =  new_player_pos;
}