mod components;
mod systems;

mod prelude {
    pub use crate::components::*;
    pub use crate::systems::*;
    pub use bevy::prelude::*;
    pub use bevy::sprite::{MaterialMesh2dBundle};
}

use prelude::*;
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

    commands.spawn((Player, Speed{value: 100.0}, MaterialMesh2dBundle{
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::default()),
        ..default()
    }));
}




