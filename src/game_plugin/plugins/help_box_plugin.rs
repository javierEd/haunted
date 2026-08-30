use bevy::camera::visibility::VisibleEntities;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::game_plugin::components::{
    AfterInteractionTimer, AfterKnockTimer, Door, DoorInteraction, DoorStatus, LightSwitch, Talk,
};
use crate::game_plugin::helpers::{ComponentQueryTrait, QueryTrait};
use crate::game_plugin::states::{ChapterState, GameState};
use crate::states::AppState;

use self::components::InteractionHelpBox;

pub struct HelpBoxPlugin;

impl Plugin for HelpBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup_help_box).add_systems(
            Update,
            toggle_interaction_help_box
                .run_if(in_state(AppState::Game))
                .run_if(not(in_state(ChapterState::None))),
        );
    }
}

fn setup_help_box(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(AppState::Game),
        Node {
            width: percent(100.0),
            height: percent(100.0),
            row_gap: px(12.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(InteractionHelpBox, Visibility::Hidden, Text::new("")),],
    ));
}

fn toggle_interaction_help_box(
    game_state: Res<State<GameState>>,
    player_query: Query<&Transform, With<KinematicCharacterController>>,
    camera_query: Query<&VisibleEntities, With<Camera>>,
    switch_query: Query<(Entity, &Transform), With<LightSwitch>>,
    talk_query: Query<(Entity, &Transform), With<Talk>>,
    door_query: Query<(Entity, &Door, &Transform), (Without<AfterInteractionTimer>, Without<AfterKnockTimer>)>,
    children_query: Query<&Children>,
    mut interaction_help_box_query: Query<(&mut Visibility, &mut Text), With<InteractionHelpBox>>,
) {
    let Ok((mut visibility, mut interaction_text)) = interaction_help_box_query.single_mut() else {
        return;
    };

    if !game_state.is_playing() {
        *visibility = Visibility::Hidden;
        **interaction_text = "".to_owned();

        return;
    }

    if let Some((_, _)) = switch_query.nearest(player_query, camera_query, children_query) {
        **interaction_text = "Press E to switch light".to_owned();
        *visibility = Visibility::default();

        return;
    }

    if let Some((_, _)) = talk_query.nearest(player_query, camera_query, children_query) {
        **interaction_text = "Press E to talk".to_owned();
        *visibility = Visibility::default();

        return;
    }

    if let Some((_, door, _)) = door_query.nearest(player_query, camera_query, children_query) {
        match door.interaction {
            DoorInteraction::Knock => {
                if !door.is_open() {
                    **interaction_text = "Press E to knock door".to_owned();
                    *visibility = Visibility::default();
                }
            }
            DoorInteraction::Open => {
                **interaction_text = match door.status {
                    DoorStatus::Open => "Press E to close door".to_owned(),
                    DoorStatus::Locked => "Press E to unlock door".to_owned(),
                    _ => "Press E to open door".to_owned(),
                };

                *visibility = Visibility::default();
            }
        }
    } else {
        *visibility = Visibility::Hidden;
        **interaction_text = "".to_owned();
    }
}

mod components {
    use bevy::ecs::component::Component;

    #[derive(Component)]
    pub struct InteractionHelpBox;
}
