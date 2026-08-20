use bevy::prelude::*;

#[derive(Component)]
pub struct DelayTimer(pub Timer);

impl DelayTimer {
    pub fn new(seconds: f32) -> Self {
        Self(Timer::from_seconds(seconds, TimerMode::Once))
    }
}

#[derive(Clone, Component, Default)]
pub enum Door {
    #[default]
    Locked,
    MapLimit,
    Knockable,
    Closed,
    Open,
}

impl Door {
    pub fn is_open(&self) -> bool {
        matches!(self, Self::Open)
    }

    pub fn toggle_open(&mut self) {
        *self = if self.is_open() { Self::Closed } else { Self::Open };
    }
}
