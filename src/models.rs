use crate::constants::*;
use bevy::prelude::*;

// This resource tracks the game's score
#[derive(Resource)]
pub struct Scoreboard {
    pub score: u32,
}
