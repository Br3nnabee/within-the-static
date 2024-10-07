use crate::utils::log;
use bevy::prelude::*;

pub struct Game {}

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_game);
    }
}

fn init_game() {
    log("Info", "Game initialization started.");
}
