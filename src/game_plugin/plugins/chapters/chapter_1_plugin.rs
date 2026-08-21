use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};
use std::sync::LazyLock;

use bevy::color::palettes::css::SKY_BLUE;
use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::game_plugin::components::{Door, Talk};
use crate::game_plugin::constants::*;
use crate::game_plugin::events::{
    DialogBoxClosedEvent, DialogBoxEvent, DoorCloseEvent, DoorKnockEvent, DoorOpenEvent, TalkEvent,
};
use crate::game_plugin::resources::{DialogBoxMessage, DoorAnimations, GameCharacter, LoadingData};
use crate::game_plugin::states::{ChapterState, GameState};

use self::components::ChapterCharacter;
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

static DOORS: LazyLock<Vec<(Door, ChapterCharacter, Transform)>> = LazyLock::new(|| {
    vec![
        // Stairs
        (
            Door::MAP_LIMIT,
            ChapterCharacter::Unknown,
            Transform::from_xyz(-1.575, 1.1, 3.5).with_rotation(Quat::from_rotation_y(FRAC_PI_2)),
        ),
        // 301
        (
            Door::KNOCKABLE,
            ChapterCharacter::Neighbor301,
            Transform::from_xyz(-1.575, 1.1, 14.5).with_rotation(Quat::from_rotation_y(FRAC_PI_2)),
        ),
        // 302
        (
            Door::KNOCKABLE,
            ChapterCharacter::Neighbor302,
            Transform::from_xyz(1.575, 1.1, 14.5).with_rotation(Quat::from_rotation_y(-FRAC_PI_2)),
        ),
        // 303
        (
            Door::KNOCKABLE,
            ChapterCharacter::Neighbor303,
            Transform::from_xyz(-1.575, 1.1, 9.5).with_rotation(Quat::from_rotation_y(FRAC_PI_2)),
        ),
        // 304
        (
            Door::KNOCKABLE,
            ChapterCharacter::Unknown,
            Transform::from_xyz(1.575, 1.1, 9.5).with_rotation(Quat::from_rotation_y(-FRAC_PI_2)),
        ),
        // 305
        (
            Door::KNOCKABLE,
            ChapterCharacter::Neighbor305,
            Transform::from_xyz(-1.575, 1.1, -7.5).with_rotation(Quat::from_rotation_y(FRAC_PI_2)),
        ),
        // 306
        (
            Door::KNOCKABLE,
            ChapterCharacter::Neighbor306,
            Transform::from_xyz(1.575, 1.1, -7.5).with_rotation(Quat::from_rotation_y(-FRAC_PI_2)),
        ),
        // 307
        (
            Door::KNOCKABLE,
            ChapterCharacter::Unknown,
            Transform::from_xyz(-1.575, 1.1, -12.5).with_rotation(Quat::from_rotation_y(FRAC_PI_2)),
        ),
        // 308
        (
            Door::KNOCKABLE,
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
                intensity: 15000.0,
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

fn on_close_dialog_knock_303(
    event: On<DialogBoxClosedEvent>,
    mut commands: Commands,
    entity_query: Query<(Entity, &ChapterCharacter), With<Door>>,
    mut talk_query: Query<(&ChapterCharacter, &mut Visibility), With<Talk>>,
) {
    commands.entity(event.observer()).despawn();

    let Some((door_entity, _)) = entity_query
        .iter()
        .find(|(_, character)| **character == ChapterCharacter::Neighbor303)
    else {
        return;
    };

    commands.trigger(DoorOpenEvent { entity: door_entity });

    let Some((_, mut visiblity)) = talk_query
        .iter_mut()
        .find(|(character, _)| **character == ChapterCharacter::Neighbor303)
    else {
        return;
    };

    *visiblity = Visibility::default();
}

fn on_close_dialog_talk_303(
    event: On<DialogBoxClosedEvent>,
    mut commands: Commands,
    entity_query: Query<(Entity, &ChapterCharacter), With<Door>>,
    mut talk_query: Query<(&ChapterCharacter, &mut Visibility), With<Talk>>,
) {
    commands.entity(event.observer()).despawn();

    let Some((_, mut visiblity)) = talk_query
        .iter_mut()
        .find(|(character, _)| **character == ChapterCharacter::Neighbor303)
    else {
        return;
    };

    *visiblity = Visibility::Hidden;

    let Some((door_entity, _)) = entity_query
        .iter()
        .find(|(_, character)| **character == ChapterCharacter::Neighbor303)
    else {
        return;
    };

    commands.trigger(DoorCloseEvent { entity: door_entity });
}

fn on_door_knock_event(
    event: On<DoorKnockEvent>,
    mut commands: Commands,
    query_character: Query<&ChapterCharacter>,
    mut progress: ResMut<ChapterProgress>,
    mut talk_query: Query<(&ChapterCharacter, &mut Visibility), With<Talk>>,
) {
    let Ok(character) = query_character.get(event.entity) else {
        return;
    };

    match character {
        ChapterCharacter::Neighbor301 => {
            commands.trigger(DoorOpenEvent { entity: event.entity });

            let Some((_, mut visiblity)) = talk_query
                .iter_mut()
                .find(|(character, _)| **character == ChapterCharacter::Neighbor301)
            else {
                return;
            };

            *visiblity = Visibility::default();
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

            commands.add_observer(on_close_dialog_knock_303);

            progress.knocked_on_303 = true;
        }
        ChapterCharacter::Neighbor305 => {
            commands.trigger(DialogBoxEvent::with_message(DialogBoxMessage::new(
                character,
                "I don't want to talk to you.",
            )));
        }
        ChapterCharacter::Neighbor306 => {
            commands.trigger(DialogBoxEvent::with_messages(vec![
                DialogBoxMessage::new(character, "Who is it?"),
                DialogBoxMessage::player("Good evening lady! I'm a journalist for..."),
                DialogBoxMessage::new(character, "I don't have time for you, go away!"),
            ]));
        }
        ChapterCharacter::Neighbor308 => {
            commands.trigger(DialogBoxEvent::with_message(DialogBoxMessage::new(
                character,
                "I'm busy, go away!",
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

fn on_talk_event(event: On<TalkEvent>, mut commands: Commands, talk_query: Query<&ChapterCharacter, With<Talk>>) {
    let Ok(character) = talk_query.get(event.entity) else {
        return;
    };

    match character {
        ChapterCharacter::Neighbor301 => {
            commands.trigger(DialogBoxEvent::with_messages(vec![
                DialogBoxMessage::new(ChapterCharacter::Neighbor301, "Hello! How can I help you?"),
                DialogBoxMessage::player("Good evening sir! I'm a journalist for the Macondo Gazette, and I'm writing about Martinez family disappearing..."),
                DialogBoxMessage::player("I would like to ask you a few questions."),
                DialogBoxMessage::new(ChapterCharacter::Neighbor301, "Ok."),
            ]));
        }
        ChapterCharacter::Neighbor303 => {
            commands.trigger(DialogBoxEvent::with_messages(vec![
                DialogBoxMessage::new(ChapterCharacter::Neighbor303, "Yes? Who are you? What do you want?"),
                DialogBoxMessage::player("Good evening sir! I'm a journalist for the Macondo Gazette, and I'm writing about Martinez family disappearing..."),
                DialogBoxMessage::player("I would like to ask you a few questions."),
                DialogBoxMessage::new(ChapterCharacter::Neighbor303, "I'm a little busy right now."),
                DialogBoxMessage::player("It will take just a few seconds."),
                DialogBoxMessage::new(ChapterCharacter::Neighbor303, "I can't, sorry"),
            ]));

            commands.add_observer(on_close_dialog_talk_303);
        }
        _ => {}
    }
}

fn cleanup_chapter(mut progress: ResMut<ChapterProgress>) {
    *progress = ChapterProgress::default();
}

mod components {
    use std::fmt::Display;

    use bevy::ecs::component::Component;

    #[derive(Clone, Component, Default, Eq, PartialEq)]
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
        pub knocked_on_303: bool,
    }
}
