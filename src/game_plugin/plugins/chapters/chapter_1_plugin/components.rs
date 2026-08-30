use std::fmt::Display;

use bevy::ecs::component::Component;

#[derive(Clone, Component, Default, Eq, PartialEq)]
pub enum ChapterCharacter {
    #[default]
    None,
    Neighbor301,
    Neighbor302,
    Neighbor303,
    Neighbor305,
    Neighbor306,
    Neighbor308,
}

impl Display for ChapterCharacter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Neighbor301 => write!(f, "Neighbor on 301"),
            Self::Neighbor302 => write!(f, "Neighbor on 302"),
            Self::Neighbor303 => write!(f, "Neighbor on 303"),
            Self::Neighbor305 => write!(f, "Neighbor on 305"),
            Self::Neighbor306 => write!(f, "Neighbor on 306"),
            Self::Neighbor308 => write!(f, "Neighbor on 308"),
        }
    }
}

#[derive(Clone, Component)]
pub enum ChapterObject {
    DoorStairs,
    Door301,
    Door302,
    Door303,
    Door304,
    Door305,
    Door306,
    Door307,
    Door308,
}
