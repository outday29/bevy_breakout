use crate::constants::*;
use crate::game_config::GameConfig;
use crate::logic::controllable::Controllable;
use crate::logic::physics::Collider;
use crate::models::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Paddle;

const PADDLE_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;
const INITIAL_PADDLE_SPEED: f32 = 30.0;
const PADDLE_PADDING: f32 = 10.0; // How close can the paddle get to the wall
const BOTTOM_WALL: f32 = -300.;

pub fn setup_paddle(commands: &mut Commands, game_config: &Res<GameConfig>) {
    let paddle_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, paddle_y, 0.0),
                scale: game_config.paddle_config.size,
                ..default()
            },
            sprite: Sprite {
                color: game_config.paddle_config.color,
                ..default()
            },
            ..default()
        },
        Paddle,
        Collider,
        Controllable {
            speed: INITIAL_PADDLE_SPEED,
        },
    ));
}

pub fn constrain_paddle(
    mut paddle_query: Query<&mut Transform, With<Paddle>>,
    game_config: Res<GameConfig>,
) {
    let mut paddle = paddle_query.single_mut();
    let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.x / 2.0 + PADDLE_PADDING;
    let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 - PADDLE_PADDING;

    paddle.translation.x = paddle.translation.x.clamp(left_bound, right_bound);
}
