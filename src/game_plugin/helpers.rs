use bevy::camera::visibility::VisibleEntities;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::game_plugin::components::{DelayTimer, Door};

pub trait QueryTrait {
    fn nearest(
        &self,
        _player_query: Query<&Transform, With<KinematicCharacterController>>,
        _camera_query: Query<&VisibleEntities, With<Camera>>,
        _children_query: Query<&Children>,
    ) -> Option<(Entity, &Door, &Transform)> {
        None
    }

    fn nearest_mut(
        &mut self,
        _player_query: Query<&Transform, With<KinematicCharacterController>>,
        _camera_query: Query<&VisibleEntities, With<Camera>>,
        _children_query: Query<&Children>,
    ) -> Option<(Entity, Mut<'_, Door>, &Transform)> {
        None
    }
}

impl QueryTrait for Query<'_, '_, (Entity, &Door, &Transform), Without<DelayTimer>> {
    fn nearest(
        &self,
        player_query: Query<&Transform, With<KinematicCharacterController>>,
        camera_query: Query<&VisibleEntities, With<Camera>>,
        children_query: Query<&Children>,
    ) -> Option<(Entity, &Door, &Transform)> {
        let Ok(player_transform) = player_query.single() else {
            return None;
        };

        let Ok(camera_visible_entities) = camera_query.single() else {
            return None;
        };

        self.iter()
            .filter(|(e, d, t)| {
                let door_translation = if d.is_open() {
                    t.translation + (t.forward() * 0.5)
                } else {
                    t.translation
                };

                door_translation.is_near(&player_transform.translation)
                    && children_query
                        .iter_descendants(*e)
                        .any(|ce| camera_visible_entities.is_visible(ce))
            })
            .min_by(move |(_, _, t1), (_, _, t2)| {
                t1.translation
                    .distance(player_transform.translation)
                    .partial_cmp(&t2.translation.distance(player_transform.translation))
                    .unwrap()
            })
    }
}

impl QueryTrait for Query<'_, '_, (Entity, &mut Door, &Transform), Without<DelayTimer>> {
    fn nearest_mut(
        &mut self,
        player_query: Query<&Transform, With<KinematicCharacterController>>,
        camera_query: Query<&VisibleEntities, With<Camera>>,
        children_query: Query<&Children>,
    ) -> Option<(Entity, Mut<'_, Door>, &Transform)> {
        let Ok(player_transform) = player_query.single() else {
            return None;
        };

        let Ok(camera_visible_entities) = camera_query.single() else {
            return None;
        };

        self.iter_mut()
            .filter(|(e, d, t)| {
                let door_translation = if d.is_open() {
                    t.translation + (t.forward() * 0.5)
                } else {
                    t.translation
                };

                door_translation.is_near(&player_transform.translation)
                    && children_query
                        .iter_descendants(*e)
                        .any(|ce| camera_visible_entities.is_visible(ce))
            })
            .min_by(move |(_, _, t1), (_, _, t2)| {
                t1.translation
                    .distance(player_transform.translation)
                    .partial_cmp(&t2.translation.distance(player_transform.translation))
                    .unwrap()
            })
    }
}

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
    fn is_visible(&self, origin_transform: &Transform, target_transform: &Transform, target_entity: Entity) -> bool;
}

impl ReadRapierContextTrait for ReadRapierContext<'_, '_> {
    fn is_visible(&self, origin_transform: &Transform, target_transform: &Transform, target_entity: Entity) -> bool {
        self.single()
            .ok()
            .and_then(|rc| {
                rc.cast_ray(
                    origin_transform.translation + (origin_transform.forward() * 0.26),
                    target_transform.translation - origin_transform.translation,
                    bevy_rapier3d::math::Real::MAX,
                    true,
                    QueryFilter::default(),
                )
            })
            .map(|(hit, _)| hit == target_entity)
            .unwrap_or_default()
    }
}
