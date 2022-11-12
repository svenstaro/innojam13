use bevy::{math::vec3, prelude::*};

use bevy_easings::EasingsPlugin;
use bevy_rapier2d::prelude::*;
use enemy::{EnemyPlugin, SpawnWaveEvent};
use main_menu::MainMenuPlugin;

use gadget::GadgetPlugin;
use input::InputPlugin as GameInputPlugin;

use level::LevelPlugin;
use pathfinding::PathfindingPlugin;

mod enemy;
mod gadget;
mod input;
mod level;
mod main_menu;
mod pathfinding;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    InGame,
}

#[derive(Component)]
pub struct MainCamera;

const WORLD_SIZE: (f32, f32) = (3400.0, 2000.0);
const PIXELS_PER_METER: f32 = 100.0;

fn main() {
    App::new()
        .add_event::<SpawnWaveEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(MainMenuPlugin)
        .add_plugin(PathfindingPlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(GadgetPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(GameInputPlugin)
        .add_plugin(EasingsPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        // Input handler systems.
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
            transform: Transform::from_translation(vec3(
                WORLD_SIZE.0 / 2.0,
                WORLD_SIZE.1 / 2.0,
                0.0,
            )),
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
