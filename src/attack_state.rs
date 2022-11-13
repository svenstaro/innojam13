use bevy::{math::vec3, prelude::*};

use crate::{AppState, WORLD_SIZE, enemy::SpawnWaveEvent, game_state::WaveControler};

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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut spawn_wave_events: EventWriter<SpawnWaveEvent>, mut wave_controler: ResMut<WaveControler>) {
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
                translation: vec3(WORLD_SIZE.0 / 2.0, WORLD_SIZE.1 - 400.0, 0.0),
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
                translation: vec3(WORLD_SIZE.0 / 2.0 - 150.0, WORLD_SIZE.1 - 600.0, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(AttackStateCountdown { countdown: 20.0 });
}

fn cleanup(
    mut commands: Commands,
    mut timer_q: Query<(Entity, &mut AttackStateCountdown)>,
    mut text_q: Query<(Entity, &mut AttackStateText)>,
) {
    let (entity, _) = timer_q.single_mut();
    commands.entity(entity).despawn_recursive();

    let (entity, _) = text_q.single_mut();
    commands.entity(entity).despawn_recursive();
}
