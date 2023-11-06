use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (
                move_player,
            ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    commands.spawn(Camera2dBundle::default());

    commands.spawn((Player, MaterialMesh2dBundle{
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::default()),
        ..default()
    }));
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
){
    let mut player_transform = query.single_mut();
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


    let new_player_pos = player_transform.translation + Vec3::from((direction, 0.0)) * 100.0 * time.delta_seconds();
    player_transform.translation =  new_player_pos;
}



#[derive(Component)]
struct Player;