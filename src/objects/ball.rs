use crate::logic::physics::Velocity;
use crate::prelude::*;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const BALL_COLOR: Color = Color::rgb(1.0, 0.1, 0.1);

#[derive(Component)]
pub struct Ball {
    pub is_dead: bool,
}

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
        Ball { is_dead: false },
        Velocity(
            game_config.ball_config.initial_direction.normalize() * game_config.ball_config.speed,
        ),
    ));
}

pub fn fade_away(
    mut commands: Commands,
    _materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(Entity, &mut Ball), With<Ball>>,
) {
    for (entity, ball) in query.iter() {
        if ball.is_dead {
            commands.entity(entity).despawn();
        }
    }
}
