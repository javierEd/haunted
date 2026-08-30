use std::f32::consts::FRAC_PI_2;
use std::sync::LazyLock;

use bevy::prelude::*;

use crate::game_plugin::components::{Door, Light};

use super::components::{ChapterCharacter, ChapterObject};

pub const ASSET_PATH_CHARACTER_NEIGHBOR_301: &str = "characters/neighbor-301.glb";
pub const ASSET_PATH_CHARACTER_NEIGHBOR_303: &str = "characters/neighbor-303.glb";
pub const ASSET_PATH_MAP_CHAPTER_1: &str = "maps/chapter-1.glb";

pub const LIGHT_307_HALL: &str = "light-307-hall";
pub const LIGHT_307_LIVING: &str = "light-307-living";

pub static DOORS: LazyLock<Vec<(Door, ChapterObject, ChapterCharacter, Transform)>> = LazyLock::new(|| {
    vec![
        // Stairs
        (
            Door::MAP_LIMIT,
            ChapterObject::DoorStairs,
            ChapterCharacter::None,
            Transform::from_xyz(-1.575, 1.1, 3.5).with_rotation(Quat::from_rotation_y(FRAC_PI_2)),
        ),
        // 301
        (
            Door::KNOCKABLE,
            ChapterObject::Door301,
            ChapterCharacter::Neighbor301,
            Transform::from_xyz(-1.575, 1.1, 14.5).with_rotation(Quat::from_rotation_y(FRAC_PI_2)),
        ),
        // 302
        (
            Door::KNOCKABLE,
            ChapterObject::Door302,
            ChapterCharacter::Neighbor302,
            Transform::from_xyz(1.575, 1.1, 14.5).with_rotation(Quat::from_rotation_y(-FRAC_PI_2)),
        ),
        // 303
        (
            Door::KNOCKABLE,
            ChapterObject::Door303,
            ChapterCharacter::Neighbor303,
            Transform::from_xyz(-1.575, 1.1, 9.5).with_rotation(Quat::from_rotation_y(FRAC_PI_2)),
        ),
        // 304
        (
            Door::KNOCKABLE,
            ChapterObject::Door304,
            ChapterCharacter::None,
            Transform::from_xyz(1.575, 1.1, 9.5).with_rotation(Quat::from_rotation_y(-FRAC_PI_2)),
        ),
        // 305
        (
            Door::KNOCKABLE,
            ChapterObject::Door305,
            ChapterCharacter::Neighbor305,
            Transform::from_xyz(-1.575, 1.1, -7.5).with_rotation(Quat::from_rotation_y(FRAC_PI_2)),
        ),
        // 306
        (
            Door::KNOCKABLE,
            ChapterObject::Door306,
            ChapterCharacter::Neighbor306,
            Transform::from_xyz(1.575, 1.1, -7.5).with_rotation(Quat::from_rotation_y(-FRAC_PI_2)),
        ),
        // 307
        (
            Door::LOCKED,
            ChapterObject::Door307,
            ChapterCharacter::None,
            Transform::from_xyz(-1.575, 1.1, -12.5).with_rotation(Quat::from_rotation_y(FRAC_PI_2)),
        ),
        // 308
        (
            Door::KNOCKABLE,
            ChapterObject::Door308,
            ChapterCharacter::Neighbor308,
            Transform::from_xyz(1.575, 1.1, -12.5).with_rotation(Quat::from_rotation_y(-FRAC_PI_2)),
        ),
    ]
});

pub static LIGHTS: LazyLock<Vec<(Light, Transform, Option<Transform>)>> = LazyLock::new(|| {
    vec![
        (
            Light {
                is_on: true,
                ..default()
            },
            Transform::from_xyz(0.0, 2.9, -10.0).with_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
            None,
        ),
        (
            Light {
                is_on: true,
                ..default()
            },
            Transform::from_xyz(0.0, 2.9, -4.0).with_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
            None,
        ),
        (
            Light {
                is_on: true,
                ..default()
            },
            Transform::from_xyz(0.0, 2.9, 1.0).with_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
            None,
        ),
        (
            Light {
                is_on: true,
                ..default()
            },
            Transform::from_xyz(0.0, 2.9, 6.0).with_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
            None,
        ),
        (
            Light {
                is_on: true,
                ..default()
            },
            Transform::from_xyz(0.0, 2.9, 12.0).with_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
            None,
        ),
        // Martinez living
        (
            Light {
                id: LIGHT_307_LIVING.to_owned(),
                is_on: false,
            },
            Transform::from_xyz(-3.2, 2.9, -11.5).with_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
            Some(Transform::from_xyz(-1.95, 1.1, -13.995)),
        ),
        // Martinez hall
        (
            Light {
                id: LIGHT_307_HALL.to_owned(),
                is_on: false,
            },
            Transform::from_xyz(-7.95, 2.9, -9.6).with_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
            Some(Transform::from_xyz(-5.0, 1.1, -9.005)),
        ),
    ]
});
