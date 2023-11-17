use crate::constants::*;
use crate::game_config::GameConfig;
use crate::models::*;
use crate::objects::ball::setup_ball;
use crate::ui::game_state::setup_game_state_ui;
use crate::ui::scoreboard::setup_scoreboard;
use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_config: Res<GameConfig>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Paddle
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
    ));

    setup_ball(&mut commands, &mut materials, &mut meshes, &game_config);
    setup_scoreboard(&mut commands);
    setup_brick(&mut commands, &game_config);
    setup_wall(&mut commands, &mut materials, &game_config);
    setup_game_state_ui(&mut commands, &game_config);

    // Walls
}

fn setup_wall(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    game_config: &Res<GameConfig>,
) {
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));
}

fn setup_brick(commands: &mut Commands, game_config: &Res<GameConfig>) {
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

            // brick
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: game_config.brick_config.color,
                        ..default()
                    },
                    transform: Transform {
                        translation: brick_position.extend(0.0),
                        scale: Vec3::new(
                            game_config.brick_config.brick_size.x,
                            game_config.brick_config.brick_size.y,
                            1.0,
                        ),
                        ..default()
                    },
                    ..default()
                },
                Brick,
                Collider,
            ));
        }
    }
}
