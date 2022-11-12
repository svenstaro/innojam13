use bevy::{prelude::*, render::camera::RenderTarget, sprite::MaterialMesh2dBundle};

use bevy_rapier2d::prelude::*;
use level::{Fountain, LevelPlugin};
use pathfinding::{PathfindingAgent, PathfindingPlugin};
use rand::Rng;

mod level;
mod pathfinding;

#[derive(Debug, Default)]
struct SpawnWaveEvent;

#[derive(Component, Debug, Default)]
enum EnemyType {
    #[default]
    Grunt,
    Swimmer,
    Digger,
    Tank,
}

#[derive(Component, Debug, Default)]
struct Enemy;

#[derive(Component)]
struct MainCamera;

const PIXELS_PER_METER: f32 = 100.0;

fn main() {
    App::new()
        .add_event::<SpawnWaveEvent>()
        .add_plugin(PathfindingPlugin)
        .add_plugin(LevelPlugin)
        .add_plugins(DefaultPlugins)
        .add_system(bevy::window::close_on_esc)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        // Input handler systems.
        .add_system(shoot_water)
        .add_system(debug_keymap)
        // Event reactions.
        .add_system(spawn_new_wave_on_event)
        // Enemy processes.
        .add_system(fountain_spawns_things)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands
        .spawn_bundle(Camera2dBundle {
            projection: OrthographicProjection {
                scaling_mode: bevy::render::camera::ScalingMode::Auto {
                    min_width: 3400.0,
                    min_height: 2000.0,
                },
                ..default()
            },
            ..default()
        })
        .insert(MainCamera);
}

fn get_world_cursor_pos(
    windows: Res<Windows>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) -> Option<Vec2> {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_q.single();

    // get the window that the camera is displaying to (or the primary window)
    let window = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = window.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        Some(world_pos.truncate())
    } else {
        None
    }
}

fn shoot_water(
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    if buttons.pressed(MouseButton::Left) {
        if let Some(position) = get_world_cursor_pos(windows, camera_q) {
            commands
                .spawn()
                .insert(RigidBody::Dynamic)
                .insert(Collider::ball(0.5))
                .insert(Restitution::coefficient(0.1))
                .insert(ExternalImpulse {
                    impulse: Vec2::new(5.0, -5.0),
                    torque_impulse: 0.0,
                })
                .insert_bundle(MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
                    transform: Transform::from_xyz(position.x, position.y, 0.0)
                        .with_scale(Vec3::splat(10.)),
                    material: materials.add(ColorMaterial::from(Color::BLUE)),
                    ..default()
                });
        } else {
            // cursor is not inside the window
        }
    }
}

fn debug_keymap(keys: Res<Input<KeyCode>>, mut spawn_wave_events: EventWriter<SpawnWaveEvent>) {
    // Spawn next wave.
    if keys.pressed(KeyCode::N) {
        spawn_wave_events.send_default();
    }
}

fn rand_f32(l: f32, u: f32) -> f32 {
    rand::thread_rng().gen_range(l..u)
}

fn spawn_new_wave_on_event(
    spawn_wave_events: EventReader<SpawnWaveEvent>,
    windows: Res<Windows>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Play a sound once per frame if a collision occurred.
    if spawn_wave_events.is_empty() {
        return;
    }

    // This prevents events staying active on the next frame.
    spawn_wave_events.clear();

    let _window = windows.get_primary().unwrap();

    let wave_size = 10;

    for _ in 0..wave_size {
        let base_transform = Transform::from_xyz(0.0, 0.0, 0.0);
        let offset = Vec3::new(rand_f32(-50.0, 50.0), rand_f32(-50.0, 50.0), 0.0);
        let transform = base_transform.with_translation(base_transform.translation + offset);

        commands
            .spawn()
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(50.0))
            .insert(Damping {
                linear_damping: 0.90,
                angular_damping: 0.5,
            })
            .insert(ExternalForce {
                force: Vec2::new(0.0, 0.0),
                torque: 0.0,
            })
            .insert(PathfindingAgent::new(10.0))
            .insert_bundle(TransformBundle::from(transform))
            .insert(Enemy)
            .insert(EnemyType::Grunt)
            .insert(PathfindingAgent::new(10.0))
            .insert_bundle(SpriteBundle {
                texture: asset_server.load("enemies/grunt.png"),
                transform: Transform::from_scale(Vec3::new(0.5, 0.5, 1.0)),
                ..default()
            });
    }
}

fn fountain_spawns_things(
    mut fountain_query: Query<&Transform, With<Fountain>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let &fountain_transform = fountain_query.iter_mut().next().clone().unwrap();
    if rand_f32(0.0, 1.0) > 0.95 {
        commands
            .spawn()
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(50.0))
            .insert(Damping {
                linear_damping: 0.90,
                angular_damping: 0.5,
            })
            .insert(ExternalForce {
                force: Vec2::new(0.0, 0.0),
                torque: 0.0,
            })
            .insert_bundle(TransformBundle::from(fountain_transform))
            .insert(Enemy)
            .insert(EnemyType::Grunt)
            .insert(PathfindingAgent::new(10.0))
            .insert_bundle(SpriteBundle {
                texture: asset_server.load("enemies/grunt.png"),
                transform: Transform::from_scale(Vec3::new(0.5, 0.5, 1.0)),
                ..default()
            });
    }
}
