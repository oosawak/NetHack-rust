use serde::{Deserialize, Serialize};
use crate::rng::Rng;

/// Core game state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub depth: u32,
    pub turns: u64,
    pub player_x: u32,
    pub player_y: u32,
}

impl GameState {
    /// Create a new game state
    pub fn new() -> Self {
        Self {
            depth: 1,
            turns: 0,
            player_x: 50,
            player_y: 50,
        }
    }

    /// Advance game state by one turn
    pub fn advance_turn(&mut self) {
        self.turns += 1;
    }

    /// Move player in a direction
    pub fn move_player(&mut self, dx: i32, dy: i32) {
        if dx < 0 && self.player_x as i32 + dx >= 0 {
            self.player_x = (self.player_x as i32 + dx) as u32;
        } else if dx > 0 {
            self.player_x = self.player_x.saturating_add(dx as u32);
        }
        if dy < 0 && self.player_y as i32 + dy >= 0 {
            self.player_y = (self.player_y as i32 + dy) as u32;
        } else if dy > 0 {
            self.player_y = self.player_y.saturating_add(dy as u32);
        }
        self.advance_turn();
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}
