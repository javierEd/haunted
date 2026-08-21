use bevy::color::palettes::css::{BLACK, CRIMSON};
use bevy::prelude::*;

use crate::constants::{ASSET_PATH_IMAGE_EXIT_RIGHT, COLOR_TEXT};
use crate::game_plugin::states::GameState;
use crate::helpers::icon_button;
use crate::states::AppState;

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup_game_over_menu)
            .add_systems(
                Update,
                show_game_over_menu
                    .run_if(state_changed::<GameState>)
                    .run_if(in_state(GameState::Over)),
            )
            .add_systems(Update, (game_over_menu_action).run_if(in_state(GameState::Over)));
    }
}

#[derive(Component)]
struct GameOverMenu;

#[derive(Component)]
enum GameOverMenuAction {
    Abandom,
}

fn setup_game_over_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let exit_icon = asset_server.load(ASSET_PATH_IMAGE_EXIT_RIGHT);

    // Game over menu
    commands.spawn((
        DespawnOnExit(AppState::Game),
        GameOverMenu,
        Visibility::Hidden,
        Node {
            width: percent(100.0),
            height: percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(BLACK.into()),
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
                    Text::new("Game Over"),
                    TextFont {
                        font_size: FontSize::Px(42.0),
                        ..default()
                    },
                    Node {
                        margin: UiRect::all(px(30)),
                        ..default()
                    },
                ),
                icon_button(exit_icon, "Main Menu", GameOverMenuAction::Abandom),
            ]
        )],
    ));
}

fn show_game_over_menu(mut query: Query<&mut Visibility, With<GameOverMenu>>) {
    for mut visibility in &mut query {
        *visibility = Visibility::default();
    }
}

fn game_over_menu_action(
    interaction_query: Query<(&Interaction, &GameOverMenuAction), (Changed<Interaction>, With<Button>)>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                GameOverMenuAction::Abandom => {
                    app_state.set(AppState::Menu);
                }
            }
        }
    }
}
