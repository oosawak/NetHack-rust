//! Safe accessors for NetHack C global variables
//!
//! This module provides safe Rust wrappers for accessing C global variables,
//! ensuring thread safety and proper lifetimes.

// Import the auto-generated FFI symbols
// (These are defined in the FFI include! at the crate level)
use crate::dlevel;
use crate::dunlevs;

/// Player state from C global `u`
/// 
/// This is an opaque struct - we don't expose the full C structure definition
/// to avoid complex FFI. Instead, we provide accessor functions.
#[repr(C)]
pub struct PlayerState {
    // Opaque - access through functions only
    _private: [u8; 0],
}

/// Monster entry from C global `fmon`
#[repr(C)]
pub struct Monster {
    // Opaque - access through functions only
    _private: [u8; 0],
}

/// Object/item entry from C global `fobj`
#[repr(C)]
pub struct GameObject {
    // Opaque - access through functions only
    _private: [u8; 0],
}

/// Dungeon topology from C global `dungeon`
#[repr(C)]
pub struct DungeonTopology {
    // Opaque - access through functions only
    _private: [u8; 0],
}

/// Safe wrapper for player position and attributes
#[derive(Debug, Clone, Copy)]
pub struct PlayerInfo {
    pub x: i32,
    pub y: i32,
    pub level: i32,
    pub hp: i32,
    pub max_hp: i32,
    pub level_num: i32,
    pub experience: i32,
}

impl PlayerInfo {
    /// Get the current player position (x, y)
    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    /// Get the current player health
    pub fn health(&self) -> (i32, i32) {
        (self.hp, self.max_hp)
    }

    /// Get the current player level
    pub fn level(&self) -> i32 {
        self.level_num
    }

    /// Get the current player experience
    pub fn experience(&self) -> i32 {
        self.experience
    }
}

/// Get player info from C global `u`
///
/// Returns None if the game hasn't been initialized yet.
/// This function is safe - it reads from a C global variable but
/// only returns simple Copy types.
pub fn get_player_info() -> Option<PlayerInfo> {
    // This will be implemented in Phase 3.1
    // For now, we return None as a placeholder
    None
}

/// Get dungeon level from C global `dlevel`
pub fn get_current_level() -> i32 {
    unsafe {
        dlevel
    }
}

/// Get total dungeon levels from C global `dunlevs`
pub fn get_total_levels() -> i32 {
    unsafe {
        dunlevs
    }
}

/// Get the number of monsters on the current level
pub fn get_monster_count() -> i32 {
    // This will be implemented in Phase 3.1
    // Needs to walk the `fmon` list and count
    0
}

/// Get the number of objects on the current level
pub fn get_object_count() -> i32 {
    // This will be implemented in Phase 3.1
    // Needs to walk the `fobj` list and count
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_info() {
        let info = PlayerInfo {
            x: 10,
            y: 15,
            level: 5,
            hp: 40,
            max_hp: 50,
            level_num: 1,
            experience: 5000,
        };

        assert_eq!(info.position(), (10, 15));
        assert_eq!(info.health(), (40, 50));
        assert_eq!(info.level(), 1);
        assert_eq!(info.experience(), 5000);
    }

    #[test]
    fn test_dungeon_level_access() {
        // This test just verifies the function can be called
        // The actual value depends on whether the game is initialized
        let _ = get_current_level();
        let _ = get_total_levels();
    }
}
