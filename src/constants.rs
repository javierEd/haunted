use bevy::prelude::*;

pub const ASSET_PATH_IMAGE_EXIT_RIGHT: &str = "images/exit-right.png";
pub const ASSET_PATH_IMAGE_RIGHT: &str = "images/right.png";
pub const ASSET_PATH_IMAGE_WRENCH: &str = "images/wrench.png";
pub const ASSET_PATH_MAP_CHAPTER_1: &str = "maps/chapter-1.glb";
pub const ASSET_PATH_MODEL_CAM_RECORDER: &str = "models/cam-recorder.glb";
pub const ASSET_PATH_IMAGE_BEVY: &str = "images/bevy.png";

pub const BUTTON_NORMAL: Color = Color::srgb(0.15, 0.15, 0.15);
pub const BUTTON_HOVERED: Color = Color::srgb(0.25, 0.25, 0.25);
pub const BUTTON_HOVERED_PRESSED: Color = Color::srgb(0.25, 0.65, 0.25);
pub const BUTTON_PRESSED: Color = Color::srgb(0.35, 0.75, 0.35);

pub const COLOR_TEXT: Color = Color::srgb(0.9, 0.9, 0.9);

pub const MOUSE_SENSITIVITY: f32 = 0.2;
pub const GROUND_TIMER: f32 = 0.5;
pub const MOVEMENT_SPEED: f32 = 12.0;
pub const JUMP_SPEED: f32 = 10.0;
pub const GRAVITY: f32 = -9.81;
