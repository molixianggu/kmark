use bevy::prelude::*;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Loading,

    Title,
    Game,
}

impl Default for GameState {
    fn default() -> Self {
        if cfg!(server) {
            GameState::Game
        } else {
            GameState::Loading
        }
    }
}
