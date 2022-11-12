use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct LevelPlugin;

#[derive(Debug, Default, Component)]
pub struct Fountain;

#[derive(Debug, Default, Component)]
pub struct Base;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_map);
    }
}

fn setup_map(mut commands: Commands) {
    // Create the ground.
    commands
        .spawn()
        .insert(Collider::cuboid(500.0, 100.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            1500.0, 200.0, 0.0,
        )));
}
