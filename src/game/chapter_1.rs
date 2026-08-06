use bevy::color::palettes::css::SKY_BLUE;
use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::constants::ASSET_PATH_MAP_CHAPTER_1;
use crate::resources::LoadingData;
use crate::states::AppState;

pub fn setup_map_chapter_1(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut loading_data: ResMut<LoadingData>,
) {
    let map_asset = asset_server.load(GltfAssetLabel::Scene(0).from_asset(ASSET_PATH_MAP_CHAPTER_1));

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
