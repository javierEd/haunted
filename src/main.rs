use bevy::prelude::*;

mod components;
mod constants;
mod game;
mod plugins;
mod resources;
mod states;
mod systems;

use crate::plugins::{GamePlugin, MenuPlugin, SplashPlugin};
use crate::resources::{DisplayQuality, Volume};
use crate::states::AppState;
use crate::systems::button_system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::NONE))
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        .init_state::<AppState>()
        .add_plugins(SplashPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(GamePlugin)
        .add_systems(Update, button_system)
        .run();
}
