use std::f32::consts::FRAC_PI_2;
use std::sync::LazyLock;

use bevy::color::palettes::css::SKY_BLUE;
use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::game_plugin::components::Door;
use crate::game_plugin::constants::{ASSET_PATH_MAP_CHAPTER_1, ASSET_PATH_OBJECT_DOOR};
use crate::game_plugin::events::{DialogBoxEvent, DoorKnockEvent};
use crate::game_plugin::resources::{DialogBoxMessage, DoorAnimations, GameCharacter, LoadingData};
use crate::game_plugin::states::{ChapterState, GameState};

use self::components::ChapterCharacter;
use self::resources::ChapterProgress;

pub struct Chapter1Plugin;

impl Plugin for Chapter1Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChapterProgress::default())
            .add_systems(OnEnter(ChapterState::One), (setup_map, setup_objects))
            .add_systems(
                Update,
                show_initial_monologue
                    .run_if(in_state(GameState::Playing))
                    .run_if(in_state(ChapterState::One))
                    .run_if(resource_equals(ChapterProgress::default())),
            )
            .add_systems(OnExit(ChapterState::One), cleanup_chapter)
            .add_observer(on_door_knock);
    }
}

static DOORS: LazyLock<Vec<(Door, ChapterCharacter, Transform)>> = LazyLock::new(|| {
    vec![
        // Stairs
        (
            Door::MapLimit,
            ChapterCharacter::Unknown,
            Transform::from_xyz(-1.575, 1.1, 3.5).with_rotation(Quat::from_rotation_y(FRAC_PI_2)),
        ),
        // 301
        (
            Door::Knockable,
            ChapterCharacter::Neighbor301,
            Transform::from_xyz(-1.575, 1.1, 14.5).with_rotation(Quat::from_rotation_y(FRAC_PI_2)),
        ),
        // 302
        (
            Door::Knockable,
            ChapterCharacter::Neighbor302,
            Transform::from_xyz(1.575, 1.1, 14.5).with_rotation(Quat::from_rotation_y(-FRAC_PI_2)),
        ),
        // 303
        (
            Door::Knockable,
            ChapterCharacter::Neighbor303,
            Transform::from_xyz(-1.575, 1.1, 9.5).with_rotation(Quat::from_rotation_y(FRAC_PI_2)),
        ),
        // 304
        (
            Door::Knockable,
            ChapterCharacter::Unknown,
            Transform::from_xyz(1.575, 1.1, 9.5).with_rotation(Quat::from_rotation_y(-FRAC_PI_2)),
        ),
        // 305
        (
            Door::Knockable,
            ChapterCharacter::Neighbor305,
            Transform::from_xyz(-1.575, 1.1, -7.5).with_rotation(Quat::from_rotation_y(FRAC_PI_2)),
        ),
        // 306
        (
            Door::Knockable,
            ChapterCharacter::Neighbor306,
            Transform::from_xyz(1.575, 1.1, -7.5).with_rotation(Quat::from_rotation_y(-FRAC_PI_2)),
        ),
        // 307
        (
            Door::Knockable,
            ChapterCharacter::Unknown,
            Transform::from_xyz(-1.575, 1.1, -12.5).with_rotation(Quat::from_rotation_y(FRAC_PI_2)),
        ),
        // 308
        (
            Door::Knockable,
            ChapterCharacter::Neighbor308,
            Transform::from_xyz(1.575, 1.1, -12.5).with_rotation(Quat::from_rotation_y(-FRAC_PI_2)),
        ),
    ]
});

static LIGHTS: LazyLock<Vec<Transform>> = LazyLock::new(|| {
    vec![
        Transform::from_xyz(0.0, 2.9, -10.0).with_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
        Transform::from_xyz(0.0, 2.9, -4.0).with_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
        Transform::from_xyz(0.0, 2.9, 1.0).with_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
        Transform::from_xyz(0.0, 2.9, 6.0).with_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
        Transform::from_xyz(0.0, 2.9, 12.0).with_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
    ]
});

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

    // Sky
    commands.spawn((
        DespawnOnExit(ChapterState::One),
        Mesh3d(meshes.add(Sphere::new(100.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: SKY_BLUE.with_luminance(0.333).into(),
            unlit: true,
            cull_mode: None,
            ..default()
        })),
        Transform::default(),
        NotShadowCaster,
    ));
}

fn setup_objects(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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
    for (door, character, transform) in DOORS.iter() {
        commands.spawn((
            DespawnOnExit(ChapterState::One),
            door.clone(),
            character.clone(),
            *transform,
            WorldAssetRoot(door_asset.clone()),
            AsyncCollider::default(),
        ));
    }

    // Hall lights
    for transform in LIGHTS.iter() {
        commands.spawn((
            DespawnOnExit(ChapterState::One),
            PointLight {
                intensity: 10000.0,
                radius: 0.1,
                shadow_maps_enabled: true,
                ..default()
            },
            *transform,
        ));
    }
}

fn show_initial_monologue(mut commands: Commands, mut progress: ResMut<ChapterProgress>) {
    commands.trigger(DialogBoxEvent::with_messages(vec![
        DialogBoxMessage::player(
            "The Martinez family has been missing for a month now, but finally I found a way into their building...",
        ),
        DialogBoxMessage::player("Someone told me one of their neighbors has a spare key to access their apartment..."),
    ]));

    progress.initial_monologue = true;
}

fn on_door_knock(event: On<DoorKnockEvent>, mut commands: Commands, query_character: Query<&ChapterCharacter>) {
    let Ok(character) = query_character.get(event.entity) else {
        return;
    };

    match character {
        ChapterCharacter::Neighbor301 => {
            commands.trigger(DialogBoxEvent::with_message(DialogBoxMessage::new(
                character,
                "I'm busy, go away!",
            )));
        }
        ChapterCharacter::Neighbor302 => {
            commands.trigger(DialogBoxEvent::with_message(DialogBoxMessage::new(
                character,
                "I don't know you, please leave me alone.",
            )));
        }
        ChapterCharacter::Neighbor303 => {
            commands.trigger(DialogBoxEvent::with_message(DialogBoxMessage::new(
                character,
                "Wait, Just give me a second, I'm coming.",
            )));
        }
        ChapterCharacter::Neighbor305 => {
            commands.trigger(DialogBoxEvent::with_message(DialogBoxMessage::new(
                character,
                "I don't want to talk to you.",
            )));
        }
        ChapterCharacter::Neighbor306 => {
            commands.trigger(DialogBoxEvent::with_message(DialogBoxMessage::new(
                character,
                "Who is it?",
            )));
        }
        ChapterCharacter::Neighbor308 => {
            commands.trigger(DialogBoxEvent::with_message(DialogBoxMessage::new(
                character,
                "Who is it?",
            )));
        }
        ChapterCharacter::Unknown => {
            commands.trigger(DialogBoxEvent::with_message(DialogBoxMessage::new(
                GameCharacter::Player,
                "Looks like nobody is here",
            )));
        }
    }
}

fn cleanup_chapter(mut progress: ResMut<ChapterProgress>) {
    *progress = ChapterProgress::default();
}

mod components {
    use std::fmt::Display;

    use bevy::ecs::component::Component;

    #[derive(Clone, Component, Default)]
    pub enum ChapterCharacter {
        #[default]
        Unknown,
        Neighbor301,
        Neighbor302,
        Neighbor303,
        Neighbor305,
        Neighbor306,
        Neighbor308,
    }

    impl Display for ChapterCharacter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Unknown => write!(f, "Unknown"),
                Self::Neighbor301 => write!(f, "Neighbor on 301"),
                Self::Neighbor302 => write!(f, "Neighbor on 302"),
                Self::Neighbor303 => write!(f, "Neighbor on 303"),
                Self::Neighbor305 => write!(f, "Neighbor on 305"),
                Self::Neighbor306 => write!(f, "Neighbor on 306"),
                Self::Neighbor308 => write!(f, "Neighbor on 308"),
            }
        }
    }
}

mod resources {
    use bevy::ecs::resource::Resource;

    #[derive(Default, Eq, PartialEq, Resource)]
    pub struct ChapterProgress {
        pub initial_monologue: bool,
    }
}
