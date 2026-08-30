use std::f32::consts::FRAC_PI_4;

use bevy::color::palettes::css::SKY_BLUE;
use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::game_plugin::components::{LightSwitch, Talk};
use crate::game_plugin::constants::ASSET_PATH_OBJECT_DOOR;
use crate::game_plugin::events::DialogBoxEvent;
use crate::game_plugin::resources::{DialogBoxMessage, DoorAnimations, LoadingData};
use crate::game_plugin::states::{ChapterState, GameState};

mod components;
mod constants;
mod observers;
mod resources;

use self::components::ChapterCharacter;
use self::constants::*;
use self::observers::{on_door_knock_event, on_talk_event};
use self::resources::ChapterProgress;

pub struct Chapter1Plugin;

impl Plugin for Chapter1Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChapterProgress::default())
            .add_systems(
                OnEnter(ChapterState::One),
                (setup_map, setup_characters, setup_talks, setup_objects),
            )
            .add_systems(
                Update,
                show_initial_monologue
                    .run_if(in_state(GameState::Playing))
                    .run_if(in_state(ChapterState::One))
                    .run_if(resource_equals(ChapterProgress::default())),
            )
            .add_systems(OnExit(ChapterState::One), cleanup_chapter)
            .add_observer(on_door_knock_event)
            .add_observer(on_talk_event);
    }
}

fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut loading_data: ResMut<LoadingData>,
) {
    let map_asset = asset_server.load(GltfAssetLabel::Scene(0).from_asset(ASSET_PATH_MAP_CHAPTER_1));

    loading_data.assets.push(map_asset.clone().into());

    // Map
    commands.spawn((
        DespawnOnExit(ChapterState::One),
        Transform::default(),
        WorldAssetRoot(map_asset),
        AsyncCollider::default(),
    ));

    // Sun
    commands.spawn((
        DespawnOnExit(ChapterState::One),
        DirectionalLight {
            color: Color::srgb(0.98, 0.95, 0.82),
            shadow_maps_enabled: true,
            illuminance: 50.0,
            ..default()
        },
        Transform::from_xyz(-10.0, 15.0, -100.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
    ));

    // Sky
    commands.spawn((
        DespawnOnExit(ChapterState::One),
        Mesh3d(meshes.add(Sphere::new(100.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: SKY_BLUE.with_luminance(0.1).into(),
            unlit: true,
            cull_mode: None,
            ..default()
        })),
        Transform::default(),
        NotShadowCaster,
    ));
}

fn setup_characters(mut commands: Commands, asset_server: Res<AssetServer>, mut loading_data: ResMut<LoadingData>) {
    let neighbor_301_asset = asset_server.load(GltfAssetLabel::Scene(0).from_asset(ASSET_PATH_CHARACTER_NEIGHBOR_301));

    loading_data.assets.push(neighbor_301_asset.clone().into());

    commands.spawn((
        DespawnOnExit(ChapterState::One),
        Transform::from_xyz(-1.975, 0.0, 14.0).with_rotation(Quat::from_rotation_y(FRAC_PI_4)),
        WorldAssetRoot(neighbor_301_asset.clone()),
    ));

    let neighbor_303_asset = asset_server.load(GltfAssetLabel::Scene(0).from_asset(ASSET_PATH_CHARACTER_NEIGHBOR_303));

    loading_data.assets.push(neighbor_303_asset.clone().into());

    commands.spawn((
        DespawnOnExit(ChapterState::One),
        Transform::from_xyz(-1.975, 0.0, 9.0).with_rotation(Quat::from_rotation_y(FRAC_PI_4)),
        WorldAssetRoot(neighbor_303_asset.clone()),
    ));
}

fn setup_talks(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn((
        DespawnOnExit(ChapterState::One),
        Talk,
        Visibility::Hidden,
        ChapterCharacter::Neighbor301,
        Mesh3d(meshes.add(Sphere::new(0.5))),
        Transform::from_xyz(-1.575, 1.0, 14.5),
    ));

    commands.spawn((
        DespawnOnExit(ChapterState::One),
        Talk,
        Visibility::Hidden,
        ChapterCharacter::Neighbor303,
        Mesh3d(meshes.add(Sphere::new(0.5))),
        Transform::from_xyz(-1.575, 1.0, 9.5),
    ));
}

fn setup_objects(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut loading_data: ResMut<LoadingData>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
) {
    let door_asset = asset_server.load(GltfAssetLabel::Scene(0).from_asset(ASSET_PATH_OBJECT_DOOR));
    let door_close_animation = asset_server.load(GltfAssetLabel::Animation(0).from_asset(ASSET_PATH_OBJECT_DOOR));
    let door_open_animation = asset_server.load(GltfAssetLabel::Animation(1).from_asset(ASSET_PATH_OBJECT_DOOR));

    loading_data.assets.push(door_asset.clone().into());
    loading_data.assets.push(door_close_animation.clone().into());
    loading_data.assets.push(door_open_animation.clone().into());

    let (close_graph, close_node_index) = AnimationGraph::from_clip(door_close_animation);
    let close_graph_handle = animation_graphs.add(close_graph);
    let (open_graph, open_node_index) = AnimationGraph::from_clip(door_open_animation);
    let open_graph_handle = animation_graphs.add(open_graph);

    commands.insert_resource(DoorAnimations::new(
        close_graph_handle.clone(),
        close_node_index,
        open_graph_handle.clone(),
        open_node_index,
    ));

    // Doors
    for (door, object, character, transform) in DOORS.iter() {
        commands.spawn((
            DespawnOnExit(ChapterState::One),
            door.clone(),
            object.clone(),
            character.clone(),
            *transform,
            WorldAssetRoot(door_asset.clone()),
            AsyncCollider::default(),
        ));
    }

    // Lights
    for (light, transform, switch_transform) in LIGHTS.iter() {
        commands.spawn((
            DespawnOnExit(ChapterState::One),
            light.clone(),
            PointLight {
                intensity: 15000.0,
                radius: 0.1,
                shadow_maps_enabled: true,
                ..default()
            },
            if light.is_on {
                Visibility::default()
            } else {
                Visibility::Hidden
            },
            *transform,
        ));

        // Light switch
        if let Some(transform) = switch_transform {
            commands.spawn((
                DespawnOnExit(ChapterState::One),
                LightSwitch {
                    target_id: light.id.clone(),
                },
                Mesh3d(meshes.add(Sphere::new(0.1))),
                *transform,
            ));
        }
    }
}

fn show_initial_monologue(mut commands: Commands, mut progress: ResMut<ChapterProgress>) {
    commands.trigger(DialogBoxEvent::with_messages(vec![
        DialogBoxMessage::player(
            "The Martinez family has been missing for a month now, but finally I found a way into their building...",
        ),
        DialogBoxMessage::player(
            "I've been told that one of their neighbors has a spare key to access their apartment...",
        ),
    ]));

    progress.initial_monologue = true;
}

fn cleanup_chapter(mut progress: ResMut<ChapterProgress>) {
    *progress = ChapterProgress::default();
}
