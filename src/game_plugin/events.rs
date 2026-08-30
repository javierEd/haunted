use bevy::prelude::*;

use crate::game_plugin::resources::DialogBoxMessage;

#[derive(Event)]
pub struct ContinueKeyEvent;

#[derive(Event)]
pub struct DialogBoxClosedEvent;

#[derive(Default, Event)]
pub struct DialogBoxEvent {
    pub messages: Vec<DialogBoxMessage>,
}

impl DialogBoxEvent {
    pub fn with_message(message: DialogBoxMessage) -> Self {
        let mut event = Self::default();

        event.messages.push(message);

        event
    }

    pub fn with_messages(mut messages: Vec<DialogBoxMessage>) -> Self {
        let mut event = Self::default();

        event.messages.append(&mut messages);

        event
    }
}

#[derive(EntityEvent)]
pub struct DoorCloseEvent {
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct DoorKnockEvent {
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct DoorOpenEvent {
    pub entity: Entity,
}

#[derive(Event)]
pub struct InteractionKeyEvent;

#[derive(EntityEvent)]
pub struct ToggleLightEvent {
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct LockPickingEvent {
    pub entity: Entity,
}

#[derive(Event)]
pub struct LockPickingMoveLeftEvent;

#[derive(Event)]
pub struct LockPickingMoveRightEvent;

#[derive(Event)]
pub struct LockPickingRotateEvent;

#[derive(EntityEvent)]
pub struct TalkEvent {
    pub entity: Entity,
}
