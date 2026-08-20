use bevy::state::state::States;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    Splash,
    Menu,
    Game,
}

impl AppState {
    pub fn is_game(&self) -> bool {
        matches!(self, AppState::Game)
    }
}
