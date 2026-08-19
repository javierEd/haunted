use bevy::camera::visibility::VisibleEntities;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::game_plugin::components::{DelayTimer, Door};
use crate::game_plugin::events::{DialogBoxEvent, DoorKnockEvent, InteractionEvent};
use crate::game_plugin::helpers::QueryTrait;
use crate::game_plugin::resources::{DialogBoxMessage, DoorAnimations, PlayerSounds};

pub struct PlayerInteractionPlugin;

impl Plugin for PlayerInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, thick_delayed_actions).add_observer(on_event);
    }
}

fn thick_delayed_actions(mut commands: Commands, time: Res<Time>, mut query: Query<(Entity, &mut DelayTimer)>) {
    for (entity, mut timer) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.is_finished() {
            commands.entity(entity).remove::<DelayTimer>();
            commands.trigger(DoorKnockEvent { entity });
        }
    }
}

fn on_event(
    _: On<InteractionEvent>,
    mut commands: Commands,
    player_query: Query<&Transform, With<KinematicCharacterController>>,
    camera_query: Query<&VisibleEntities, With<Camera>>,
    mut door_query: Query<(Entity, &mut Door, &Transform), Without<DelayTimer>>,
    children_query: Query<&Children>,
    mut animation_player_query: Query<(Entity, &mut AnimationPlayer)>,
    door_animations: Res<DoorAnimations>,
    player_sounds: Res<PlayerSounds>,
) {
    if let Some((entity, mut door, _)) = door_query.nearest_mut(player_query, camera_query, children_query) {
        match *door {
            Door::Open | Door::Closed => {
                commands.entity(entity).insert(DelayTimer::new(1.0));

                door.toggle_open();

                for child_entity in children_query.iter_descendants(entity) {
                    if let Ok((entity, mut animation_player)) = animation_player_query.get_mut(child_entity) {
                        if door.is_open() {
                            commands
                                .entity(entity)
                                .insert(AnimationGraphHandle(door_animations.open_graph_handle.clone()));
                            animation_player.start(door_animations.open_node_index);
                        } else {
                            commands
                                .entity(entity)
                                .insert(AnimationGraphHandle(door_animations.close_graph_handle.clone()));
                            animation_player.start(door_animations.close_node_index);
                        }
                    }
                }
            }
            Door::Knockable => {
                commands.entity(entity).insert(DelayTimer::new(2.0));
                commands.spawn(player_sounds.knock_bundle.clone());
            }
            Door::Locked => {
                commands.trigger(DialogBoxEvent::with_message(DialogBoxMessage::player(
                    "This door is locked.",
                )));
            }
            Door::MapLimit => {
                commands.trigger(DialogBoxEvent::with_message(DialogBoxMessage::player(
                    "I have things to do here first.",
                )));
            }
        }
    }
}
