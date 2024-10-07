#![allow(dead_code)]
use crate::game::Game;
use crate::render::Render;
use crate::ui::UI;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod game;
mod render;
mod ui;
mod utils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Render {})
        .add_plugins(Game {})
        .add_plugins(UI {})
        .add_plugins(TilemapPlugin)
        .add_plugins(TiledMapPlugin)
        .run();
}
