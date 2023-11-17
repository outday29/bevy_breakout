use bevy::prelude::*;

#[derive(PartialEq)]
pub enum GameStatus {
    Stopped,
    Running,
    GameOver,
}

#[derive(Resource)]
pub struct GameState {
    pub status: GameStatus,
    pub score: u32,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            status: GameStatus::Stopped,
            score: 0,
        }
    }
}
