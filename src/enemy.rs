use std::time::Duration;

use bevy::{math::vec2, prelude::*};
use bevy_easings::{Ease, EaseFunction, EasingType};
use bevy_rapier2d::prelude::*;

pub struct EnemyPlugin;

use rand::Rng;

use crate::{
    game_state::AppState, level::{Fountain, LevelComponent}, pathfinding::PathfindingAgent,
    polishing_constants::ENEMY_STRENGTH, MainCamera,
};

#[derive(Debug, Clone)]
pub struct WaveConfig {
    count_remaining: u32,
    timer: Timer,
}

impl Default for WaveConfig {
    fn default() -> Self {
        Self {
            count_remaining: 3,
            timer: Timer::new(Duration::from_secs(2), true),
        }
    }
}

impl WaveConfig {
    pub fn new(count: u32) -> Self {
        WaveConfig {
            count_remaining: count,
            timer: Timer::new(Duration::from_secs(2), true),
        }
    }
}

#[derive(Debug, Default)]
pub struct SpawnWaveEvent {
    wave_cfg: WaveConfig,
}

impl SpawnWaveEvent {
    pub fn new(count: u32) -> Self {
        SpawnWaveEvent {
            wave_cfg: WaveConfig::new(count),
        }
    }
}

fn rand_f32(l: f32, u: f32) -> f32 {
    rand::thread_rng().gen_range(l..u)
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnWaveEvent>()
            .init_resource::<WaveConfig>()
            .add_system(spawn_new_wave_on_event)
            .add_system_set(SystemSet::on_update(AppState::Attack).with_system(check_for_spawn));
        // Enemy processes.
        // .add_system(fountain_spawns_things);
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
pub struct Enemy;

fn spawn_new_wave_on_event(
    mut spawn_wave_events: EventReader<SpawnWaveEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    fountain_query: Query<&Transform, With<Fountain>>,
    windows: Res<Windows>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // Play a sound once per frame if a collision occurred.

    for wave_ev in spawn_wave_events.iter() {
        let mut wave_cfg = wave_ev.wave_cfg.clone();
        let fountain_pos = fountain_query.single().translation;
        spawn_enemy_at(&mut commands, &asset_server, fountain_pos, 120.0);
        wave_cfg.count_remaining -= 1;
        commands.insert_resource(wave_cfg);
    }
}

fn check_for_spawn(
    mut commands: Commands,
    mut wave_cfg: ResMut<WaveConfig>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    fountain_query: Query<&Transform, With<Fountain>>,
) {
    wave_cfg.timer.tick(time.delta());
    if wave_cfg.count_remaining > 0 && wave_cfg.timer.just_finished() {
        let fountain_pos = fountain_query.single().translation;
        spawn_enemy_at(&mut commands, &asset_server, fountain_pos, 120.0);
        wave_cfg.count_remaining -= 1;
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
        .insert(PathfindingAgent::new(ENEMY_STRENGTH))
        .insert(LevelComponent)
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(1.0)),
                ..default()
            },
            texture: asset_server.load("enemies/grunt.png"),
            transform: Transform::from_scale(Vec3::new(size, size, 1.0)).with_translation(pos),
            ..default()
        })
        // what is this for?
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
