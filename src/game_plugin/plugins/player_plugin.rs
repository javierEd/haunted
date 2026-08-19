use bevy::audio::Volume;
use bevy::input::InputSystems;
use bevy::input::mouse::MouseMotion;
use bevy::pbr::ContactShadows;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions};
use bevy_rapier3d::prelude::*;

use crate::game_plugin::constants::*;
use crate::game_plugin::events::{ContinueEvent, InteractionEvent, TalkEvent};
use crate::game_plugin::resources::{LoadingData, PlayerLookInput, PlayerMovementInput, PlayerSounds};
use crate::game_plugin::states::GameState;
use crate::states::AppState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerLookInput::default())
            .insert_resource(PlayerMovementInput::default())
            .add_systems(OnEnter(AppState::Game), (setup_player, setup_sounds))
            .add_systems(
                PreUpdate,
                handle_input.after(InputSystems).run_if(
                    in_state(GameState::Playing)
                        .or_else(in_state(GameState::InDialog))
                        .or_else(in_state(GameState::Paused)),
                ),
            )
            .add_systems(Update, player_look.run_if(in_state(GameState::Playing)))
            .add_systems(FixedUpdate, player_movement.run_if(in_state(GameState::Playing)))
            .add_systems(Update, toggle_cursor.run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), cleanup_player);
    }
}

#[derive(Component)]
struct StepTimer(Timer);

impl Default for StepTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.4, TimerMode::Once))
    }
}

fn setup_player(mut commands: Commands) {
    commands
        .spawn((
            DespawnOnExit(AppState::Game),
            Transform::from_xyz(0.0, 1.0, 0.0),
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
            StepTimer::default(),
        ))
        .with_children(|parent| {
            // FPS Camera
            parent.spawn((
                DespawnOnExit(AppState::Game),
                Camera3d::default(),
                ContactShadows::default(),
                Transform::from_xyz(0.0, 0.70, -0.1).looking_at(Vec3::ZERO, Vec3::Y),
            ));
        });
}

fn setup_sounds(mut commands: Commands, asset_server: Res<AssetServer>, mut loading_data: ResMut<LoadingData>) {
    let step_sound: Handle<AudioSource> = asset_server.load(ASSET_PATH_SOUND_STEP);

    loading_data.assets.push(step_sound.clone().into());

    let knock_sound: Handle<AudioSource> = asset_server.load(ASSET_PATH_SOUND_KNOCK);

    loading_data.assets.push(knock_sound.clone().into());

    commands.insert_resource(PlayerSounds {
        knock_bundle: (AudioPlayer::new(knock_sound), PlaybackSettings::DESPAWN),
        step_bundle: (
            AudioPlayer::new(step_sound),
            PlaybackSettings {
                volume: Volume::Linear(0.25),
                ..PlaybackSettings::DESPAWN
            },
        ),
    });
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
            commands.trigger(ContinueEvent);
        }

        return;
    }

    if keyboard.just_pressed(KeyCode::KeyE) {
        commands.trigger(InteractionEvent);
    } else if keyboard.just_pressed(KeyCode::KeyT) {
        commands.trigger(TalkEvent);
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
    input: Res<PlayerLookInput>,
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
    mut commands: Commands,
    time: Res<Time>,
    mut input: ResMut<PlayerMovementInput>,
    mut player: Query<(
        &mut Transform,
        &mut KinematicCharacterController,
        Option<&KinematicCharacterControllerOutput>,
        &mut StepTimer,
    )>,
    mut game_state: ResMut<NextState<GameState>>,
    mut vertical_movement: Local<f32>,
    mut grounded_timer: Local<f32>,
    player_sounds: Res<PlayerSounds>,
) {
    let Ok((transform, mut controller, output, mut timer)) = player.single_mut() else {
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

    if movement.x.abs() > 0.0 || movement.z.abs() > 0.0 {
        timer.0.tick(time.delta());

        if timer.0.is_finished() {
            commands.spawn(player_sounds.step_bundle.clone());

            timer.0.reset();
        }
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

fn cleanup_player(
    mut keyboard: ResMut<ButtonInput<KeyCode>>,
    mut mouse_events: MessageReader<MouseMotion>,
    mut look: ResMut<PlayerLookInput>,
    mut movement: ResMut<PlayerMovementInput>,
) {
    keyboard.clear();
    mouse_events.clear();
    *look = PlayerLookInput::default();
    *movement = PlayerMovementInput::default();
}
