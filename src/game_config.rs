use bevy::prelude::*;

pub struct PaddleConfig {
    pub size: Vec3,
    pub color: Color,
}

impl Default for PaddleConfig {
    fn default() -> Self {
        PaddleConfig {
            size: Vec3::new(120.0, 20.0, 0.0),
            color: Color::rgb(0.3, 0.3, 0.7),
        }
    }
}

pub struct BrickConfig {
    pub color: Color,
    pub brick_size: Vec2,
}

impl Default for BrickConfig {
    fn default() -> Self {
        BrickConfig {
            color: Color::rgb(0.5, 0.5, 0.5), // Default color for bricks
            brick_size: Vec2::new(60.0, 20.0),
        }
    }
}

pub struct BallConfig {
    pub color: Color,
    pub speed: f32,
    pub initial_direction: Vec2,
}

impl Default for BallConfig {
    fn default() -> Self {
        BallConfig {
            color: Color::rgb(0.7, 0.7, 0.7), // Default color for ball
            speed: 250.0,                     // Default speed for ball
            initial_direction: Vec2::new(0.2, 0.5).normalize(),
        }
    }
}

pub struct GameStateColor {
    pub stopped: Color,
    pub running: Color,
    pub game_over: Color,
}

impl Default for GameStateColor {
    fn default() -> Self {
        GameStateColor {
            stopped: Color::rgb(0.8, 0.8, 0.8),
            running: Color::rgb(0.2, 0.8, 0.2),
            game_over: Color::rgb(0.8, 0.2, 0.2),
        }
    }
}


#[derive(Resource)]
pub struct GameConfig {
    pub paddle_config: PaddleConfig,
    pub brick_config: BrickConfig,
    pub ball_config: BallConfig,
    pub game_state_color: GameStateColor
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig {
            paddle_config: PaddleConfig::default(),
            brick_config: BrickConfig::default(),
            ball_config: BallConfig::default(),
            game_state_color: GameStateColor::default(),
        }
    }
}
