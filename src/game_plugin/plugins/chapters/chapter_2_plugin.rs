use std::f32::consts::{FRAC_PI_2, PI};

use bevy::color::palettes::css::SKY_BLUE;
use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::game_plugin::components::Door;
use crate::game_plugin::constants::{ASSET_PATH_MAP_CHAPTER_2, ASSET_PATH_OBJECT_DOOR};
use crate::game_plugin::resources::{DoorAnimations, LoadingData};
use crate::game_plugin::states::ChapterState;
use crate::states::AppState;

pub struct Chapter2Plugin;

impl Plugin for Chapter2Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ChapterState::Two), (setup_map, setup_objects));
    }
}

pub fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut loading_data: ResMut<LoadingData>,
) {
    let map_asset = asset_server.load(GltfAssetLabel::Scene(0).from_asset(ASSET_PATH_MAP_CHAPTER_2));

    loading_data.assets.push(map_asset.clone().into());

    // Ground
    commands.spawn((
        DespawnOnExit(AppState::Game),
        Transform::default(),
        WorldAssetRoot(map_asset),
        AsyncCollider::default(),
    ));

    // Sun
    commands.spawn((
        DespawnOnExit(AppState::Game),
        DirectionalLight {
            color: Color::srgb(0.98, 0.95, 0.82),
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(100.0, 100.0, 0.0).looking_at(Vec3::new(-0.15, -0.05, 0.25), Vec3::Y),
    ));

    // Sky
    commands.spawn((
        DespawnOnExit(AppState::Game),
        Mesh3d(meshes.add(Sphere::new(100.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: SKY_BLUE.into(),
            unlit: true,
            cull_mode: None,
            ..default()
        })),
        Transform::default(),
        NotShadowCaster,
    ));
}

pub fn setup_objects(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading_data: ResMut<LoadingData>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
) {
    let door_close_animation = asset_server.load(GltfAssetLabel::Animation(0).from_asset(ASSET_PATH_OBJECT_DOOR));
    let door_open_animation = asset_server.load(GltfAssetLabel::Animation(1).from_asset(ASSET_PATH_OBJECT_DOOR));
    let door_asset = asset_server.load(GltfAssetLabel::Scene(0).from_asset(ASSET_PATH_OBJECT_DOOR));

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

    commands.spawn((
        DespawnOnExit(AppState::Game),
        Door::Locked,
        Transform::from_xyz(-0.06, 1.3, -17.356),
        WorldAssetRoot(door_asset.clone()),
        AsyncCollider::default(),
    ));

    commands.spawn((
        DespawnOnExit(AppState::Game),
        Door::Locked,
        Transform::from_xyz(-1.018, 1.3, -20.386).with_rotation(Quat::from_rotation_y(FRAC_PI_2)),
        WorldAssetRoot(door_asset.clone()),
        AsyncCollider::default(),
    ));

    commands.spawn((
        DespawnOnExit(AppState::Game),
        Door::Locked,
        Transform::from_xyz(-1.018, 1.3, -24.133).with_rotation(Quat::from_rotation_y(FRAC_PI_2)),
        WorldAssetRoot(door_asset.clone()),
        AsyncCollider::default(),
    ));

    commands.spawn((
        DespawnOnExit(AppState::Game),
        Door::Locked,
        Transform::from_xyz(0.099, 1.3, -25.182),
        WorldAssetRoot(door_asset.clone()),
        AsyncCollider::default(),
    ));

    commands.spawn((
        DespawnOnExit(AppState::Game),
        Door::Locked,
        Transform::from_xyz(3.145, 1.3, -27.206).with_rotation(Quat::from_rotation_y(PI)),
        WorldAssetRoot(door_asset),
        AsyncCollider::default(),
    ));
}
