use bevy::color::palettes::css::CRIMSON;
use bevy::ecs::component::Mutable;
use bevy::prelude::*;

use crate::components::SelectedOption;
use crate::constants::*;
use crate::resources::{DisplayQuality, Volume};
use crate::states::AppState;

use super::{icon_button, text_button};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // At start, the menu is not enabled. This will be changed in `menu_setup` when
            // entering the `GameState::Menu` state.
            // Current screen in the menu is handled by an independent state from `GameState`
            .init_state::<MenuState>()
            .add_systems(OnEnter(AppState::Menu), setup_menu)
            // Systems to handle the main menu screen
            .add_systems(OnEnter(MenuState::Main), setup_main_menu)
            // Systems to handle the settings menu screen
            .add_systems(OnEnter(MenuState::Settings), setup_settings_menu)
            // Systems to handle the display settings screen
            .add_systems(OnEnter(MenuState::SettingsDisplay), setup_display_settings_menu)
            .add_systems(
                Update,
                (setting_button::<DisplayQuality>.run_if(in_state(MenuState::SettingsDisplay)),),
            )
            // Systems to handle the sound settings screen
            .add_systems(OnEnter(MenuState::SettingsSound), setup_sound_settings_menu)
            .add_systems(
                Update,
                setting_button::<Volume>.run_if(in_state(MenuState::SettingsSound)),
            )
            // Common systems to all screens that handles buttons behavior
            .add_systems(Update, menu_action.run_if(in_state(AppState::Menu)));
    }
}

#[derive(Component)]
struct Setting<T>(T);

// All actions that can be triggered from a button click
#[derive(Component)]
enum MenuButtonAction {
    Play,
    Settings,
    SettingsDisplay,
    SettingsSound,
    BackToMainMenu,
    BackToSettings,
    Exit,
}

// This system updates the settings when a new value for a setting is selected, and marks
// the button as the one currently selected
#[allow(clippy::type_complexity)]
fn setting_button<T: Resource<Mutability = Mutable> + Component + PartialEq + Copy>(
    interaction_query: Query<(&Interaction, &Setting<T>, Entity), (Changed<Interaction>, With<Button>)>,
    selected_query: Single<(Entity, &mut BackgroundColor), With<SelectedOption>>,
    mut commands: Commands,
    mut setting: ResMut<T>,
) {
    let (previous_button, mut previous_button_color) = selected_query.into_inner();
    for (interaction, button_setting, entity) in &interaction_query {
        if *interaction == Interaction::Pressed && *setting != button_setting.0 {
            *previous_button_color = BUTTON_NORMAL.into();
            commands.entity(previous_button).remove::<SelectedOption>();
            commands.entity(entity).insert(SelectedOption);
            *setting = button_setting.0;
        }
    }
}

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuState {
    Main,
    Settings,
    SettingsDisplay,
    SettingsSound,
    #[default]
    Disabled,
}

fn setup_menu(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let right_icon = asset_server.load(ASSET_PATH_IMAGE_RIGHT);
    let wrench_icon = asset_server.load(ASSET_PATH_IMAGE_WRENCH);
    let exit_icon = asset_server.load(ASSET_PATH_IMAGE_EXIT_RIGHT);

    commands.spawn((
        DespawnOnExit(MenuState::Main),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(CRIMSON.into()),
            children![
                // Display the game name
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
                icon_button(right_icon, "New Game", MenuButtonAction::Play),
                icon_button(wrench_icon, "Settings", MenuButtonAction::Settings),
                icon_button(exit_icon, "Exit", MenuButtonAction::Exit),
            ]
        )],
    ));

    commands.spawn((DespawnOnExit(AppState::Menu), Camera2d));
}

fn setup_settings_menu(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(MenuState::Settings),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(CRIMSON.into()),
            Children::spawn(SpawnIter(
                [
                    (MenuButtonAction::SettingsDisplay, "Display"),
                    (MenuButtonAction::SettingsSound, "Sound"),
                    (MenuButtonAction::BackToMainMenu, "Back"),
                ]
                .into_iter()
                .map(move |(action, text)| { text_button(text, action) })
            ))
        )],
    ));
}

fn setup_display_settings_menu(mut commands: Commands, display_quality: Res<DisplayQuality>) {
    fn button_node() -> Node {
        Node {
            width: px(200),
            height: px(65),
            margin: UiRect::all(px(20)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        }
    }
    fn button_text_style() -> impl Bundle {
        (
            TextFont {
                font_size: FontSize::Px(33.0),
                ..default()
            },
            TextColor(COLOR_TEXT),
        )
    }

    let display_quality = *display_quality;
    commands.spawn((
        DespawnOnExit(MenuState::SettingsDisplay),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(CRIMSON.into()),
            children![
                // Create a new `Node`, this time not setting its `flex_direction`. It will
                // use the default value, `FlexDirection::Row`, from left to right.
                (
                    Node {
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(CRIMSON.into()),
                    Children::spawn((
                        // Display a label for the current setting
                        Spawn((Text::new("Display Quality"), button_text_style())),
                        SpawnWith(move |parent: &mut ChildSpawner| {
                            for quality_setting in [DisplayQuality::Low, DisplayQuality::Medium, DisplayQuality::High] {
                                let mut entity = parent.spawn((
                                    Button,
                                    Node {
                                        width: px(150),
                                        height: px(65),
                                        ..button_node()
                                    },
                                    BackgroundColor(BUTTON_NORMAL),
                                    Setting(quality_setting),
                                    children![(Text::new(format!("{quality_setting:?}")), button_text_style(),)],
                                ));
                                if display_quality == quality_setting {
                                    entity.insert(SelectedOption);
                                }
                            }
                        })
                    ))
                ),
                // Display the back button to return to the settings screen
                (
                    Button,
                    button_node(),
                    BackgroundColor(BUTTON_NORMAL),
                    MenuButtonAction::BackToSettings,
                    children![(Text::new("Back"), button_text_style())]
                )
            ]
        )],
    ));
}

fn setup_sound_settings_menu(mut commands: Commands, volume: Res<Volume>) {
    let button_node = Node {
        width: px(200),
        height: px(65),
        margin: UiRect::all(px(20)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = (
        TextFont {
            font_size: FontSize::Px(33.0),
            ..default()
        },
        TextColor(COLOR_TEXT),
    );

    let volume = *volume;
    let button_node_clone = button_node.clone();
    commands.spawn((
        DespawnOnExit(MenuState::SettingsSound),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(CRIMSON.into()),
            children![
                (
                    Node {
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(CRIMSON.into()),
                    Children::spawn((
                        Spawn((Text::new("Volume"), button_text_style.clone())),
                        SpawnWith(move |parent: &mut ChildSpawner| {
                            for volume_setting in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
                                let mut entity = parent.spawn((
                                    Button,
                                    Node {
                                        width: px(30),
                                        height: px(65),
                                        ..button_node_clone.clone()
                                    },
                                    BackgroundColor(BUTTON_NORMAL),
                                    Setting(Volume(volume_setting)),
                                ));
                                if volume == Volume(volume_setting) {
                                    entity.insert(SelectedOption);
                                }
                            }
                        })
                    ))
                ),
                (
                    Button,
                    button_node,
                    BackgroundColor(BUTTON_NORMAL),
                    MenuButtonAction::BackToSettings,
                    children![(Text::new("Back"), button_text_style)]
                )
            ]
        )],
    ));
}

#[allow(clippy::type_complexity)]
fn menu_action(
    interaction_query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_writer: MessageWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Exit => {
                    app_exit_writer.write(AppExit::Success);
                }
                MenuButtonAction::Play => {
                    app_state.set(AppState::Game);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
                MenuButtonAction::SettingsDisplay => {
                    menu_state.set(MenuState::SettingsDisplay);
                }
                MenuButtonAction::SettingsSound => {
                    menu_state.set(MenuState::SettingsSound);
                }
                MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
                MenuButtonAction::BackToSettings => {
                    menu_state.set(MenuState::Settings);
                }
            }
        }
    }
}
