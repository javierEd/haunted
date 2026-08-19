use bevy::prelude::*;

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, PartialEq, Eq, Clone, Copy)]
pub enum DisplayQuality {
    Low,
    Medium,
    High,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Volume(pub u32);
