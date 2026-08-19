use bevy::prelude::*;

use crate::game_plugin::resources::DialogBoxMessage;

#[derive(EntityEvent)]
pub struct DoorKnockEvent {
    pub entity: Entity,
}

#[derive(Event)]
pub struct InteractionEvent;

#[derive(Event)]
pub struct ContinueEvent;

#[derive(Event)]
pub struct TalkEvent;

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
