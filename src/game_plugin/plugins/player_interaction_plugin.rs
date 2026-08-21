use bevy::camera::visibility::VisibleEntities;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::game_plugin::components::{AfterInteractionTimer, AfterKnockTimer, Door, DoorIteraction, DoorStatus, Talk};
use crate::game_plugin::events::*;
use crate::game_plugin::helpers::{DoorQueryTrait, QueryTrait};
use crate::game_plugin::resources::{DialogBoxMessage, DoorAnimations, PlayerSounds};

pub struct PlayerInteractionPlugin;

impl Plugin for PlayerInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, thick_after_interaction)
            .add_systems(Update, thick_after_knock)
            .add_observer(on_interaction_key_event)
            .add_observer(on_door_open_event)
            .add_observer(on_door_close_event);
    }
}

fn thick_after_interaction(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut AfterInteractionTimer)>,
) {
    for (entity, mut timer) in &mut query {
        timer.0.tick(time.delta());

        if timer.0.is_finished() {
            commands.entity(entity).remove::<AfterInteractionTimer>();
        }
    }
}

fn thick_after_knock(mut commands: Commands, time: Res<Time>, mut query: Query<(Entity, &mut AfterKnockTimer)>) {
    for (entity, mut timer) in &mut query {
        timer.0.tick(time.delta());

        if timer.0.is_finished() {
            commands.trigger(DoorKnockEvent { entity });
            commands.entity(entity).remove::<AfterKnockTimer>();
        }
    }
}

fn on_door_open_event(
    event: On<DoorOpenEvent>,
    mut commands: Commands,
    mut door_query: Query<&mut Door>,
    children_query: Query<&Children>,
    mut animation_player_query: Query<(Entity, &mut AnimationPlayer)>,
    door_animations: Res<DoorAnimations>,
) {
    let Ok(mut door) = door_query.get_mut(event.entity) else {
        return;
    };

    commands.entity(event.entity).insert(AfterInteractionTimer::new(2.25));

    door.set_is_open(true);

    for child_entity in children_query.iter_descendants(event.entity) {
        if let Ok((entity, mut animation_player)) = animation_player_query.get_mut(child_entity) {
            commands
                .entity(entity)
                .insert(AnimationGraphHandle(door_animations.open_graph_handle.clone()));
            animation_player.start(door_animations.open_node_index);
        }
    }
}

fn on_door_close_event(
    event: On<DoorCloseEvent>,
    mut commands: Commands,
    mut door_query: Query<&mut Door>,
    children_query: Query<&Children>,
    mut animation_player_query: Query<(Entity, &mut AnimationPlayer)>,
    door_animations: Res<DoorAnimations>,
) {
    let Ok(mut door) = door_query.get_mut(event.entity) else {
        return;
    };

    commands.entity(event.entity).insert(AfterInteractionTimer::new(2.25));

    door.set_is_open(false);

    for child_entity in children_query.iter_descendants(event.entity) {
        if let Ok((entity, mut animation_player)) = animation_player_query.get_mut(child_entity) {
            commands
                .entity(entity)
                .insert(AnimationGraphHandle(door_animations.close_graph_handle.clone()));
            animation_player.start(door_animations.close_node_index);
        }
    }
}

fn on_interaction_key_event(
    _: On<InteractionKeyEvent>,
    mut commands: Commands,
    player_query: Query<&Transform, With<KinematicCharacterController>>,
    camera_query: Query<&VisibleEntities, With<Camera>>,
    door_query: Query<(Entity, &Door, &Transform), (Without<AfterInteractionTimer>, Without<AfterKnockTimer>)>,
    talk_query: Query<(Entity, &Transform), With<Talk>>,
    children_query: Query<&Children>,
    player_sounds: Res<PlayerSounds>,
) {
    if let Some((entity, _)) = talk_query.nearest(player_query, camera_query, children_query) {
        commands.trigger(TalkEvent { entity });

        return;
    }

    let Some((entity, door, _)) = door_query.nearest(player_query, camera_query, children_query) else {
        return;
    };

    commands.entity(entity).insert(AfterInteractionTimer::new(2.25));

    match (door.interaction.clone(), door.status.clone()) {
        (DoorIteraction::Open, DoorStatus::Closed) => {
            commands.trigger(DoorOpenEvent { entity });
        }
        (DoorIteraction::Open, DoorStatus::Open) => {
            commands.trigger(DoorCloseEvent { entity });
        }
        (DoorIteraction::Open, DoorStatus::Locked) => {
            commands.trigger(DialogBoxEvent::with_message(DialogBoxMessage::player(
                "This door is locked.",
            )));
        }
        (DoorIteraction::Open, DoorStatus::MapLimit) => {
            commands.trigger(DialogBoxEvent::with_message(DialogBoxMessage::player(
                "I have things to do here first.",
            )));
        }
        (DoorIteraction::Knock, DoorStatus::Locked) => {
            commands.entity(entity).insert(AfterKnockTimer::new(2.0));
            commands.spawn(player_sounds.knock_bundle.clone());
        }
        _ => {}
    }
}
