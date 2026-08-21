use bevy::prelude::*;
use bevy::world_serialization::WorldInstance;
use bevy_rapier3d::prelude::*;

use crate::states::AppState;

mod components;
mod constants;
mod events;
mod helpers;
mod plugins;
mod resources;
mod states;

use self::plugins::*;
use self::states::{ChapterState, GameState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GlobalAmbientLight {
            color: Color::BLACK,
            brightness: 0.0,
            ..default()
        })
        .insert_state(GameState::default())
        .insert_state(ChapterState::default())
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin {
                enabled: cfg!(debug_assertions),
                ..default()
            },
            LoadingOverlayPlugin,
            InputPlugin,
            PlayerPlugin,
            PlayerInteractionPlugin,
            DialogBoxPlugin,
            HelpBoxPlugin,
            PauseMenuPlugin,
            GameOverMenuPlugin,
            chapters::Chapter1Plugin,
            chapters::Chapter2Plugin,
        ))
        .add_systems(OnEnter(AppState::Game), setup_game)
        .add_systems(Update, update_colliders.run_if(in_state(AppState::Game)))
        .add_systems(OnExit(AppState::Game), cleanup_game);
    }
}

fn setup_game(mut chapter_state: ResMut<NextState<ChapterState>>) {
    chapter_state.set(ChapterState::One);
}

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

fn cleanup_game(mut game_state: ResMut<NextState<GameState>>, mut chapter_state: ResMut<NextState<ChapterState>>) {
    game_state.set(GameState::default());
    chapter_state.set(ChapterState::default());
}
