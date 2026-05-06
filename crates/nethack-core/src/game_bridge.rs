//! Game Bridge: Integrates C FFI state with Rust World3D
//! 
//! This module bridges the C NetHack library with the Rust game state,
//! allowing safe access to C global variables and synchronization
//! between the two worlds.

use crate::world::{World3D, Entity};
use crate::camera::{Camera3D, ViewMode};
use glam::Vec3;

/// Safe wrapper around NetHack C library state
///
/// Provides controlled access to the C game engine while maintaining
/// Rust safety guarantees. The GameBridge coordinates between:
/// - C FFI calls (nethack-sys) for game logic
/// - Rust World3D for game state representation
/// - Camera system for rendering
pub struct GameBridge {
    /// Reference to the Rust game world
    world: World3D,
    
    /// Current camera state
    camera: Camera3D,
    
    /// Current dungeon level (cached from C)
    current_level: i32,
    
    /// Whether the C game state has been synchronized
    synced: bool,
}

impl GameBridge {
    /// Create a new Game Bridge
    pub fn new() -> Self {
        let world = World3D::new(80, 24);  // Standard NetHack dimensions
        let camera = Camera3D::new(40.0, 0.0, 12.0, ViewMode::TopDown);

        GameBridge {
            world,
            camera,
            current_level: 0,
            synced: false,
        }
    }

    /// Initialize the game through the C library
    ///
    /// This calls the FFI initialization stages to set up NetHack.
    pub fn init_game(&mut self) -> Result<(), String> {
        // Use the globals module to get C state
        let current_level = nethack_sys::globals::get_current_level();
        self.current_level = current_level;

        self.synced = false;
        Ok(())
    }

    /// Synchronize C game state to Rust World3D
    ///
    /// This reads the current player position and other game state
    /// from the C library and updates the Rust World3D accordingly.
    pub fn sync_from_c(&mut self) -> Result<(), String> {
        // NOTE: This is a placeholder for Phase 3.1
        // In Phase 3.1, we'll implement:
        // 1. Player position access from C global `u`
        // 2. Monster list reading from C global `fmon`
        // 3. Object list reading from C global `fobj`
        // 4. World3D entity updates

        // For now, we'll just mark as synced
        // (actual implementation in Phase 3.1)
        
        self.synced = true;
        Ok(())
    }

    /// Get the current dungeon level
    pub fn dungeon_level(&self) -> i32 {
        self.current_level
    }

    /// Get a mutable reference to the world
    pub fn world_mut(&mut self) -> &mut World3D {
        &mut self.world
    }

    /// Get a reference to the world
    pub fn world(&self) -> &World3D {
        &self.world
    }

    /// Get a mutable reference to the camera
    pub fn camera_mut(&mut self) -> &mut Camera3D {
        &mut self.camera
    }

    /// Get a reference to the camera
    pub fn camera(&self) -> &Camera3D {
        &self.camera
    }

    /// Check if the C state is synchronized with Rust
    pub fn is_synced(&self) -> bool {
        self.synced
    }

    /// Get player position (to be filled in Phase 3.1)
    ///
    /// This will read the player position from the C global `u`
    pub fn player_position(&self) -> Option<(i32, i32)> {
        // TODO: Implement in Phase 3.1
        // Will read from C: you.ux, you.uy
        None
    }

    /// Get current view mode
    pub fn view_mode(&self) -> ViewMode {
        self.camera.mode
    }

    /// Switch to a different view mode
    pub fn set_view_mode(&mut self, mode: ViewMode) {
        // Get the current player position
        let player = self.world.player();
        let player_pos = player.position;
        
        // Create a new camera with the new mode
        self.camera = Camera3D::new(
            player_pos.x,
            player_pos.y,
            player_pos.z,
            mode,
        );
    }

    /// Update camera to follow player
    pub fn update_camera_follow_player(&mut self) {
        if let Some((x, y)) = self.player_position() {
            let player = self.world.player();
            let player_pos = player.position;
            
            self.camera = Camera3D::new(
                player_pos.x,
                player_pos.y,
                player_pos.z,
                self.view_mode(),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_bridge_creation() {
        let bridge = GameBridge::new();
        assert_eq!(bridge.dungeon_level(), 0);
        assert!(!bridge.is_synced());
        assert_eq!(bridge.view_mode(), ViewMode::TopDown);
    }

    #[test]
    fn test_camera_switching() {
        let mut bridge = GameBridge::new();
        
        bridge.set_view_mode(ViewMode::Isometric);
        assert_eq!(bridge.view_mode(), ViewMode::Isometric);

        bridge.set_view_mode(ViewMode::FirstPerson);
        assert_eq!(bridge.view_mode(), ViewMode::FirstPerson);
    }

    #[test]
    fn test_world_access() {
        let bridge = GameBridge::new();
        
        // Verify the player entity exists
        let player = bridge.world().player();
        assert_eq!(player.glyph, '@');
        assert_eq!(bridge.world().entities.len(), 1);
    }
}
