use bevy::prelude::*;

use crate::{level::Base, enemy::Enemy};

pub struct GameStatePlugin;

const KILL_DIST: f32 = 170.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Intro,
    MainMenu,
    // TODO: Settings,
    // Settings,
    // Credits,
    Build,
    Attack,
    GameOver,
}

pub struct WaveControler {
    pub wave_size: u32,

}

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::Intro)
        .add_startup_system(audio_system)
        .add_system(check_game_over)
        .insert_resource(WaveControler{ wave_size: 1 });
    }
}


fn check_game_over(mut commands: Commands, base_query: Query<&Transform, With<Base>>, enemy_query: Query<&Transform, With<Enemy>>, mut app_state: ResMut<State<AppState>>, mut wave_controler: ResMut<WaveControler>) {
    if base_query.is_empty() {
        println!("no base in scene");
        return;
    }
    let base_pos = base_query.single().translation;
    for enemy_trans in enemy_query.iter() {
        let enemy_pos = enemy_trans.translation;
        let enemy_dist = base_pos.distance(enemy_pos);
        dbg!(enemy_dist);
        if enemy_dist < KILL_DIST && *app_state.current() != AppState::GameOver{
            println!("gameover");
            wave_controler.wave_size = 1;
            app_state.set(AppState::Intro).unwrap();
        }
    }
}

fn audio_system(
    audio: Res<Audio>,
    app_state: Res<State<AppState>>,
    asset_server: Res<AssetServer>,
) {
    match *app_state.current() {
        AppState::Intro => {
            audio.play_with_settings(
                asset_server.load("music/menu-start.ogg"),
                PlaybackSettings {
                    repeat: false,
                    volume: 0.75,
                    speed: 1.0,
                },
            );

            audio.play_with_settings(
                asset_server.load("music/menu-loop.ogg"),
                PlaybackSettings {
                    repeat: true,
                    volume: 0.75,
                    speed: 1.0,
                },
            );
        }

        AppState::MainMenu => {
            audio.play_with_settings(
                asset_server.load("music/menu-loop.ogg"),
                PlaybackSettings {
                    repeat: true,
                    volume: 0.75,
                    speed: 1.0,
                },
            );
        }
        // AppState::Build => {
        //     audio.play_with_settings(
        //        ...
        //         PlaybackSettings {
        //             repeat: true,
        //             volume: 0.75,
        //             speed: 1.0,
        //         },
        //     );
        // }
        // AppState::Attack => {
        //     audio.play_with_settings(
        //         ...
        //         PlaybackSettings {
        //             repeat: true,
        //             volume: 0.75,
        //             speed: 1.0,
        //         },
        //     );
        // }
        _ => {}
    }
}
