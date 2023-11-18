use crate::objects::ball::Ball;
use crate::objects::brick::Brick;
use crate::objects::wall::{WallLocation, WallPosition};
use crate::prelude::*;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Collider;

#[derive(Event, Default)]
pub struct CollisionEvent;

pub fn check_for_collisions(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    mut ball_query: Query<(&mut Velocity, &Transform, &mut Ball), With<Ball>>,
    mut collider_query: Query<
        (
            Entity,
            &Transform,
            Option<&mut Brick>,
            Option<&WallPosition>,
        ),
        With<Collider>,
    >,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    if ball_query.is_empty() {
        return;
    }
    let (mut ball_velocity, ball_transform, mut ball) = ball_query.single_mut();
    let ball_size = ball_transform.scale.truncate();

    // check collision with walls
    for (collider_entity, transform, maybe_brick, maybe_wall_position) in collider_query.iter_mut()
    {
        let collision = collide(
            ball_transform.translation,
            ball_size,
            transform.translation,
            transform.scale.truncate(),
        );
        if let Some(collision) = collision {
            // Sends a collision event so that other systems can react to the collision
            collision_events.send_default();

            if let Some(mut brick) = maybe_brick {
                brick.take_damage(1);
                if brick.is_dead() {
                    scoreboard.score += brick.points;
                    commands.entity(collider_entity).despawn();
                }
            }

            // If maybe_wall_position is Some, check if it is the bottom wall, if so, the game is over
            match maybe_wall_position {
                Some(wall_position) => {
                    if wall_position.0 == WallLocation::Bottom {
                        ball.is_dead = true;
                    }
                }
                None => {}
            }
            // reflect the ball when it collides
            let mut reflect_x = false;
            let mut reflect_y = false;

            // only reflect if the ball's velocity is going in the opposite direction of the
            // collision
            match collision {
                Collision::Left => reflect_x = ball_velocity.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.x < 0.0,
                Collision::Top => reflect_y = ball_velocity.y < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                Collision::Inside => { /* do nothing */ }
            }

            // reflect velocity on the x-axis if we hit something on the x-axis
            if reflect_x {
                ball_velocity.x = -ball_velocity.x;
            }

            // reflect velocity on the y-axis if we hit something on the y-axis
            if reflect_y {
                ball_velocity.y = -ball_velocity.y;
            }
        }
    }
}

pub fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>,
    game_state: Res<GameState>,
) {
    if game_state.status == GameStatus::Running {
        for (mut transform, velocity) in &mut query {
            transform.translation.x += velocity.x * time.delta_seconds();
            transform.translation.y += velocity.y * time.delta_seconds();
        }
    }
}
