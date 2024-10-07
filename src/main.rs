use crate::render::Render;
use bevy::prelude::*;

mod render;
mod utils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Render {})
        .run();
}
