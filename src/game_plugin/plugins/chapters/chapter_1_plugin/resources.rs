use bevy::ecs::resource::Resource;

#[derive(Default, Eq, PartialEq, Resource)]
pub struct ChapterProgress {
    pub initial_monologue: bool,
    pub knocked_on_303: bool,
}
