use bevy::prelude::*;

use crate::components::SelectedOption;
use crate::constants::{BUTTON_HOVERED, BUTTON_HOVERED_PRESSED, BUTTON_NORMAL, BUTTON_PRESSED, COLOR_TEXT};

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

pub fn icon_button<T>(icon: Handle<Image>, label: &str, action: T) -> impl Bundle
where
    T: Component,
{
    (
        Button,
        Node {
            width: px(300),
            height: px(65),
            margin: UiRect::all(px(20)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(BUTTON_NORMAL),
        action,
        children![
            (
                ImageNode::new(icon),
                Node {
                    width: px(30),
                    // This takes the icons out of the flexbox flow, to be positioned exactly
                    position_type: PositionType::Absolute,
                    // The icon will be close to the left border of the button
                    left: px(10),
                    ..default()
                }
            ),
            (
                Text::new(label),
                TextFont {
                    font_size: FontSize::Px(33.0),
                    ..default()
                },
                TextColor(COLOR_TEXT),
            ),
        ],
    )
}

pub fn text_button<T>(label: &str, action: T) -> impl Bundle
where
    T: Component,
{
    (
        Button,
        Node {
            width: px(300),
            height: px(65),
            margin: UiRect::all(px(20)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(BUTTON_NORMAL),
        action,
        children![(
            Text::new(label),
            (
                TextFont {
                    font_size: FontSize::Px(33.0),
                    ..default()
                },
                TextColor(COLOR_TEXT),
            )
        )],
    )
}
