use bevy::prelude::*;
use crate::prelude::*;

pub fn control_game_start(keyboard_input: Res<Input<KeyCode>>, mut game_state: ResMut<GameState>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match game_state.status {
            GameStatus::Stopped => {
                game_state.status = GameStatus::Running;
            }
            GameStatus::Running => {
                game_state.status = GameStatus::Stopped;
            }
            GameStatus::GameOver => {
                game_state.status = GameStatus::Stopped;
                game_state.score = 0;
            }
        }
    }
}
