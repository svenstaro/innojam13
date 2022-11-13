use bevy::{math::vec3, prelude::*};

use crate::{AppState, WORLD_SIZE};

#[derive(Component)]
struct BuildStateCountdown {
    pub countdown: f64,
}

#[derive(Component)]
pub struct BuildStateText;

pub struct BuildStatePlugin;

impl Plugin for BuildStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Build).with_system(build_system))
            .add_system_set(SystemSet::on_enter(AppState::Build).with_system(setup))
            .add_system_set(SystemSet::on_exit(AppState::Build).with_system(cleanup));
    }
}

fn build_system(
    mut commands: Commands,
    time: Res<Time>,
    mut timer_q: Query<(&mut BuildStateCountdown, &mut Text)>,
    mut state: ResMut<State<AppState>>,
) {
    let (mut countdown, mut text) = timer_q.single_mut();
    countdown.countdown -= time.delta_seconds_f64();
    text.sections[0].value = format!("{:.3}s", countdown.countdown);

    if countdown.countdown <= 0.0 {
        state
            .set(AppState::Attack)
            .expect("Couldn't switch state to Attack");
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Oswald-SemiBold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 250.0,
        color: Color::WHITE,
    };

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section("Build phase", text_style)
                .with_alignment(TextAlignment::CENTER),
            transform: Transform {
                translation: vec3(WORLD_SIZE.0 / 2.0, WORLD_SIZE.1 - 400.0, 0.97),
                ..default()
            },
            ..default()
        })
        .insert(BuildStateText);

    let text_style = TextStyle {
        font,
        font_size: 150.0,
        color: Color::WHITE,
    };

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section("lol", text_style).with_alignment(TextAlignment::TOP_LEFT),
            transform: Transform {
                translation: vec3(WORLD_SIZE.0 / 2.0 - 150.0, WORLD_SIZE.1 - 600.0, 0.97),
                ..default()
            },
            ..default()
        })
        .insert(BuildStateCountdown { countdown: 20.0 });
}

fn cleanup(
    mut commands: Commands,
    mut timer_q: Query<(Entity, &mut BuildStateCountdown)>,
    mut text_q: Query<(Entity, &mut BuildStateText)>,
) {
    let (entity, _) = timer_q.single_mut();
    commands.entity(entity).despawn_recursive();

    let (entity, _) = text_q.single_mut();
    commands.entity(entity).despawn_recursive();
}
