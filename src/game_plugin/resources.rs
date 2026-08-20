use std::fmt::Display;

use bevy::prelude::*;

#[derive(Clone)]
pub enum GameCharacter {
    Player,
}

impl Display for GameCharacter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Player => write!(f, "You"),
        }
    }
}

#[derive(Clone)]
pub struct DialogBoxMessage {
    pub character: String,
    pub content: String,
}

impl DialogBoxMessage {
    pub fn new<C: ToString>(character: C, content: &str) -> Self {
        Self {
            character: character.to_string(),
            content: content.to_owned(),
        }
    }

    pub fn player(content: &str) -> Self {
        Self::new(GameCharacter::Player, content)
    }
}

#[derive(Default, Resource)]
pub struct DialogBoxMessages(pub Vec<DialogBoxMessage>);

#[derive(Resource)]
pub struct DoorAnimations {
    pub close_graph_handle: Handle<AnimationGraph>,
    pub close_node_index: AnimationNodeIndex,
    pub open_graph_handle: Handle<AnimationGraph>,
    pub open_node_index: AnimationNodeIndex,
}

impl DoorAnimations {
    pub fn new(
        close_graph_handle: Handle<AnimationGraph>,
        close_node_index: AnimationNodeIndex,
        open_graph_handle: Handle<AnimationGraph>,
        open_node_index: AnimationNodeIndex,
    ) -> Self {
        Self {
            close_graph_handle,
            close_node_index,
            open_graph_handle,
            open_node_index,
        }
    }
}

// A resource that holds the current loading data.
#[derive(Resource, Default)]
pub struct LoadingData {
    // This will hold the currently unloaded/loading assets.
    pub assets: Vec<UntypedHandle>,
    pub loaded_assets_count: usize,
    pub pipelines_ready: bool,
}

/// Mouse input vector
#[derive(Default, Resource, Deref, DerefMut)]
pub struct PlayerLookInput(Vec2);

/// Keyboard input vector
#[derive(Default, Resource, Deref, DerefMut)]
pub struct PlayerMovementInput(Vec3);

#[derive(Resource)]
pub struct PlayerSounds {
    pub knock_bundle: (AudioPlayer, PlaybackSettings),
    pub step_bundle: (AudioPlayer, PlaybackSettings),
}
