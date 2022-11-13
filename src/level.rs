use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use crate::WORLD_SIZE;

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
    meshes: &mut Assets<Mesh>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    add_collider: bool,
    asset_server: &Res<AssetServer>,
) {
    let entity = commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(2.0)),
                ..default()
            },
            texture: asset_server.load("items/LowerTile.png"),
            transform: Transform::from_scale(Vec3::new(width, height, 1.0))
                .with_translation(Vec3::new(position_x, position_y, 0.0))
                .with_rotation(Quat::from_rotation_z(rotation.to_radians())),
            ..default()
        })
        .id();

    if add_collider {
        commands.entity(entity).insert(Collider::cuboid(1.0, 1.0));
    }
}

fn setup_map(
    mut commands: Commands,
    windows: Res<Windows>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let window = windows.get_primary().unwrap();

    let window_width = window.width() as f32;
    let window_height = window.height() as f32;

    // background
    let quad = shape::Quad::new(WORLD_SIZE + Vec2::new(400.0, 400.0));
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(quad)).into(),
        material: materials.add(ColorMaterial::from(Color::hex("0C1E21").unwrap())),
        transform: Transform::from_xyz(WORLD_SIZE.x / 2.0, WORLD_SIZE.y / 2.0, 0.0),
        ..default()
    });

    let fountain_offset = Transform::from_xyz(180.0, 135.0, 1.0);
    commands
        .spawn()
        .insert(Fountain)
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(shape::Quad::new(Vec2::new(200.0, 200.0))))
                .into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: fountain_offset,
            visibility: Visibility { is_visible: false },
            ..default()
        });

    let base_offset = Transform::from_xyz(1500.0, 1800.0, 0.0);
    commands
        .spawn()
        .insert(Base)
        .insert_bundle(TransformBundle::from(base_offset));

    // Colliders around the map to prevent everything from leaving the map
    // bottom
    create_chunk(
        &mut commands,
        WORLD_SIZE.x + 200.0,
        100.0,
        0.0,
        -100.0,
        0.0,
        &mut meshes,
        &mut materials,
        true,
        &asset_server,
    );
    //top
    create_chunk(
        &mut commands,
        WORLD_SIZE.x + 200.0,
        100.0,
        0.0,
        WORLD_SIZE.y + 100.0,
        0.0,
        &mut meshes,
        &mut materials,
        true,
        &asset_server,
    );
    //left
    create_chunk(
        &mut commands,
        100.0,
        WORLD_SIZE.y,
        -200.0,
        0.0,
        0.0,
        &mut meshes,
        &mut materials,
        true,
        &asset_server,
    );
    //right
    create_chunk(
        &mut commands,
        100.0,
        WORLD_SIZE.y,
        WORLD_SIZE.x + 200.0,
        0.0,
        0.0,
        &mut meshes,
        &mut materials,
        true,
        &asset_server,
    );

    // ground
    create_chunk(
        &mut commands,
        1600.0,
        35.0,
        1600.0,
        50.0,
        0.0,
        &mut meshes,
        &mut materials,
        true,
        &asset_server,
    );

    // cave exit enemies left and right

    create_chunk(
        &mut commands,
        35.0,
        150.0,
        35.0,
        120.0,
        0.0,
        &mut meshes,
        &mut materials,
        true,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        35.0,
        150.0,
        3165.0,
        120.0,
        0.0,
        &mut meshes,
        &mut materials,
        true,
        &asset_server,
    );

    // chunks for walking; not rotated; from lowest to highest

    create_chunk(
        &mut commands,
        150.0,
        35.0,
        2045.0,
        500.0,
        0.0,
        &mut meshes,
        &mut materials,
        true,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        325.0,
        35.0,
        325.0,
        700.0,
        0.0,
        &mut meshes,
        &mut materials,
        true,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        200.0,
        35.0,
        2900.0,
        750.0,
        0.0,
        &mut meshes,
        &mut materials,
        true,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        200.0,
        35.0,
        1386.0,
        1012.0,
        0.0,
        &mut meshes,
        &mut materials,
        true,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        420.0,
        35.0,
        750.0,
        1304.0,
        0.0,
        &mut meshes,
        &mut materials,
        true,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        400.0,
        35.0,
        1600.0,
        1700.0,
        0.0,
        &mut meshes,
        &mut materials,
        true,
        &asset_server,
    );

    // chunks for walking up, from lowest to highest
    create_chunk(
        &mut commands,
        400.0,
        35.0,
        2500.0,
        280.0,
        145.0,
        &mut meshes,
        &mut materials,
        true,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        250.0,
        35.0,
        830.0,
        570.0,
        145.0,
        &mut meshes,
        &mut materials,
        true,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        320.0,
        35.0,
        1750.0,
        770.0,
        127.0,
        &mut meshes,
        &mut materials,
        true,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        125.0,
        35.0,
        273.0,
        1222.0,
        50.0,
        &mut meshes,
        &mut materials,
        true,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        400.0,
        35.0,
        2300.0,
        1480.0,
        145.0,
        &mut meshes,
        &mut materials,
        true,
        &asset_server,
    );

    // ladder priority one: from right to to left, priority second: from lowest to highest

    // first
    create_chunk(
        &mut commands,
        50.0,
        17.5,
        1150.0,
        140.0,
        0.0,
        &mut meshes,
        &mut materials,
        false,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        50.0,
        17.5,
        1150.0,
        215.0,
        0.0,
        &mut meshes,
        &mut materials,
        false,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        50.0,
        17.5,
        1150.0,
        290.0,
        0.0,
        &mut meshes,
        &mut materials,
        false,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        50.0,
        17.5,
        1150.0,
        365.0,
        0.0,
        &mut meshes,
        &mut materials,
        false,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        50.0,
        17.5,
        1150.0,
        435.0,
        0.0,
        &mut meshes,
        &mut materials,
        false,
        &asset_server,
    );

    //first
    create_chunk(
        &mut commands,
        50.0,
        17.5,
        100.0,
        775.0,
        0.0,
        &mut meshes,
        &mut materials,
        false,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        50.0,
        17.5,
        100.0,
        850.0,
        0.0,
        &mut meshes,
        &mut materials,
        false,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        50.0,
        17.5,
        100.0,
        925.0,
        0.0,
        &mut meshes,
        &mut materials,
        false,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        50.0,
        17.5,
        100.0,
        1000.0,
        0.0,
        &mut meshes,
        &mut materials,
        false,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        50.0,
        17.5,
        100.0,
        1075.0,
        0.0,
        &mut meshes,
        &mut materials,
        false,
        &asset_server,
    );

    //last
    create_chunk(
        &mut commands,
        50.0,
        17.5,
        1300.0,
        1075.0,
        0.0,
        &mut meshes,
        &mut materials,
        false,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        50.0,
        17.5,
        1300.0,
        1150.0,
        0.0,
        &mut meshes,
        &mut materials,
        false,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        50.0,
        17.5,
        1300.0,
        1225.0,
        0.0,
        &mut meshes,
        &mut materials,
        false,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        50.0,
        17.5,
        1300.0,
        1300.0,
        0.0,
        &mut meshes,
        &mut materials,
        false,
        &asset_server,
    );

    //last
    create_chunk(
        &mut commands,
        50.0,
        17.5,
        1100.0,
        1400.0,
        0.0,
        &mut meshes,
        &mut materials,
        false,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        50.0,
        17.5,
        1100.0,
        1475.0,
        0.0,
        &mut meshes,
        &mut materials,
        false,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        50.0,
        17.5,
        1100.0,
        1550.0,
        0.0,
        &mut meshes,
        &mut materials,
        false,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        50.0,
        17.5,
        1100.0,
        1625.0,
        0.0,
        &mut meshes,
        &mut materials,
        false,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        50.0,
        17.5,
        1100.0,
        1700.0,
        0.0,
        &mut meshes,
        &mut materials,
        false,
        &asset_server,
    );

    //vorletzter
    create_chunk(
        &mut commands,
        50.0,
        17.5,
        2750.0,
        840.0,
        0.0,
        &mut meshes,
        &mut materials,
        false,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        50.0,
        17.5,
        2750.0,
        915.0,
        0.0,
        &mut meshes,
        &mut materials,
        false,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        50.0,
        17.5,
        2750.0,
        990.0,
        0.0,
        &mut meshes,
        &mut materials,
        false,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        50.0,
        17.5,
        2750.0,
        1065.0,
        0.0,
        &mut meshes,
        &mut materials,
        false,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        50.0,
        17.5,
        2750.0,
        1140.0,
        0.0,
        &mut meshes,
        &mut materials,
        false,
        &asset_server,
    );

    //last
    create_chunk(
        &mut commands,
        50.0,
        17.5,
        3000.0,
        140.0,
        0.0,
        &mut meshes,
        &mut materials,
        false,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        50.0,
        17.5,
        3000.0,
        215.0,
        0.0,
        &mut meshes,
        &mut materials,
        false,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        50.0,
        17.5,
        3000.0,
        290.0,
        0.0,
        &mut meshes,
        &mut materials,
        false,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        50.0,
        17.5,
        3000.0,
        365.0,
        0.0,
        &mut meshes,
        &mut materials,
        false,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        50.0,
        17.5,
        3000.0,
        435.0,
        0.0,
        &mut meshes,
        &mut materials,
        false,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        50.0,
        17.5,
        3000.0,
        510.0,
        0.0,
        &mut meshes,
        &mut materials,
        false,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        50.0,
        17.5,
        3000.0,
        585.0,
        0.0,
        &mut meshes,
        &mut materials,
        false,
        &asset_server,
    );
    create_chunk(
        &mut commands,
        50.0,
        17.5,
        3000.0,
        660.0,
        0.0,
        &mut meshes,
        &mut materials,
        false,
        &asset_server,
    );
}

#[warn(dead_code)]
fn spawn_fountain(mut commands: Commands) {
    let fountain_offset = Transform::from_xyz(100.0, 100.0, 0.0);
    commands
        .spawn()
        .insert(Fountain)
        .insert_bundle(TransformBundle::from(fountain_offset));

    let base_offset = Transform::from_xyz(1500.0, 500.0, 0.0);
    commands
        .spawn()
        .insert(Base)
        .insert_bundle(TransformBundle::from(base_offset));
}
