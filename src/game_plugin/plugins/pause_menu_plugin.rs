use bevy::color::palettes::css::CRIMSON;
use bevy::prelude::*;

use crate::constants::{ASSET_PATH_IMAGE_EXIT_RIGHT, ASSET_PATH_IMAGE_RIGHT, COLOR_TEXT};
use crate::game_plugin::states::GameState;
use crate::helpers::icon_button;
use crate::states::AppState;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup_pause_menu)
            .add_systems(
                Update,
                toggle_pause_menu.run_if(state_changed::<GameState>).run_if(
                    in_state(GameState::Playing)
                        .or_else(in_state(GameState::InDialog))
                        .or_else(in_state(GameState::Paused)),
                ),
            )
            .add_systems(Update, pause_menu_action.run_if(in_state(GameState::Paused)));
    }
}

#[derive(Component)]
struct PauseMenu;

#[derive(Component)]
enum PauseMenuAction {
    Abandom,
    Resume,
}

fn setup_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let right_icon = asset_server.load(ASSET_PATH_IMAGE_RIGHT);
    let exit_icon = asset_server.load(ASSET_PATH_IMAGE_EXIT_RIGHT);

    commands.spawn((
        DespawnOnExit(AppState::Game),
        PauseMenu,
        Visibility::Hidden,
        Node {
            width: percent(100.0),
            height: percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ZIndex(10),
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(CRIMSON.into()),
            children![
                (
                    Text::new("Haunted"),
                    TextFont {
                        font_size: FontSize::Px(67.0),
                        ..default()
                    },
                    TextColor(COLOR_TEXT),
                    Node {
                        margin: UiRect::all(px(30)),
                        ..default()
                    },
                ),
                (
                    Text::new("Paused"),
                    TextFont {
                        font_size: FontSize::Px(42.0),
                        ..default()
                    },
                    Node {
                        margin: UiRect::all(px(30)),
                        ..default()
                    },
                ),
                icon_button(right_icon.clone(), "Resume", PauseMenuAction::Resume),
                icon_button(exit_icon.clone(), "Main Menu", PauseMenuAction::Abandom),
            ]
        )],
    ));
}

fn toggle_pause_menu(
    game_state: Res<State<GameState>>,
    mut time: ResMut<Time<Virtual>>,
    mut query: Query<&mut Visibility, With<PauseMenu>>,
) {
    if game_state.is_paused() {
        time.pause();
    } else {
        time.unpause();
    }

    for mut visibility in &mut query {
        if game_state.get().is_paused() {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

#[allow(clippy::type_complexity)]
fn pause_menu_action(
    interaction_query: Query<(&Interaction, &PauseMenuAction), (Changed<Interaction>, With<Button>)>,
    mut game_state: ResMut<NextState<GameState>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                PauseMenuAction::Resume => {
                    game_state.set(GameState::Playing);
                }
                PauseMenuAction::Abandom => {
                    app_state.set(AppState::Menu);
                }
            }
        }
    }
}
