use attack_state::AttackStatePlugin;
use bevy::{audio::AudioPlugin, math::vec3, prelude::*};

use bevy_easings::EasingsPlugin;
use bevy_rapier2d::prelude::*;
use build_state::BuildStatePlugin;
use enemy::{EnemyPlugin, SpawnWaveEvent};
use game_state::{AppState, GameStatePlugin};
use main_menu::MainMenuPlugin;

use gadget::GadgetPlugin;
use input::InputPlugin as GameInputPlugin;

use level::LevelPlugin;
use pathfinding::PathfindingPlugin;

mod attack_state;
mod build_state;
mod enemy;
mod gadget;
mod game_state;
mod input;
mod level;
mod main_menu;
mod pathfinding;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
pub struct MainCamera;

const WORLD_SIZE: (f32, f32) = (3400.0, 2000.0);
const PIXELS_PER_METER: f32 = 100.0;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .add_event::<SpawnWaveEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(PathfindingPlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(GadgetPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(GameInputPlugin)
        .add_plugin(EasingsPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(GameStatePlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(BuildStatePlugin)
        .add_plugin(AttackStatePlugin)
        .add_startup_system(setup_graphics)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle {
            transform: Transform::from_translation(vec3(
                WORLD_SIZE.0 / 2.0,
                WORLD_SIZE.1 / 2.0,
                2.0,
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
