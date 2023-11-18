use crate::game_state::{GameState, GameStatus};
use bevy::prelude::*;

#[derive(Component)]
pub struct Controllable {
    pub speed: f32,
}

pub fn move_controllable(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Controllable, &mut Transform)>,
    game_state: Res<GameState>,
) {
    if game_state.status == GameStatus::Running {
        for (controllable, mut transform) in query.iter_mut() {
            let mut direction = 0.0;
            if keyboard_input.pressed(KeyCode::Left) {
                direction -= 1.0;
            }
            if keyboard_input.pressed(KeyCode::Right) {
                direction += 1.0;
            }
            transform.translation.x += direction * controllable.speed;
        }
    }
}