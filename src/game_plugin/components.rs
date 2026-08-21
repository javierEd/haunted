use bevy::prelude::*;

#[derive(Component)]
pub struct AfterInteractionTimer(pub Timer);

impl AfterInteractionTimer {
    pub fn new(seconds: f32) -> Self {
        Self(Timer::from_seconds(seconds, TimerMode::Once))
    }
}

#[derive(Component)]
pub struct AfterKnockTimer(pub Timer);

impl AfterKnockTimer {
    pub fn new(seconds: f32) -> Self {
        Self(Timer::from_seconds(seconds, TimerMode::Once))
    }
}

#[derive(Clone)]
pub enum DoorIteraction {
    Knock,
    Open,
}

#[derive(Clone)]
pub enum DoorStatus {
    Closed,
    Locked,
    MapLimit,
    Open,
}

#[derive(Clone, Component)]
pub struct Door {
    pub interaction: DoorIteraction,
    pub status: DoorStatus,
}

impl Door {
    pub const KNOCKABLE: Self = Self {
        interaction: DoorIteraction::Knock,
        status: DoorStatus::Locked,
    };
    pub const MAP_LIMIT: Self = Self {
        interaction: DoorIteraction::Open,
        status: DoorStatus::MapLimit,
    };
    pub const LOCKED: Self = Self {
        interaction: DoorIteraction::Open,
        status: DoorStatus::Locked,
    };
    pub const OPENABLE: Self = Self {
        interaction: DoorIteraction::Open,
        status: DoorStatus::Closed,
    };

    pub fn is_open(&self) -> bool {
        matches!(self.status, DoorStatus::Open)
    }

    pub fn set_is_open(&mut self, is_open: bool) {
        self.status = if is_open {
            DoorStatus::Open
        } else {
            match self.interaction {
                DoorIteraction::Open => DoorStatus::Closed,
                DoorIteraction::Knock => DoorStatus::Locked,
            }
        };
    }
}

#[derive(Component)]
pub struct Talk;
