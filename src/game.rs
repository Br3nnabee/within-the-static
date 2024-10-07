use crate::render::{despawn_screen, OnGameScreen};
use bevy::prelude::*;
pub struct Game {}

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.insert_resource(DisplayQuality::Medium);
        app.insert_resource(Volume(7));

        app.add_systems(Update, game.run_if(in_state(GameState::Game)));
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

#[derive(Resource, Deref, DerefMut)]
pub struct GameTimer(pub Timer);

fn game(
    time: Res<Time>,
    mut game_state: ResMut<NextState<GameState>>,
    mut timer: ResMut<GameTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu);
    }
}
