use bevy::camera::visibility::VisibleEntities;
use bevy::prelude::*;
use bevy::world_serialization::WorldInstance;
use bevy_rapier3d::geometry::Collider;
use bevy_rapier3d::prelude::*;

use crate::components::{Door, HelpOverlay, HelpOverlayText, LoadingOverlay};
use crate::game::chapter_1::{setup_map_chapter_1, setup_objects_chapter_1};
use crate::resources::{LoadingData, LookInput, MovementInput};
use crate::states::{AppState, GameState};

use super::{PipelinesReady, PipelinesReadyPlugin, PlayerPlugin, Vec3Trait, VisibleEntitiesTrait};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_plugins((
                RapierPhysicsPlugin::<NoUserData>::default(),
                RapierDebugRenderPlugin {
                    enabled: cfg!(debug_assertions),
                    ..default()
                },
                PipelinesReadyPlugin,
                PlayerPlugin,
            ))
            .insert_resource(LoadingData::default())
            .add_systems(
                OnEnter(AppState::Game),
                (cleanup_game, setup_map_chapter_1, setup_objects_chapter_1),
            )
            // .add_systems(Update, spawn_objects_chapter_1.run_if(in_state(AppState::Game)))
            .add_systems(Update, update_colliders.run_if(in_state(AppState::Game)))
            .add_systems(
                Update,
                update_help_overlay
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                hide_loading_overlay
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(GameState::Loading)),
            )
            .add_systems(OnExit(AppState::Game), cleanup_game);
    }
}

#[allow(clippy::type_complexity)]
fn update_colliders(
    mut commands: Commands,
    collider_query: Query<(Entity, &AsyncCollider), With<WorldInstance>>,
    children_query: Query<&Children>,
    mesh_query: Query<&Mesh3d, Without<Collider>>,
    meshes: Res<Assets<Mesh>>,
) {
    for (entity, async_collider) in collider_query.iter() {
        let mut collider_applied = false;

        for child_entity in children_query.iter_descendants(entity) {
            if let Ok(mesh_handle) = mesh_query.get(child_entity)
                && let Some(mesh) = meshes.get(mesh_handle)
            {
                match Collider::from_bevy_mesh(mesh, &async_collider.0) {
                    Some(collider) => {
                        commands.entity(child_entity).insert(collider);
                        collider_applied = true;
                    }
                    None => log::error!("Unable to generate collider from mesh {mesh:?}"),
                }
            }
        }

        if collider_applied {
            commands.entity(entity).remove::<AsyncCollider>();
        }
    }
}

fn update_help_overlay(
    player_query: Query<&GlobalTransform, With<KinematicCharacterController>>,
    mut help_overlay_query: Query<&mut Visibility, With<HelpOverlay>>,
    mut help_overlay_text_query: Query<&mut Text, With<HelpOverlayText>>,
    door_query: Query<(Entity, &Door, &GlobalTransform)>,
    camera_query: Query<&VisibleEntities, With<Camera>>,
    children_query: Query<&Children>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let Ok(mut help_visibility) = help_overlay_query.single_mut() else {
        return;
    };

    let Ok(mut help_text) = help_overlay_text_query.single_mut() else {
        return;
    };

    let Ok(camera_visible_entities) = camera_query.single() else {
        return;
    };

    if let Some((_, door, _)) = door_query
        .iter()
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
        **help_text = if door.is_open {
            "Press E to close door".to_owned()
        } else {
            "Press E to open door".to_owned()
        };

        *help_visibility = Visibility::default();
    } else {
        *help_visibility = Visibility::Hidden;
    }
}

// Monitors current loading status of assets.
fn hide_loading_overlay(
    mut game_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    mut loading_data: ResMut<LoadingData>,
    pipelines_ready: Res<PipelinesReady>,
    mut query: Query<&mut Visibility, With<LoadingOverlay>>,
) {
    if loading_data.loaded_assets_count == loading_data.assets.len() && pipelines_ready.0 {
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

fn cleanup_game(
    mut game_state: ResMut<NextState<GameState>>,
    mut movement: ResMut<MovementInput>,
    mut look: ResMut<LookInput>,
    mut loading_data: ResMut<LoadingData>,
) {
    game_state.set(GameState::default());
    *movement = MovementInput::default();
    *look = LookInput::default();
    *loading_data = LoadingData::default();
}
