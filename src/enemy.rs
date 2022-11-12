use std::time::Duration;

use bevy::{math::vec2, prelude::*};
use bevy_easings::{Ease, EaseFunction, EasingType};
use bevy_rapier2d::prelude::*;

pub struct EnemyPlugin;

use rand::Rng;

use crate::{level::Fountain, pathfinding::PathfindingAgent};

#[derive(Debug, Default)]
pub struct SpawnWaveEvent;

fn rand_f32(l: f32, u: f32) -> f32 {
    rand::thread_rng().gen_range(l..u)
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnWaveEvent>()
            .add_system(spawn_new_wave_on_event)

            // Enemy processes.
            .add_system(fountain_spawns_things);
    }
}

#[derive(Component, Debug, Default)]
enum EnemyType {
    #[default]
    Grunt,
    // Swimmer,
    // Digger,
    // Tank,
}

#[derive(Component, Debug, Default)]
struct Enemy;

fn spawn_new_wave_on_event(
    spawn_wave_events: EventReader<SpawnWaveEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut fountain_query: Query<&Transform, With<Fountain>>,
) {
    // Play a sound once per frame if a collision occurred.
    if spawn_wave_events.is_empty() {
        return;
    }
    // This prevents events staying active on the next frame.
    spawn_wave_events.clear();

    let base_pos = if let Some(fountain) = fountain_query.iter_mut().next().map(|x| x.clone()) {
        fountain.translation
    } else {
        Vec3::new(1000.0, 500.0, 0.0)
    };

    let wave_size = 10;
    for _ in 0..wave_size {
        let offset = Vec3::new(rand_f32(-50.0, 50.0), rand_f32(-50.0, 50.0), 0.0);
        let pos = base_pos + offset;
        spawn_enemy_at(&mut commands, &asset_server, pos, 120.0);
    }
}

fn fountain_spawns_things(
    mut fountain_query: Query<&Transform, With<Fountain>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if let Some(fountain) = fountain_query.iter_mut().next().map(|x| x.clone()) {
        if rand_f32(0.0, 1.0) > 0.95 {
            spawn_enemy_at(&mut commands, &asset_server, fountain.translation, 120.0);
        }
    }
}

fn spawn_enemy_at(commands: &mut Commands, asset_server: &Res<AssetServer>, pos: Vec3, size: f32) {
    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(CollisionGroups::new(
            Group::GROUP_1,
            Group::GROUP_1 | Group::GROUP_2,
        ))
        .insert(Damping {
            linear_damping: 0.90,
            angular_damping: 0.5,
        })
        .insert(ExternalForce {
            force: Vec2::new(0.0, 0.0),
            torque: 0.0,
        })
        .insert(Enemy)
        .insert(EnemyType::Grunt)
        .insert(PathfindingAgent::new(10.0))
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(1.0)),
                ..default()
            },
            texture: asset_server.load("enemies/grunt.png"),
            transform: Transform::from_scale(Vec3::new(size, size, 1.0)).with_translation(pos),
            ..default()
        })
        .insert(
            Sprite {
                custom_size: Some(Vec2::splat(1.0)),
                ..default()
            }
            .ease_to(
                Sprite {
                    custom_size: Some(vec2(1.3, 0.7)),
                    ..default()
                },
                EaseFunction::CubicInOut,
                EasingType::PingPong {
                    duration: Duration::from_millis(500),
                    pause: Some(Duration::from_millis(70)),
                },
            ),
        );
}
