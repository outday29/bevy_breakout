use crate::constants::*;
use crate::models::*;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    config: Res<Configuration>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(config.paddle_size).into())
                .into(),
            material: materials.add(ColorMaterial::from(config.paddle_color)),
            transform: Transform::from_translation(PADDLE_STARTING_POSITION.extend(0.)),
            ..default()
        },
        Player,
        Collider,
        Paddle,
    ));
    let wall_list = [
        WallLocation::Bottom,
        WallLocation::Left,
        WallLocation::Right,
        WallLocation::Top,
    ];
    for i in wall_list.into_iter() {
        commands.spawn(WallBundle::new(i));
    }
}

pub fn setup_bricks(mut commands: Commands, config: Res<Configuration>) {
    commands.spawn((BrickBundle::new(Vec2::new(0.0, 0.0), config.brick_size, config.brick_color)));
    commands.spawn((BrickBundle::new(Vec2::new(150.0, -250.0), config.brick_size, config.brick_color)));
}

pub fn setup_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    config: Res<Configuration>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Circle::new(config.ball_radius).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        Ball,
        Velocity(INITIAL_BALL_VELOCITY * config.ball_speed),
    ));
}

pub fn setup_scoreboard(mut commands: Commands, config: Res<Configuration>) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: config.scoreboard_font_size,
                    color: config.scoreboard_color,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: config.scoreboard_font_size,
                color: config.scoreboard_color,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Percent(2.0),
            right: Val::Percent(2.0),
            ..default()
        }),
        Scoreboard,
    ));
}

// pub fn setup_debug_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands.spawn((
//         TextBundle::from_sections([
//             TextSection::new(
//                 "       Position: ",
//                 TextStyle {
//                     font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//                     font_size: 20.0,
//                     color: Color::WHITE,
//                 },
//             ),
//             TextSection::from_style(TextStyle {
//                 font_size: 30.0,
//                 color: Color::GOLD,
//                 font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//                 ..default()
//             }),
//         ]),
//         FPSText,
//     ));
// }

pub fn setup_position_debug(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Position: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 30.0,
                color: Color::GOLD,
                // If no font is specified, it will use the default font.
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                ..default()
            }),
        ]),
        DebugPositionText,
    ));
}
