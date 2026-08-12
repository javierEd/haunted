use bevy::camera::visibility::VisibleEntities;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod game_plugin;
mod menu_plugin;
mod pipelines_ready_plugin;
mod player_plugin;
mod splash_plugin;

use pipelines_ready_plugin::{PipelinesReady, PipelinesReadyPlugin};
use player_plugin::PlayerPlugin;

pub use game_plugin::GamePlugin;
pub use menu_plugin::MenuPlugin;
pub use splash_plugin::SplashPlugin;

use crate::constants::{BUTTON_NORMAL, COLOR_TEXT};

trait Vec3Trait {
    fn is_near(&self, target_translation: &Vec3) -> bool;
}

impl Vec3Trait for Vec3 {
    fn is_near(&self, target_translation: &Vec3) -> bool {
        self.distance(*target_translation) < 1.2
    }
}

trait VisibleEntitiesTrait {
    fn is_visible(&self, target_entity: Entity) -> bool;
}

impl VisibleEntitiesTrait for VisibleEntities {
    fn is_visible(&self, target_entity: Entity) -> bool {
        self.entities
            .iter()
            .any(|(_, entities)| entities.contains(&target_entity))
    }
}

#[allow(dead_code)]
trait ReadRapierContextTrait {
    fn is_visible(
        &self,
        origin_transform: &GlobalTransform,
        target_transform: &GlobalTransform,
        target_entity: Entity,
    ) -> bool;
}

impl ReadRapierContextTrait for ReadRapierContext<'_, '_> {
    fn is_visible(
        &self,
        origin_transform: &GlobalTransform,
        target_transform: &GlobalTransform,
        target_entity: Entity,
    ) -> bool {
        self.single()
            .ok()
            .and_then(|rc| {
                rc.cast_ray(
                    origin_transform.translation() + (origin_transform.forward() * 0.26),
                    target_transform.translation() - origin_transform.translation(),
                    bevy_rapier3d::math::Real::MAX,
                    true,
                    QueryFilter::default(),
                )
            })
            .map(|(hit, _)| hit == target_entity)
            .unwrap_or_default()
    }
}

fn icon_button<T>(icon: Handle<Image>, label: &str, action: T) -> impl Bundle
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

fn text_button<T>(label: &str, action: T) -> impl Bundle
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
