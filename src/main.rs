use bevy::{
    math::{vec2, vec3},
    prelude::*,
    render::camera::RenderTarget,
    sprite::MaterialMesh2dBundle,
};

use bevy_easings::EasingsPlugin;
use bevy_rapier2d::prelude::*;
use enemy::{EnemyPlugin, SpawnWaveEvent};

use gadget::GadgetPlugin;
use input::InputPlugin as GameInputPlugin;

use level::{Fountain, LevelPlugin};
use pathfinding::{PathfindingAgent, PathfindingPlugin};

mod enemy;
mod level;
mod pathfinding;

mod gadget;
mod input;


#[derive(Component)]
pub struct MainCamera;

const PIXELS_PER_METER: f32 = 100.0;

const WORLD_SIZE: (f32, f32) = (3400.0, 2000.0);

fn main() {
    App::new()
        .add_event::<SpawnWaveEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EasingsPlugin)
        .add_plugin(PathfindingPlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(GadgetPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(GameInputPlugin)
        .add_system(bevy::window::close_on_esc)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        // Input handler systems.
        
        
        // Event reactions.
        .run();
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





