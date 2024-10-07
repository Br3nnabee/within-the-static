use crate::render::{despawn_screen, OnGameScreen};
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

pub struct Game {}

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.insert_resource(DisplayQuality::Medium);
        app.insert_resource(Volume(7));
        app.add_systems(OnEnter(GameState::Game), init_game);
        app.add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
        app.init_state::<GameState>();
    }
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum DisplayQuality {
    Low,
    Medium,
    High,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Volume(pub u32);

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Menu,
    Loading,
    Pause,
    Game,
}

fn init_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_handle: Handle<TiledMap> = asset_server.load("map.tmx");
    commands.spawn(TiledMapBundle {
        tiled_map: map_handle,
        ..Default::default()
    });
}
