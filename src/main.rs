use crate::game::Game;
use crate::render::Render;
use bevy::prelude::*;

mod game;
mod render;
mod utils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Render {})
        .add_plugins(Game {})
        .run();
}
