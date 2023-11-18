use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
mod constants;
mod game_config;
mod game_state;
mod logic;
mod models;
mod objects;
mod setup;
mod ui;

mod prelude {
    pub use crate::constants::*;
    pub use crate::game_config::*;
    pub use crate::game_state::*;
    pub use crate::models::*;
}


use constants::*;
use game_config::*;
use game_state::*;
use logic::general::control_game_start;
use logic::physics::check_for_collisions;
use logic::controllable::move_controllable;
use models::*;
use objects::ball::*;
use objects::paddle::*;
use setup::setup;
use ui::game_state::update_game_state_ui;
use ui::scoreboard::update_scoreboard;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard { score: 0 })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(GameConfig::default())
        .insert_resource(GameState::default())
        .add_event::<CollisionEvent>()
        .add_systems(Startup, setup)
        // Add our gameplay simulation systems to the fixed timestep schedule
        // which runs at 64 Hz by default
        .add_systems(
            FixedUpdate,
            (apply_velocity, move_controllable, check_for_collisions)
                // `chain`ing systems together runs them in order
                .chain(),
        )
        .add_systems(
            Update,
            (
                control_game_start,
                constrain_paddle,
                update_scoreboard,
                update_game_state_ui,
                fade_away,
                bevy::window::close_on_esc,
            ),
        )
        .run();
}

fn apply_velocity(
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
