use crate::game_config::GameConfig;
use crate::models::*;
use crate::objects::ball::setup_ball;
use crate::objects::brick::setup_brick;
use crate::objects::paddle::setup_paddle;
use crate::objects::wall::setup_wall;
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

    setup_ball(&mut commands, &mut materials, &mut meshes, &game_config);
    setup_scoreboard(&mut commands);
    setup_brick(&mut commands, &game_config);
    setup_wall(&mut commands, &mut materials, &game_config);
    setup_game_state_ui(&mut commands, &game_config);
    setup_paddle(&mut commands, &game_config);
}
