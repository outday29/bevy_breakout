use crate::objects::ball::Ball;
use crate::prelude::*;
use bevy::prelude::*;

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

pub fn check_game_over(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut ball_query: Query<&Ball>,
) {
    if game_state.status == GameStatus::Running {
        if let Ok(_) = ball_query.get_single() {
        } else {
            game_state.status = GameStatus::GameOver;
        }
    }
}
