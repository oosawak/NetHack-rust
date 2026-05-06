#![allow(unused_variables)]

//! NetHack compiled to WebAssembly
//! 
//! This crate compiles NetHack to WASM that runs in the browser.
//! Note: C library integration is disabled for WASM (no C compilation support in wasm32-unknown-unknown)
//! Game logic is handled entirely in Rust via nethack-core.

use wasm_bindgen::prelude::*;
use nethack_core::world::World3D;
use nethack_core::camera::{Camera3D, ViewMode};
use nethack_core::game_renderer::GameRenderer;
use glam::Vec3;

// Initialize logging for WASM
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    #[cfg(target_arch = "wasm32")]
    {
        // Setup panic hook for WASM
        std::panic::set_hook(Box::new(|panic_info| {
            let msg = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
                *s
            } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
                s.as_str()
            } else {
                "Unknown panic"
            };
            web_sys::console::error_1(&format!("Panic: {}", msg).into());
        }));
    }
    
    Ok(())
}

/// Game state wrapper for JavaScript access
#[wasm_bindgen]
pub struct Game {
    world: World3D,
    camera: Camera3D,
    renderer: GameRenderer,
    running: bool,
}

#[wasm_bindgen]
impl Game {
    /// Create a new game instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        let world = World3D::new(80, 24);
        let camera = world.get_camera(ViewMode::TopDown);
        
        Game {
            world,
            camera,
            renderer: GameRenderer::new(),
            running: true,
        }
    }

    /// Initialize the game (player already placed by dungeon generation)
    pub fn init(&mut self) {
        #[cfg(target_arch = "wasm32")]
        {
            let p = self.world.player();
            web_sys::console::log_1(
                &format!("Game initialized: player at ({}, {})", p.position.x, p.position.z).into()
            );
        }
    }

    /// Get player X coordinate
    pub fn player_x(&self) -> i32 {
        let player = self.world.player();
        player.position.x as i32
    }

    /// Get player Y coordinate
    pub fn player_y(&self) -> i32 {
        let player = self.world.player();
        player.position.z as i32
    }

    /// Get dungeon width
    pub fn width(&self) -> i32 {
        self.world.level.width as i32
    }

    /// Get dungeon height
    pub fn height(&self) -> i32 {
        self.world.level.height as i32
    }

    /// Move player in a direction (dx=left/right, dy=up/down on screen)
    pub fn move_player(&mut self, dx: i32, dy: i32) {
        // dx → dungeon X axis, dy → dungeon Z axis (screen Y)
        self.world.move_player(dx as f32, 0.0, dy as f32);
    }

    /// Execute command character (e.g., 'k' for kick, 's' for search)
    pub fn execute_command(&mut self, command: char) {
        match command {
            'k' => {
                // Kick action - placeholder
            },
            's' => {
                // Search action - placeholder
            },
            _ => {}
        }
    }

    /// Update game state each frame
    pub fn update(&mut self) {
        // Update game logic if needed
    }

    /// Render game state to vertices
    /// Returns flattened vertex buffer as Vec<f32> (x, y, z, r, g, b, a for each vertex)
    pub fn render(&mut self) -> Vec<f32> {
        let (px, py) = {
            let player = self.world.player();
            (player.position.x as i32, player.position.z as i32)
        };
        self.renderer.update_from_game_state(
            px,
            py,
            &self.world.level,
        );

        // Convert vertices to flat f32 array for JavaScript
        let vertices = self.renderer.vertices();
        let mut result = Vec::new();
        for vertex in vertices {
            result.push(vertex.position[0]);
            result.push(vertex.position[1]);
            result.push(vertex.position[2]);
            result.push(vertex.color[0]);
            result.push(vertex.color[1]);
            result.push(vertex.color[2]);
            result.push(vertex.color[3]);
        }
        result
    }

    /// Get render indices (triangle indices)
    pub fn render_indices(&self) -> Vec<u16> {
        self.renderer.indices().to_vec()
    }

    /// Check if game is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Quit the game
    pub fn quit(&mut self) {
        self.running = false;
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&"Game quit".into());
    }

    /// Get vertex count for rendering
    pub fn vertex_count(&self) -> usize {
        self.renderer.vertex_count() as usize
    }

    /// Get index count for rendering
    pub fn index_count(&self) -> usize {
        self.renderer.index_count() as usize
    }
}

#[wasm_bindgen]
pub fn create_game() -> Game {
    Game::new()
}

/// Get version info
#[wasm_bindgen]
pub fn get_version() -> String {
    "NetHack WASM 0.1.0 (Rust-only edition)".to_string()
}
