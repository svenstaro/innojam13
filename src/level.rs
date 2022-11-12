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

fn setup_map(mut commands: Commands, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();

    let window_width = window.width() as f32;
    let window_height = window.height() as f32;

    // Create the ground.
    commands
        .spawn()
        .insert(Collider::cuboid(500.0, 100.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            1500.0, 200.0, 0.0,
        )));

    let fountain = Collider::cuboid(50.0, 50.0);
    let fountain_offset = Transform::from_xyz(
        -(window_width / 2.0 - 50.0 - 10.0),
        -window_height / 2.0 + 100.0 + 50.0 + 10.0,
        0.0,
    );
    commands
        .spawn()
        .insert(Fountain)
        .insert(fountain)
        .insert_bundle(TransformBundle::from(fountain_offset));

    let base = Collider::cuboid(50.0, 50.0);
    let base_offset = Transform::from_xyz(
        window_width / 2.0 - 50.0 - 10.0,
        -window_height / 2.0 + 100.0 + 50.0 + 10.0,
        0.0,
    );
    commands
        .spawn()
        .insert(Base)
        .insert(base)
        .insert_bundle(TransformBundle::from(base_offset));
}
