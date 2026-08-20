use bevy::camera::visibility::VisibleEntities;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::game_plugin::components::{DelayTimer, Door};
use crate::game_plugin::helpers::QueryTrait;
use crate::game_plugin::states::{ChapterState, GameState};
use crate::states::AppState;

use self::components::{HelpBox, HelpBoxText};

pub struct HelpBoxPlugin;

impl Plugin for HelpBoxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup_help_box).add_systems(
            Update,
            toggle_help_box
                .run_if(in_state(AppState::Game))
                .run_if(not(in_state(ChapterState::None))),
        );
    }
}

fn setup_help_box(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(AppState::Game),
        HelpBox,
        Visibility::Hidden,
        Node {
            width: percent(100.0),
            height: percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(Text::new(""), HelpBoxText)],
    ));
}

fn toggle_help_box(
    game_state: Res<State<GameState>>,
    player_query: Query<&Transform, With<KinematicCharacterController>>,
    camera_query: Query<&VisibleEntities, With<Camera>>,
    door_query: Query<(Entity, &Door, &Transform), Without<DelayTimer>>,
    children_query: Query<&Children>,
    mut help_box_query: Query<&mut Visibility, With<HelpBox>>,
    mut help_box_text_query: Query<&mut Text, With<HelpBoxText>>,
) {
    let Ok(mut help_box_visibility) = help_box_query.single_mut() else {
        return;
    };

    let Ok(mut help_box_text) = help_box_text_query.single_mut() else {
        return;
    };

    if game_state.is_playing()
        && let Some((_, door, _)) = door_query.nearest(player_query, camera_query, children_query)
    {
        **help_box_text = match door {
            Door::Knockable => "Press E to knock door".to_owned(),
            Door::Open => "Press E to close door".to_owned(),
            _ => "Press E to open door".to_owned(),
        };

        *help_box_visibility = Visibility::default();
    } else {
        *help_box_visibility = Visibility::Hidden;
        **help_box_text = "".to_owned();
    }
}

mod components {
    use bevy::ecs::component::Component;

    #[derive(Component)]
    pub struct HelpBox;

    #[derive(Component)]
    pub struct HelpBoxText;
}
