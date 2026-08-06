use bevy::prelude::*;

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, PartialEq, Eq, Clone, Copy)]
pub enum DisplayQuality {
    Low,
    Medium,
    High,
}

// A resource that holds the current loading data.
#[derive(Resource, Debug, Default)]
pub struct LoadingData {
    // This will hold the currently unloaded/loading assets.
    pub assets: Vec<UntypedHandle>,
    pub loaded_assets_count: usize,
}

/// Mouse input vector
#[derive(Default, Resource, Deref, DerefMut)]
pub struct LookInput(Vec2);

/// Keyboard input vector
#[derive(Default, Resource, Deref, DerefMut)]
pub struct MovementInput(Vec3);

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Volume(pub u32);
