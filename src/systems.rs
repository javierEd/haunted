use bevy::prelude::*;

use crate::components::SelectedOption;
use crate::constants::{BUTTON_HOVERED, BUTTON_HOVERED_PRESSED, BUTTON_NORMAL, BUTTON_PRESSED};

// This system handles changing all buttons color based on mouse interaction
#[allow(clippy::type_complexity)]
pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color, selected) in &mut interaction_query {
        *background_color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => BUTTON_PRESSED.into(),
            (Interaction::Hovered, Some(_)) => BUTTON_HOVERED_PRESSED.into(),
            (Interaction::Hovered, None) => BUTTON_HOVERED.into(),
            (Interaction::None, None) => BUTTON_NORMAL.into(),
        }
    }
}
