use bevy::render::render_resource::PipelineCache;
use bevy::render::{MainWorld, RenderApp};
use bevy::{color::palettes::css::BLACK, prelude::*};

use crate::game_plugin::resources::LoadingData;
use crate::game_plugin::states::{ChapterState, GameState};
use crate::states::AppState;

use self::components::LoadingOverlay;

pub struct LoadingOverlayPlugin;

impl Plugin for LoadingOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoadingData::default())
            .add_systems(OnEnter(AppState::Game), setup_loading_overlay)
            .add_systems(
                Update,
                hide_loading_overlay
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(GameState::Loading))
                    .run_if(not(in_state(ChapterState::None))),
            )
            .add_systems(OnExit(AppState::Game), cleanup_loading_overlay)
            .sub_app_mut(RenderApp)
            .add_systems(ExtractSchedule, update_pipelines_ready);
    }
}

fn setup_loading_overlay(mut commands: Commands) {
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
}

// Monitors current loading status of assets.
fn hide_loading_overlay(
    mut game_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    mut loading_data: ResMut<LoadingData>,
    mut query: Query<&mut Visibility, With<LoadingOverlay>>,
) {
    if loading_data.loaded_assets_count == loading_data.assets.len() && loading_data.pipelines_ready {
        game_state.set(GameState::Playing);

        for mut visibility in &mut query {
            *visibility = Visibility::Hidden;
        }
    } else if loading_data.loaded_assets_count < loading_data.assets.len() {
        loading_data.loaded_assets_count = loading_data
            .assets
            .iter()
            .filter(|asset| {
                asset_server
                    .get_recursive_dependency_load_state(*asset)
                    .map(|state| state.is_loaded())
                    .unwrap_or_default()
            })
            .count();
    }
}

fn update_pipelines_ready(mut main_world: ResMut<MainWorld>, pipelines: Res<PipelineCache>) {
    let Some(true) = main_world
        .get_resource::<State<AppState>>()
        .map(|app_state| app_state.get().is_game())
    else {
        return;
    };

    let Some(true) = main_world
        .get_resource::<State<GameState>>()
        .map(|game_state| game_state.get().is_loading())
    else {
        return;
    };

    let Some(true) = main_world
        .get_resource::<State<ChapterState>>()
        .map(|chapter_state| chapter_state.get().is_none())
    else {
        return;
    };

    if let Some(mut pipelines_ready) = main_world.get_resource_mut::<LoadingData>() {
        pipelines_ready.pipelines_ready = pipelines.waiting_pipelines().count() == 0;
    }
}

fn cleanup_loading_overlay(mut loading_data: ResMut<LoadingData>) {
    *loading_data = LoadingData::default();
}

mod components {
    use bevy::ecs::component::Component;

    #[derive(Component)]
    pub struct LoadingOverlay;
}
