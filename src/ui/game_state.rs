use crate::game_config::GameConfig;
use crate::game_state::*;
use bevy::prelude::*;

const GAME_STATE_UI_POSITION: Vec2 = Vec2::new(5.0, 5.0);
const GAME_STATE_UI_FONT_SIZE: f32 = 20.0;
const GAME_STATE_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

#[derive(Component)]
pub struct GameStateUI;

pub fn setup_game_state_ui(
    commands: &mut Commands,
    game_config: &Res<GameConfig>,
) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Status: ",
                TextStyle {
                    font_size: GAME_STATE_UI_FONT_SIZE,
                    color: GAME_STATE_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: GAME_STATE_UI_FONT_SIZE,
                color: game_config.game_state_color.stopped,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(GAME_STATE_UI_POSITION.y),
            right: Val::Px(GAME_STATE_UI_POSITION.x.into()),
            ..default()
        }),
        GameStateUI,
    ));
}

pub fn update_game_state_ui(
    mut query: Query<&mut Text, With<GameStateUI>>,
    game_state: Res<GameState>,
    game_config: Res<GameConfig>,
) {
    let mut text = query.single_mut();
    match game_state.status {
        GameStatus::Stopped => {
            text.sections[1].value = "Stopped".to_string();
            text.sections[1].style.color = game_config.game_state_color.stopped;
        }
        GameStatus::Running => {
            text.sections[1].value = "Running".to_string();
            text.sections[1].style.color = game_config.game_state_color.running;
        }
        GameStatus::GameOver => {
            text.sections[1].value = "Game Over".to_string();
            text.sections[1].style.color = game_config.game_state_color.game_over;
        }
    }
}
