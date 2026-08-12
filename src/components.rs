use bevy::prelude::*;

#[derive(Component)]
pub struct Door {
    pub is_open: bool,
    pub close_graph: Handle<AnimationGraph>,
    pub close_node_index: AnimationNodeIndex,
    pub open_graph: Handle<AnimationGraph>,
    pub open_node_index: AnimationNodeIndex,
}

impl Door {
    pub fn new(
        close_graph: Handle<AnimationGraph>,
        close_node_index: AnimationNodeIndex,
        open_graph: Handle<AnimationGraph>,
        open_node_index: AnimationNodeIndex,
    ) -> Self {
        Self {
            is_open: false,
            close_graph,
            close_node_index,
            open_graph,
            open_node_index,
        }
    }
}

#[derive(Component)]
pub struct HelpOverlay;

#[derive(Component)]
pub struct HelpOverlayText;

#[derive(Component)]
pub struct LoadingOverlay;

#[derive(Component)]
pub struct SelectedOption;
