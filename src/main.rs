use bevy::{prelude::*, render::camera::RenderTarget, sprite::MaterialMesh2dBundle};

use bevy_rapier2d::prelude::*;
use enemy::{EnemyPlugin, SpawnWaveEvent};
use level::LevelPlugin;
use pathfinding::PathfindingPlugin;

mod enemy;
mod main_menu;
use main_menu::MainMenuPlugin;

mod level;
mod pathfinding;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    InGame,
}

#[derive(Component)]
struct MainCamera;

const PIXELS_PER_METER: f32 = 100.0;

const WORLD_SIZE: (f32, f32) = (3400.0, 2000.0);

fn main() {
    App::new()
        .add_event::<SpawnWaveEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(MainMenuPlugin)
        .add_plugin(PathfindingPlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        // Input handler systems.
        .add_system(shoot_water)
        .add_system(debug_keymap)
        .add_state(AppState::MainMenu)
        .add_system(main_menu_controls)
        .run();
}

fn main_menu_controls(mut keys: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if *app_state.current() == AppState::MainMenu {
        if keys.just_pressed(KeyCode::Return) {
            app_state.set(AppState::InGame).unwrap();
            keys.reset(KeyCode::Return);
        }
    } else {
        if keys.just_pressed(KeyCode::Escape) {
            app_state.set(AppState::MainMenu).unwrap();
            // still needed?
            keys.reset(KeyCode::Escape);
        }
    }
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands
        .spawn_bundle(Camera2dBundle {
            projection: OrthographicProjection {
                scaling_mode: bevy::render::camera::ScalingMode::Auto {
                    min_width: WORLD_SIZE.0,
                    min_height: WORLD_SIZE.1,
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
