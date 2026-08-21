use bevy::input::InputSystems;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions};

use crate::game_plugin::constants::MOUSE_SENSITIVITY;
use crate::game_plugin::events::{ContinueKeyEvent, InteractionKeyEvent};
use crate::game_plugin::resources::{PlayerLookInput, PlayerMovementInput};
use crate::game_plugin::states::GameState;
use crate::states::AppState;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            handle_input.after(InputSystems).run_if(
                in_state(GameState::Playing)
                    .or_else(in_state(GameState::InDialog))
                    .or_else(in_state(GameState::Paused)),
            ),
        )
        .add_systems(Update, toggle_cursor.run_if(in_state(AppState::Game)))
        .add_systems(OnExit(AppState::Game), cleanup_input);
    }
}

fn handle_input(
    mut commands: Commands,
    mut keyboard: ResMut<ButtonInput<KeyCode>>,
    mut look: ResMut<PlayerLookInput>,
    mut movement: ResMut<PlayerMovementInput>,
    mut mouse_events: MessageReader<MouseMotion>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut in_dialog: Local<bool>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        if game_state.get().is_paused() {
            next_game_state.set(if *in_dialog {
                GameState::InDialog
            } else {
                GameState::Playing
            });
        } else {
            *in_dialog = game_state.get().is_in_dialog();

            next_game_state.set(GameState::Paused);
        }

        return;
    }

    if game_state.get().is_paused() {
        keyboard.clear();
        mouse_events.clear();

        return;
    }

    if game_state.get().is_in_dialog() {
        mouse_events.clear();

        if keyboard.just_pressed(KeyCode::Enter) {
            commands.trigger(ContinueKeyEvent);
        }

        return;
    }

    if keyboard.just_pressed(KeyCode::KeyE) {
        commands.trigger(InteractionKeyEvent);
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

fn toggle_cursor(game_state: Res<State<GameState>>, mut cursor_options: Single<&mut CursorOptions>) {
    if game_state.get().is_playing() {
        cursor_options.visible = false;
        cursor_options.grab_mode = CursorGrabMode::Locked;
    } else {
        cursor_options.visible = true;
        cursor_options.grab_mode = CursorGrabMode::None;
    }
}

fn cleanup_input(mut keyboard: ResMut<ButtonInput<KeyCode>>, mut mouse_events: MessageReader<MouseMotion>) {
    keyboard.clear();
    mouse_events.clear();
}
