#![allow(unused_variables)]

//! NetHack compiled to WebAssembly
//! 
//! This crate compiles NetHack to WASM that runs in the browser.

use wasm_bindgen::prelude::*;
use nethack_core::GameState;

#[wasm_bindgen]
pub struct Game {
    state: GameState,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        Game {
            state: GameState::new(),
        }
    }

    pub fn get_depth(&self) -> u32 {
        self.state.depth
    }

    pub fn get_turns(&self) -> u64 {
        self.state.turns
    }

    pub fn move_player(&mut self, dx: i32, dy: i32) {
        self.state.move_player(dx, dy);
    }

    pub fn get_player_pos(&self) -> u32 {
        (self.state.player_x << 16) | self.state.player_y
    }
}
