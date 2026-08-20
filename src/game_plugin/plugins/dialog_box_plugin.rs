use bevy::color::palettes::css::GRAY;
use bevy::prelude::*;

use crate::game_plugin::events::{ContinueEvent, DialogBoxEvent};
use crate::game_plugin::resources::DialogBoxMessages;
use crate::game_plugin::states::GameState;
use crate::states::AppState;

use self::components::{DialogBox, DialogBoxCharacter, DialogBoxContent, DialogBoxContinueButton};

pub struct DialogBoxPlugin;

impl Plugin for DialogBoxPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DialogBoxMessages::default())
            .add_systems(OnEnter(AppState::Game), setup_dialog_box)
            .add_systems(
                Update,
                toggle_dialog_box
                    .run_if(in_state(GameState::Playing).or_else(in_state(GameState::InDialog)))
                    .run_if(resource_changed::<DialogBoxMessages>),
            )
            .add_systems(Update, continue_button_action.run_if(in_state(GameState::InDialog)))
            .add_systems(OnExit(AppState::Game), cleanup_dialog_box)
            .add_observer(on_event)
            .add_observer(on_continue);
    }
}

fn setup_dialog_box(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(AppState::Game),
        DialogBox,
        Visibility::Hidden,
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::End,
            ..default()
        },
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                width: percent(100),
                margin: UiRect::all(px(24)),
                padding: UiRect::all(px(24)),
                row_gap: px(12.0),
                ..default()
            },
            BackgroundColor(GRAY.with_alpha(0.5).into()),
            children![
                (
                    Text::new(""),
                    TextFont {
                        font_size: FontSize::Px(26.0),
                        weight: FontWeight::BOLD,
                        ..default()
                    },
                    DialogBoxCharacter
                ),
                (
                    Node {
                        column_gap: px(12),
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    children![
                        (Text::new(""), DialogBoxContent),
                        (
                            Button,
                            Node {
                                padding: UiRect::all(px(12)),
                                ..default()
                            },
                            DialogBoxContinueButton,
                            children![Text::new("Continue"),]
                        )
                    ]
                ),
            ],
        )],
    ));
}

fn on_event(event: On<DialogBoxEvent>, mut commands: Commands) {
    commands.insert_resource(DialogBoxMessages(event.messages.clone()));
}

fn toggle_dialog_box(
    mut game_state: ResMut<NextState<GameState>>,
    messages: Res<DialogBoxMessages>,
    mut visibility_query: Query<&mut Visibility, With<DialogBox>>,
    mut character_query: Query<&mut Text, With<DialogBoxCharacter>>,
    mut content_query: Query<&mut Text, (With<DialogBoxContent>, Without<DialogBoxCharacter>)>,
) {
    let Ok(mut visibility) = visibility_query.single_mut() else {
        return;
    };

    let Ok(mut character) = character_query.single_mut() else {
        return;
    };

    let Ok(mut content) = content_query.single_mut() else {
        return;
    };

    if let Some(message) = messages.0.first() {
        game_state.set(GameState::InDialog);

        **character = message.character.to_string();
        **content = message.content.clone();
        *visibility = Visibility::default();
    } else {
        *visibility = Visibility::Hidden;
        **content = "".to_owned();
        **character = "".to_owned();

        game_state.set(GameState::Playing);
    }
}

fn continue_button_action(
    query: Query<&Interaction, (Changed<Interaction>, With<DialogBoxContinueButton>)>,
    mut messages: ResMut<DialogBoxMessages>,
) {
    if let Ok(interaction) = query.single()
        && *interaction == Interaction::Pressed
        && !messages.0.is_empty()
    {
        messages.0.remove(0);
    }
}

fn on_continue(_: On<ContinueEvent>, mut messages: ResMut<DialogBoxMessages>) {
    if !messages.0.is_empty() {
        messages.0.remove(0);
    }
}

fn cleanup_dialog_box(mut messages: ResMut<DialogBoxMessages>) {
    *messages = DialogBoxMessages::default();
}

mod components {
    use bevy::ecs::component::Component;

    #[derive(Component)]
    pub struct DialogBox;

    #[derive(Component)]
    pub struct DialogBoxContinueButton;

    #[derive(Component)]
    pub struct DialogBoxCharacter;

    #[derive(Component)]
    pub struct DialogBoxContent;
}
