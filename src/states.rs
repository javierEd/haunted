use bevy::prelude::*;

// Enum that will be used as a global state for the app
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    Splash,
    Menu,
    Game,
}

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    Playing,
    Paused,
    Over,
}

impl GameState {
    pub fn is_paused(&self) -> bool {
        self == &GameState::Paused
    }
}
