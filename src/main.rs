use bevy::prelude::*;

mod components;
mod constants;
mod game_plugin;
mod helpers;
mod menu_plugin;
mod resources;
mod splash_plugin;
mod states;

use crate::game_plugin::GamePlugin;
use crate::helpers::button_system;
use crate::menu_plugin::MenuPlugin;
use crate::resources::{DisplayQuality, Volume};
use crate::splash_plugin::SplashPlugin;
use crate::states::AppState;

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
