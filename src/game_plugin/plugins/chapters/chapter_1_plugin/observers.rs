use bevy::prelude::*;

use crate::game_plugin::components::{Door, DoorInteraction, Talk};
use crate::game_plugin::events::*;
use crate::game_plugin::resources::{DialogBoxMessage, GameCharacter};

use super::components::ChapterCharacter;
use super::resources::ChapterProgress;

pub fn on_close_dialog_knock_303(
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

pub fn on_close_dialog_talk_303(
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

pub fn on_door_knock_event(
    event: On<DoorKnockEvent>,
    mut commands: Commands,
    mut door_query: Query<(&mut Door, &ChapterCharacter)>,
    mut progress: ResMut<ChapterProgress>,
    mut talk_query: Query<(&ChapterCharacter, &mut Visibility), With<Talk>>,
) {
    let Ok((mut door, character)) = door_query.get_mut(event.entity) else {
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
        ChapterCharacter::None => {
            commands.trigger(DialogBoxEvent::with_message(DialogBoxMessage::new(
                GameCharacter::Player,
                "Looks like nobody is here",
            )));

            door.interaction = DoorInteraction::Open;
        }
    }
}

pub fn on_talk_event(event: On<TalkEvent>, mut commands: Commands, talk_query: Query<&ChapterCharacter, With<Talk>>) {
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
                DialogBoxMessage::new(ChapterCharacter::Neighbor303, "Al right, but make it quick."),
                DialogBoxMessage::player("How well do you know Martinez family?"),
                DialogBoxMessage::new(ChapterCharacter::Neighbor303, "I like to think that I'm very close to them."),
                DialogBoxMessage::player("Do you have some clue where they could be?"),
                DialogBoxMessage::new(ChapterCharacter::Neighbor303, "They always tell me when they have to leave town for a few days, but this time they just left without any notice."),
                DialogBoxMessage::player("Interesting."),
                DialogBoxMessage::new(ChapterCharacter::Neighbor303, "I actually have a key they gave me to take care of their when they are out of town."),
                DialogBoxMessage::player("Really? Could you give me access to their apartment, so I can take some pictures?."),
            ]));

            commands.add_observer(on_close_dialog_talk_303);
        }
        _ => {}
    }
}
