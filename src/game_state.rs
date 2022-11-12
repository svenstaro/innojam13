use bevy::prelude::{App, Plugin};

pub struct GameStatePlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    Build,
    Attack,
    GameOver,
}

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::MainMenu);
    }
}
