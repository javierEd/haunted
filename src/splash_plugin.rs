use bevy::prelude::*;

use crate::{constants::ASSET_PATH_IMAGE_BEVY, states::AppState};

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
        app
            // When entering the state, spawn everything needed for this screen
            .add_systems(OnEnter(AppState::Splash), splash_setup)
            // While in this state, run the `countdown` system
            .add_systems(Update, countdown.run_if(in_state(AppState::Splash)));
    }
}

// Newtype to use a `Timer` for this screen as a resource
#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load(ASSET_PATH_IMAGE_BEVY);
    // Display the logo
    commands.spawn((
        // This entity will be despawned when exiting the state
        DespawnOnExit(AppState::Splash),
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: percent(100),
            height: percent(100),
            ..default()
        },
        children![(
            ImageNode::new(icon),
            Node {
                // This will set the logo to be 200px wide, and auto adjust its height
                width: px(200),
                ..default()
            },
        )],
        Camera2d,
    ));

    // Insert the timer as a resource
    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}

// Tick the timer, and change state when finished
fn countdown(mut app_state: ResMut<NextState<AppState>>, time: Res<Time>, mut timer: ResMut<SplashTimer>) {
    if timer.tick(time.delta()).is_finished() {
        app_state.set(AppState::Menu);
    }
}
