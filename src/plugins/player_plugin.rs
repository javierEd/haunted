use bevy::camera::visibility::VisibleEntities;
use bevy::color::palettes::css::{BLACK, CRIMSON};
use bevy::input::InputSystems;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions};
use bevy_rapier3d::prelude::*;

use crate::components::{Door, HelpOverlay, HelpOverlayText, LoadingOverlay};
use crate::constants::*;
use crate::resources::{LookInput, MovementInput};
use crate::states::{AppState, GameState};

use super::{Vec3Trait, VisibleEntitiesTrait, icon_button};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MovementInput>()
            .init_resource::<LookInput>()
            .add_systems(OnEnter(AppState::Game), setup_player)
            .add_systems(
                PreUpdate,
                handle_input
                    .after(InputSystems)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(GameState::Playing).or_else(in_state(GameState::Paused))),
            )
            .add_systems(
                Update,
                player_look
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                FixedUpdate,
                player_movement
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                show_game_over_menu
                    .run_if(in_state(AppState::Game))
                    .run_if(state_changed::<GameState>)
                    .run_if(in_state(GameState::Over)),
            )
            .add_systems(
                Update,
                toggle_pause_menu
                    .run_if(in_state(AppState::Game))
                    .run_if(state_changed::<GameState>)
                    .run_if(in_state(GameState::Playing).or_else(in_state(GameState::Paused))),
            )
            .add_systems(
                Update,
                pause_menu_action
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(GameState::Paused)),
            )
            .add_systems(
                Update,
                (game_over_menu_action)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(GameState::Over)),
            );
    }
}

#[derive(Component)]
struct GameOverMenu;

#[derive(Component)]
struct PauseMenu;

#[derive(Component)]
enum GameOverMenuAction {
    Abandom,
}

#[derive(Component)]
enum PauseMenuAction {
    Abandom,
    Resume,
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut cursor_options: Single<&mut CursorOptions>,
) {
    cursor_options.visible = false;
    cursor_options.grab_mode = CursorGrabMode::Locked;

    commands
        .spawn((
            DespawnOnExit(AppState::Game),
            Transform::from_xyz(0.0, 2.0, 0.0),
            Visibility::default(),
            Collider::round_cylinder(0.85, 0.25, 0.0),
            KinematicCharacterController {
                custom_mass: Some(5.0),
                up: Vec3::Y,
                offset: CharacterLength::Absolute(0.01),
                slide: true,
                autostep: Some(CharacterAutostep {
                    max_height: CharacterLength::Relative(0.3),
                    min_width: CharacterLength::Relative(0.5),
                    include_dynamic_bodies: false,
                }),
                // Don’t allow climbing slopes larger than 45 degrees.
                max_slope_climb_angle: 45.0_f32.to_radians(),
                // Automatically slide down on slopes smaller than 30 degrees.
                min_slope_slide_angle: 30.0_f32.to_radians(),
                apply_impulse_to_dynamic_bodies: true,
                snap_to_ground: None,
                ..default()
            },
        ))
        .with_children(|parent| {
            // FPS Camera
            parent.spawn((
                DespawnOnExit(AppState::Game),
                Camera3d::default(),
                Transform::from_xyz(0.0, 0.70, -0.1).looking_at(Vec3::ZERO, Vec3::Y),
            ));
        });

    // Loading overlay
    commands.spawn((
        DespawnOnExit(AppState::Game),
        LoadingOverlay,
        Visibility::default(),
        Node {
            width: percent(100.0),
            height: percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(BLACK.into()),
        children![Text::new("Loading...")],
    ));

    // Help overlay
    commands.spawn((
        DespawnOnExit(AppState::Game),
        HelpOverlay,
        Visibility::Hidden,
        Node {
            width: percent(100.0),
            height: percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(Text::new(""), HelpOverlayText)],
    ));

    let right_icon = asset_server.load(ASSET_PATH_IMAGE_RIGHT);
    let exit_icon = asset_server.load(ASSET_PATH_IMAGE_EXIT_RIGHT);

    // Pause menu
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

#[allow(clippy::too_many_arguments)]
fn handle_input(
    mut commands: Commands,
    mut keyboard: ResMut<ButtonInput<KeyCode>>,
    mut movement: ResMut<MovementInput>,
    mut look: ResMut<LookInput>,
    mut mouse_events: MessageReader<MouseMotion>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    player_query: Query<&GlobalTransform, With<KinematicCharacterController>>,
    mut door_query: Query<(Entity, &mut Door, &GlobalTransform)>,
    children_query: Query<&Children>,
    mut animation_player_query: Query<(Entity, &mut AnimationPlayer)>,
    camera_query: Query<&VisibleEntities, With<Camera>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        if game_state.get().is_paused() {
            next_game_state.set(GameState::Playing);
        } else {
            next_game_state.set(GameState::Paused);
        }

        return;
    }

    if game_state.get().is_paused() {
        keyboard.clear();
        mouse_events.clear();

        return;
    }

    if keyboard.just_pressed(KeyCode::KeyE)
        && let Ok(player_transform) = player_query.single()
        && let Ok(camera_visible_entities) = camera_query.single()
        && let Some((door_entity, mut door, _)) = door_query
            .iter_mut()
            .filter(|(e, d, t)| {
                let door_translation = if d.is_open {
                    t.translation() + (t.forward() * 0.5)
                } else {
                    t.translation()
                };

                door_translation.is_near(&player_transform.translation())
                    && children_query
                        .iter_descendants(*e)
                        .any(|ce| camera_visible_entities.is_visible(ce))
            })
            .min_by(|(_, _, t1), (_, _, t2)| {
                t1.translation()
                    .distance(player_transform.translation())
                    .partial_cmp(&t2.translation().distance(player_transform.translation()))
                    .unwrap()
            })
    {
        door.is_open = !door.is_open;

        for child_entity in children_query.iter_descendants(door_entity) {
            if let Ok((entity, mut animation_player)) = animation_player_query.get_mut(child_entity) {
                if door.is_open {
                    commands
                        .entity(entity)
                        .insert(AnimationGraphHandle(door.open_graph.clone()));
                    animation_player.start(door.open_node_index);
                } else {
                    commands
                        .entity(entity)
                        .insert(AnimationGraphHandle(door.close_graph.clone()));
                    animation_player.start(door.close_node_index);
                }
            }
        }
    }

    if keyboard.pressed(KeyCode::KeyW) {
        movement.z -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        movement.z += 1.0
    }
    if keyboard.pressed(KeyCode::KeyA) {
        movement.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        movement.x += 1.0
    }
    **movement = movement.normalize_or_zero();
    if keyboard.pressed(KeyCode::ShiftLeft) {
        **movement *= 2.0;
    }
    if keyboard.pressed(KeyCode::Space) {
        movement.y = 1.0;
    }

    for event in mouse_events.read() {
        look.x -= event.delta.x * MOUSE_SENSITIVITY;
        look.y -= event.delta.y * MOUSE_SENSITIVITY;
        look.y = look.y.clamp(-89.9, 89.9); // Limit pitch
    }
}

fn player_look(
    mut player: Query<&mut Transform, (With<KinematicCharacterController>, Without<Camera>)>,
    mut camera: Query<&mut Transform, With<Camera>>,
    input: Res<LookInput>,
) {
    let Ok(mut transform) = player.single_mut() else {
        return;
    };
    transform.rotation = Quat::from_axis_angle(Vec3::Y, input.x.to_radians());
    let Ok(mut transform) = camera.single_mut() else {
        return;
    };
    transform.rotation = Quat::from_axis_angle(Vec3::X, input.y.to_radians());
}

fn player_movement(
    time: Res<Time>,
    mut input: ResMut<MovementInput>,
    mut player: Query<(
        &mut Transform,
        &mut KinematicCharacterController,
        Option<&KinematicCharacterControllerOutput>,
    )>,
    mut game_state: ResMut<NextState<GameState>>,
    mut vertical_movement: Local<f32>,
    mut grounded_timer: Local<f32>,
) {
    let Ok((transform, mut controller, output)) = player.single_mut() else {
        return;
    };

    if transform.translation.y < -3.0 {
        game_state.set(GameState::Over);
        return;
    }

    let delta_time = time.delta_secs();
    // Retrieve input
    let mut movement = Vec3::new(input.x, 0.0, input.z) * MOVEMENT_SPEED;
    let jump_speed = input.y * JUMP_SPEED;
    // Clear input
    **input = Vec3::ZERO;
    // Check physics ground check
    if output.map(|o| o.grounded).unwrap_or(false) {
        *grounded_timer = GROUND_TIMER;
        *vertical_movement = 0.0;
    }
    // If we are grounded we can jump
    if *grounded_timer > 0.0 {
        *grounded_timer -= delta_time;
        // If we jump we clear the grounded tolerance
        if jump_speed > 0.0 {
            *vertical_movement = jump_speed;
            *grounded_timer = 0.0;
        }
    }
    movement.y = *vertical_movement;
    *vertical_movement += GRAVITY * delta_time * controller.custom_mass.unwrap_or(1.0);
    controller.translation = Some(transform.rotation * (movement * delta_time));
}

fn show_game_over_menu(
    mut query: Query<&mut Visibility, With<GameOverMenu>>,
    mut cursor_options: Single<&mut CursorOptions>,
) {
    cursor_options.visible = true;
    cursor_options.grab_mode = CursorGrabMode::None;

    for mut visibility in &mut query {
        *visibility = Visibility::default();
    }
}

fn toggle_pause_menu(
    game_state: Res<State<GameState>>,
    mut time: ResMut<Time<Virtual>>,
    mut query: Query<&mut Visibility, With<PauseMenu>>,
    mut cursor_options: Single<&mut CursorOptions>,
) {
    if game_state.is_paused() {
        time.pause();
        cursor_options.visible = true;
        cursor_options.grab_mode = CursorGrabMode::None;
    } else {
        time.unpause();
        cursor_options.visible = false;
        cursor_options.grab_mode = CursorGrabMode::Locked;
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

#[allow(clippy::type_complexity)]
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
