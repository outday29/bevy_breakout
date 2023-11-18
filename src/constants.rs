// All constants that are not configurable by the user are defined here.

use bevy::prelude::*;

// These constants are defined in `Transform` units.
// Using the default 2D camera they correspond 1:1 with screen pixels.

pub const WALL_THICKNESS: f32 = 10.0;
// x coordinates
pub const LEFT_WALL: f32 = -450.;
pub const RIGHT_WALL: f32 = 450.;
// y coordinates
pub const BOTTOM_WALL: f32 = -300.;
pub const TOP_WALL: f32 = 300.;

pub const BRICK_SIZE: Vec2 = Vec2::new(100., 30.);
// These values are exact
pub const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 270.0;
pub const GAP_BETWEEN_BRICKS: f32 = 5.0;
// These values are lower bounds, as the number of bricks is computed
pub const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;
pub const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.0;
pub const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;

pub const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
