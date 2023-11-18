use crate::logic::physics::Collider;
use crate::prelude::*;
use bevy::prelude::*;

pub enum BrickType {
    Normal,
    Hard,
    Unbreakable,
}

#[derive(Component)]
pub struct Brick {
    pub health: u32,
    pub max_health: u32,
    pub points: u32,
    pub color: Color,
}

impl Brick {
    fn get_bundle(brick_type: BrickType, transform: Transform) -> (SpriteBundle, Brick, Collider) {
        let sprite_bundle = match brick_type {
            BrickType::Normal => SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.5, 0.5, 0.5),
                    ..default()
                },
                transform,
                ..default()
            },
            BrickType::Hard => SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.8, 0.3, 0.3),
                    ..default()
                },
                transform,
                ..default()
            },
            BrickType::Unbreakable => SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.5, 0.5, 0.5),
                    ..default()
                },
                transform,
                ..default()
            },
        };
        let brick = match brick_type {
            BrickType::Normal => Brick {
                health: 1,
                max_health: 1,
                points: 1,
                color: Color::rgb(0.5, 0.5, 0.5),
            },
            BrickType::Hard => Brick {
                health: 2,
                max_health: 2,
                points: 2,
                color: Color::rgb(0.8, 0.3, 0.3),
            },
            BrickType::Unbreakable => Brick {
                health: 9999,
                max_health: 9999,
                points: 0,
                color: Color::rgb(0.1, 0.1, 0.1),
            },
        };
        (sprite_bundle, brick, Collider)
    }

    pub fn is_dead(&self) -> bool {
        self.health == 0
    }

    pub fn take_damage(&mut self, hitpoint: u32) {
        self.health = 0.max(self.health - hitpoint);
    }

    pub fn repair(&mut self, hitpoint: u32) {
        self.health += hitpoint;
        if self.health > self.max_health {
            self.health = self.max_health;
        }
    }
}

pub fn setup_brick(commands: &mut Commands, game_config: &Res<GameConfig>) {
    let paddle_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;
    // Bricks
    let total_width_of_bricks = (RIGHT_WALL - LEFT_WALL) - 2. * GAP_BETWEEN_BRICKS_AND_SIDES;
    let bottom_edge_of_bricks = paddle_y + GAP_BETWEEN_PADDLE_AND_BRICKS;
    let total_height_of_bricks = TOP_WALL - bottom_edge_of_bricks - GAP_BETWEEN_BRICKS_AND_CEILING;

    assert!(total_width_of_bricks > 0.0);
    assert!(total_height_of_bricks > 0.0);

    // Given the space available, compute how many rows and columns of bricks we can fit
    let n_columns = (total_width_of_bricks
        / (game_config.brick_config.brick_size.x + GAP_BETWEEN_BRICKS))
        .floor() as usize;
    let n_rows = (total_height_of_bricks
        / (game_config.brick_config.brick_size.y + GAP_BETWEEN_BRICKS))
        .floor() as usize;
    let n_vertical_gaps = n_columns - 1;

    // Because we need to round the number of columns,
    // the space on the top and sides of the bricks only captures a lower bound, not an exact value
    let center_of_bricks = (LEFT_WALL + RIGHT_WALL) / 2.0;
    let left_edge_of_bricks = center_of_bricks
    // Space taken up by the bricks
    - (n_columns as f32 / 2.0 * game_config.brick_config.brick_size.x)
    // Space taken up by the gaps
    - n_vertical_gaps as f32 / 2.0 * GAP_BETWEEN_BRICKS;

    // In Bevy, the `translation` of an entity describes the center point,
    // not its bottom-left corner
    let offset_x = left_edge_of_bricks + game_config.brick_config.brick_size.x / 2.;
    let offset_y = bottom_edge_of_bricks + game_config.brick_config.brick_size.y / 2.;

    for row in 0..n_rows {
        for column in 0..n_columns {
            let brick_position = Vec2::new(
                offset_x
                    + column as f32 * (game_config.brick_config.brick_size.x + GAP_BETWEEN_BRICKS),
                offset_y
                    + row as f32 * (game_config.brick_config.brick_size.y + GAP_BETWEEN_BRICKS),
            );

            // Brick
            let transform = Transform {
                translation: brick_position.extend(0.0),
                scale: Vec3::new(
                    game_config.brick_config.brick_size.x,
                    game_config.brick_config.brick_size.y,
                    1.0,
                ),
                ..default()
            };
            commands.spawn(Brick::get_bundle(BrickType::Hard, transform));
        }
    }
}
