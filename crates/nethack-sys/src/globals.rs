//! Safe accessors for NetHack C global variables
//!
//! This module provides safe Rust wrappers for accessing C global variables,
//! ensuring thread safety and proper lifetimes.

/// Safe wrapper for player position and attributes
#[derive(Debug, Clone, Copy)]
pub struct PlayerInfo {
    pub x: i32,
    pub y: i32,
    pub level: i32,
    pub hp: i32,
    pub max_hp: i32,
    pub dungeon_level: i32,
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
        self.level
    }

    /// Get the current dungeon level
    pub fn dungeon_level(&self) -> i32 {
        self.dungeon_level
    }
}

/// Get player info from C global `u`
///
/// Uses the safe accessor functions from wrapper.c to read player state.
pub fn get_player_info() -> PlayerInfo {
    unsafe {
        // Call the safe C accessor functions
        PlayerInfo {
            x: crate::get_player_x(),
            y: crate::get_player_y(),
            level: crate::get_player_level(),
            hp: crate::get_player_hp(),
            max_hp: crate::get_player_maxhp(),
            dungeon_level: crate::dlevel,
        }
    }
}

/// Get player X coordinate
pub fn get_x() -> i32 {
    unsafe {
        crate::get_player_x()
    }
}

/// Get player Y coordinate
pub fn get_y() -> i32 {
    unsafe {
        crate::get_player_y()
    }
}

/// Get player level
pub fn get_level() -> i32 {
    unsafe {
        crate::get_player_level()
    }
}

/// Get player current HP
pub fn get_hp() -> i32 {
    unsafe {
        crate::get_player_hp()
    }
}

/// Get player max HP
pub fn get_maxhp() -> i32 {
    unsafe {
        crate::get_player_maxhp()
    }
}

/// Get dungeon level from C global `dlevel`
pub fn get_current_level() -> i32 {
    unsafe {
        crate::get_dlevel()
    }
}

/// Get total dungeon levels from C global `dunlevs`
pub fn get_total_levels() -> i32 {
    unsafe {
        crate::get_dunlevs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_info_creation() {
        let info = PlayerInfo {
            x: 10,
            y: 15,
            level: 5,
            hp: 40,
            max_hp: 50,
            dungeon_level: 1,
        };

        assert_eq!(info.position(), (10, 15));
        assert_eq!(info.health(), (40, 50));
        assert_eq!(info.level(), 5);
        assert_eq!(info.dungeon_level(), 1);
    }
}
