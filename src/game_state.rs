use bevy::prelude::*;

use crate::{level::Base, enemy::Enemy};

pub struct GameStatePlugin;

const KILL_DIST: f32 = 30.0;

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
        app.add_state(AppState::Intro);
        app.add_system(check_game_over);
        app.insert_resource(WaveControler{ wave_size: 1 });
    }
}


fn check_game_over(mut commands: Commands, base_query: Query<&Transform, With<Base>>, enemy_query: Query<&Transform, With<Enemy>>, mut app_state: ResMut<State<AppState>>) {
    if base_query.is_empty() {
        return;
    }
    let base_pos = base_query.single().translation;
    for enemy_trans in enemy_query.iter() {
        let enemy_pos = enemy_trans.translation;
        if base_pos.distance(enemy_pos) < KILL_DIST && *app_state.current() != AppState::GameOver{
            app_state.set(AppState::GameOver).unwrap();
        }
    }
}
