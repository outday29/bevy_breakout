use crate::game_config::GameConfig;
use crate::models::*;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const BALL_COLOR: Color = Color::rgb(1.0, 0.1, 0.1);

#[derive(Component)]
pub struct Ball;

pub fn setup_ball(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    game_config: &Res<GameConfig>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(BALL_COLOR)),
            transform: Transform::from_translation(BALL_STARTING_POSITION).with_scale(BALL_SIZE),
            ..default()
        },
        Ball,
        Velocity(
            game_config.ball_config.initial_direction.normalize() * game_config.ball_config.speed,
        ),
    ));
}

pub fn fade_away(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<Entity, With<Ball>>,
) {
    let mut ball = query.single_mut();
    commands.entity(ball).despawn();
}
