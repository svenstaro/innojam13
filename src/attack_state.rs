use bevy::{math::vec3, prelude::*};

use crate::{
    enemy::{Enemy, SpawnWaveEvent},
    gadget::Water,
    game_state::WaveControler,
    polishing_constants::ATTACK_COUNTDOWN,
    AppState, WORLD_SIZE,
};

#[derive(Component)]
struct AttackStateCountdown {
    pub countdown: f64,
}

#[derive(Component)]
pub struct AttackStateText;

pub struct AttackStatePlugin;

impl Plugin for AttackStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Attack).with_system(attack_system))
            .add_system_set(SystemSet::on_enter(AppState::Attack).with_system(setup))
            .add_system_set(SystemSet::on_exit(AppState::Attack).with_system(cleanup));
    }
}

fn attack_system(
    mut commands: Commands,
    time: Res<Time>,
    mut timer_q: Query<(&mut AttackStateCountdown, &mut Text)>,
    mut state: ResMut<State<AppState>>,
) {
    let (mut countdown, mut text) = timer_q.single_mut();
    countdown.countdown -= time.delta_seconds_f64();
    text.sections[0].value = format!("{:.3}s", countdown.countdown);

    if countdown.countdown <= 0.0 {
        state
            .set(AppState::Build)
            .expect("Couldn't switch state to Attack");
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spawn_wave_events: EventWriter<SpawnWaveEvent>,
    mut wave_controler: ResMut<WaveControler>,
) {
    spawn_wave_events.send(SpawnWaveEvent::new(wave_controler.wave_size));
    wave_controler.wave_size += 1;
    let font = asset_server.load("fonts/Oswald-SemiBold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 250.0,
        color: Color::WHITE,
    };

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section("Attack phase", text_style)
                .with_alignment(TextAlignment::CENTER),
            transform: Transform {
                translation: vec3(WORLD_SIZE.x - 350.0, WORLD_SIZE.y - 200.0, 0.97),
                ..default()
            },
            ..default()
        })
        .insert(AttackStateText);

    let text_style = TextStyle {
        font,
        font_size: 150.0,
        color: Color::WHITE,
    };

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section("lol", text_style).with_alignment(TextAlignment::TOP_LEFT),
            transform: Transform {
                translation: vec3(WORLD_SIZE.x - 280.0, WORLD_SIZE.y - 350.0, 0.97),
                ..default()
            },
            ..default()
        })
        .insert(AttackStateCountdown {
            countdown: ATTACK_COUNTDOWN,
        });
}

fn cleanup(
    mut commands: Commands,
    mut timer_q: Query<(Entity, &mut AttackStateCountdown)>,
    mut text_q: Query<(Entity, &mut AttackStateText)>,
    mut enemy_q: Query<Entity, With<Enemy>>,
    mut water_q: Query<Entity, With<Water>>,
) {
    let (entity, _) = timer_q.single_mut();
    commands.entity(entity).despawn_recursive();

    let (entity, _) = text_q.single_mut();

    commands.entity(entity).despawn_recursive();

    for entity in enemy_q.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for entity in water_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
