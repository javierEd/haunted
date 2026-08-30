use bevy::state::state::States;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum ChapterState {
    #[default]
    None,
    One,
    Two,
}

impl ChapterState {
    pub fn is_none(&self) -> bool {
        matches!(self, ChapterState::None)
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GameState {
    #[default]
    Loading,
    Playing,
    InDialog,
    InLockPicking,
    Paused,
    Over,
}

impl GameState {
    pub fn is_loading(&self) -> bool {
        matches!(self, GameState::Loading)
    }

    pub fn is_playing(&self) -> bool {
        matches!(self, GameState::Playing)
    }

    pub fn is_paused(&self) -> bool {
        matches!(self, GameState::Paused)
    }
}
