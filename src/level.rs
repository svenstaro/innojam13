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

fn create_chunk(
    commands: &mut Commands,
    width: f32,
    height: f32,
    position_x: f32,
    position_y: f32,
    rotation: f32,
) {
    commands
        .spawn()
        .insert(Collider::cuboid(width, height))
        .insert_bundle(TransformBundle::from(
            Transform::from_xyz(position_x, position_y, 1.0)
                .with_rotation(Quat::from_rotation_z(rotation.to_radians())),
        ));
}

fn setup_map(mut commands: Commands, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();

    let window_width = window.width() as f32;
    let window_height = window.height() as f32;

    // Create the ground.
    // commands
    //     .spawn()
    //     .insert(Collider::cuboid(500.0, 100.0))
    //     .insert_bundle(TransformBundle::from(Transform::from_xyz(
    //         1500.0, 200.0, 0.0,
    //     )));

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

    // ground
    create_chunk(&mut commands, 1600.0, 35.0, 1600.0, 50.0, 0.0);

    // cave exit enemies left and right

    create_chunk(&mut commands, 35.0, 150.0, 35.0, 120.0, 0.0);
    create_chunk(&mut commands, 35.0, 150.0, 3165.0, 120.0, 0.0);

    // chunks for walking; not rotated; from lowest to highest

    create_chunk(&mut commands, 150.0, 35.0, 2045.0, 500.0, 0.0);
    create_chunk(&mut commands, 325.0, 35.0, 325.0, 700.0, 0.0);
    create_chunk(&mut commands, 200.0, 35.0, 2900.0, 750.0, 0.0);
    create_chunk(&mut commands, 200.0, 35.0, 1386.0, 1012.0, 0.0);
    create_chunk(&mut commands, 420.0, 35.0, 750.0, 1304.0, 0.0);
    create_chunk(&mut commands, 400.0, 35.0, 1600.0, 1700.0, 0.0);

    // chunks for walking up, from lowest to highest
    create_chunk(&mut commands, 400.0, 35.0, 2500.0, 280.0, 145.0);
    create_chunk(&mut commands, 250.0, 35.0, 830.0, 570.0, 145.0);
    create_chunk(&mut commands, 320.0, 35.0, 1750.0, 770.0, 127.0);
    create_chunk(&mut commands, 125.0, 35.0, 273.0, 1222.0, 50.0);
    create_chunk(&mut commands, 400.0, 35.0, 2300.0, 1480.0, 145.0);

    // ladder priority one: from right to to left, priority second: from lowest to highest

    // first
    create_chunk(&mut commands, 50.0, 17.5, 1150.0, 140.0, 0.0);
    create_chunk(&mut commands, 50.0, 17.5, 1150.0, 215.0, 0.0);
    create_chunk(&mut commands, 50.0, 17.5, 1150.0, 290.0, 0.0);
    create_chunk(&mut commands, 50.0, 17.5, 1150.0, 365.0, 0.0);
    create_chunk(&mut commands, 50.0, 17.5, 1150.0, 435.0, 0.0);

    //first
    create_chunk(&mut commands, 50.0, 17.5, 100.0, 775.0, 0.0);
    create_chunk(&mut commands, 50.0, 17.5, 100.0, 850.0, 0.0);
    create_chunk(&mut commands, 50.0, 17.5, 100.0, 925.0, 0.0);
    create_chunk(&mut commands, 50.0, 17.5, 100.0, 1000.0, 0.0);
    create_chunk(&mut commands, 50.0, 17.5, 100.0, 1075.0, 0.0);

    //last
    create_chunk(&mut commands, 50.0, 17.5, 1300.0, 1075.0, 0.0);
    create_chunk(&mut commands, 50.0, 17.5, 1300.0, 1150.0, 0.0);
    create_chunk(&mut commands, 50.0, 17.5, 1300.0, 1225.0, 0.0);
    create_chunk(&mut commands, 50.0, 17.5, 1300.0, 1300.0, 0.0);

    //last
    create_chunk(&mut commands, 50.0, 17.5, 1100.0, 1400.0, 0.0);
    create_chunk(&mut commands, 50.0, 17.5, 1100.0, 1475.0, 0.0);
    create_chunk(&mut commands, 50.0, 17.5, 1100.0, 1550.0, 0.0);
    create_chunk(&mut commands, 50.0, 17.5, 1100.0, 1625.0, 0.0);
    create_chunk(&mut commands, 50.0, 17.5, 1100.0, 1700.0, 0.0);

    //vorletzter
    create_chunk(&mut commands, 50.0, 17.5, 2750.0, 840.0, 0.0);
    create_chunk(&mut commands, 50.0, 17.5, 2750.0, 915.0, 0.0);
    create_chunk(&mut commands, 50.0, 17.5, 2750.0, 990.0, 0.0);
    create_chunk(&mut commands, 50.0, 17.5, 2750.0, 1065.0, 0.0);
    create_chunk(&mut commands, 50.0, 17.5, 2750.0, 1140.0, 0.0);

    //last
    create_chunk(&mut commands, 50.0, 17.5, 3000.0, 140.0, 0.0);
    create_chunk(&mut commands, 50.0, 17.5, 3000.0, 215.0, 0.0);
    create_chunk(&mut commands, 50.0, 17.5, 3000.0, 290.0, 0.0);
    create_chunk(&mut commands, 50.0, 17.5, 3000.0, 365.0, 0.0);
    create_chunk(&mut commands, 50.0, 17.5, 3000.0, 435.0, 0.0);
    create_chunk(&mut commands, 50.0, 17.5, 3000.0, 510.0, 0.0);
    create_chunk(&mut commands, 50.0, 17.5, 3000.0, 585.0, 0.0);
    create_chunk(&mut commands, 50.0, 17.5, 3000.0, 660.0, 0.0);
}

#[warn(dead_code)]
fn spawn_fountain(mut commands: Commands) {
    let fountain = Collider::cuboid(50.0, 50.0);
    let fountain_offset = Transform::from_xyz(100.0, 100.0, 0.0);
    commands
        .spawn()
        .insert(Fountain)
        .insert(fountain)
        .insert(CollisionGroups::new(Group::GROUP_31, Group::NONE))
        .insert_bundle(TransformBundle::from(fountain_offset));

    let base = Collider::cuboid(50.0, 50.0);

    let base_offset = Transform::from_xyz(1500.0, 500.0, 0.0);
    commands
        .spawn()
        .insert(Base)
        .insert(base)
        .insert(CollisionGroups::new(Group::GROUP_31, Group::GROUP_1))
        .insert_bundle(TransformBundle::from(base_offset));
}
