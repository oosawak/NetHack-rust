//! Game state to vertex data conversion
//! 
//! This module converts game state (player, dungeon, monsters, items)
//! into vertex buffers for rendering.

use bytemuck::{Pod, Zeroable};

/// Vertex data for rendering (shared with nethack-render)
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct RenderVertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

/// Game renderer that generates vertices from game state
pub struct GameRenderer {
    /// All vertices for current frame
    vertices: Vec<RenderVertex>,
    /// Index buffer (if needed)
    indices: Vec<u16>,
}

impl GameRenderer {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    /// Generate vertices from game state
    /// 
    /// This is called each frame to update vertex data based on:
    /// - Player position
    /// - Dungeon layout
    /// - Monsters (desktop only)
    /// - Items (desktop only)
    pub fn update_from_game_state(
        &mut self,
        player_x: i32,
        player_y: i32,
        level: &crate::dungeon::Level,
    ) {
        self.vertices.clear();
        self.indices.clear();

        // Draw dungeon tiles first (background)
        self.add_dungeon_tiles(level);

        // Draw monsters from C library (desktop only)
        #[cfg(not(target_arch = "wasm32"))]
        self.add_monsters_from_c();

        // Draw items from C library (desktop only)
        #[cfg(not(target_arch = "wasm32"))]
        self.add_items_from_c();

        // Draw traps from C library (desktop only)
        #[cfg(not(target_arch = "wasm32"))]
        self.add_traps_from_c();

        // Draw stairs from C library (desktop only)
        #[cfg(not(target_arch = "wasm32"))]
        self.add_stairs_from_c();

        // Draw player last (on top of everything)
        self.add_player_tile(player_x as f32, player_y as f32);
    }

    /// Add player as a visible tile (top-down 2D)
    fn add_player_tile(&mut self, x: f32, y: f32) {
        let player_color = [1.0, 1.0, 0.0, 1.0]; // Bright Yellow
        self.add_tile(x, y, 1.0, 0.0, player_color);
    }

    /// Add dungeon tiles with color-coded types
    fn add_dungeon_tiles(&mut self, level: &crate::dungeon::Level) {
        use crate::dungeon::Tile;

        for ty in 0..level.height {
            for tx in 0..level.width {
                let color = match level.tiles[ty][tx] {
                    Tile::Empty    => continue, // Don't draw — black background shows through
                    Tile::Wall     => [0.8, 0.8, 0.8, 1.0], // Light gray (stone wall)
                    Tile::Floor    => [0.45, 0.32, 0.18, 1.0], // Brown (dungeon floor)
                    Tile::Corridor => [0.30, 0.22, 0.12, 1.0], // Dark brown (corridor)
                    Tile::Stairs   => [0.2, 0.8, 0.2, 1.0],  // Green (stairs)
                    Tile::Trap     => [0.8, 0.2, 0.2, 1.0],  // Red (trap)
                };
                self.add_tile(tx as f32, ty as f32, 1.0, 0.0, color);
            }
        }
    }

    /// Add a single dungeon tile (as a simple rectangle)
    fn add_tile(
        &mut self,
        x: f32,
        y: f32,
        size: f32,
        height: f32,
        color: [f32; 4],
    ) {
        let start_idx = self.vertices.len() as u16;

        let half = size / 2.0;
        let x1 = x - half;
        let x2 = x + half;
        let z1 = y - half;
        let z2 = y + half;

        // Add 4 vertices for tile quad
        self.add_vertex(x1, height, z1, color);
        self.add_vertex(x2, height, z1, color);
        self.add_vertex(x2, height, z2, color);
        self.add_vertex(x1, height, z2, color);

        // Add 2 triangles to form quad
        self.add_triangle(start_idx + 0, start_idx + 1, start_idx + 2);
        self.add_triangle(start_idx + 0, start_idx + 2, start_idx + 3);
    }

    /// Helper: add a vertex
    fn add_vertex(&mut self, x: f32, y: f32, z: f32, color: [f32; 4]) {
        self.vertices.push(RenderVertex {
            position: [x, y, z],
            color,
        });
    }

    /// Helper: add a triangle (via indices)
    fn add_triangle(&mut self, i1: u16, i2: u16, i3: u16) {
        self.indices.push(i1);
        self.indices.push(i2);
        self.indices.push(i3);
    }

    /// Get current vertices
    pub fn vertices(&self) -> &[RenderVertex] {
        &self.vertices
    }

    /// Get current indices
    pub fn indices(&self) -> &[u16] {
        &self.indices
    }

    /// Get vertex count
    pub fn vertex_count(&self) -> u32 {
        self.vertices.len() as u32
    }

    /// Get index count
    pub fn index_count(&self) -> u32 {
        self.indices.len() as u32
    }

    /// Add monsters from C library (if available)
    #[cfg(not(target_arch = "wasm32"))]
    fn add_monsters_from_c(&mut self) {
        // Call C wrapper functions to get monsters from the game library
        // These are safe wrappers around the C fmon linked list
        
        let count = unsafe { nethack_sys::get_monster_count() };
        
        for i in 0..count {
            let mut monster_data: nethack_sys::monster_data_t = unsafe { std::mem::zeroed() };
            let result = unsafe { nethack_sys::get_monster_by_index(i, &mut monster_data) };
            
            if result != 0 {
                let x = monster_data.x as f32;
                let y = monster_data.y as f32;
                
                // Render as colored cube
                let color = if monster_data.is_peaceful != 0 {
                    [1.0, 1.0, 0.0, 1.0]  // Yellow for peaceful
                } else {
                    [1.0, 0.0, 0.0, 1.0]  // Red for hostile
                };
                
                self.add_creature_cube(x, y, 0.3, color);
                
                // Debug logging (once per 10 monsters to avoid spam)
                if i % 10 == 0 {
                    tracing::debug!("Rendered monster {} at ({}, {}) - peaceful={}", 
                                   i, monster_data.x, monster_data.y, monster_data.is_peaceful);
                }
            }
        }
        
        if count > 0 {
            tracing::info!("Rendered {} monsters from C library", count);
        }
    }

    /// Add items from C library (if available)
    #[cfg(not(target_arch = "wasm32"))]
    fn add_items_from_c(&mut self) {
        let count = unsafe { nethack_sys::get_object_count() };
        
        for i in 0..count {
            let mut object_data: nethack_sys::object_data_t = unsafe { std::mem::zeroed() };
            let result = unsafe { nethack_sys::get_object_by_index(i, &mut object_data) };
            
            if result != 0 {
                let x = object_data.x as f32;
                let y = object_data.y as f32;
                
                // Render as small cyan cube (different from monsters)
                let color = [0.0, 1.0, 1.0, 1.0];  // Cyan for items
                self.add_creature_cube(x, y, 0.2, color);  // Smaller than monsters
                
                // Debug logging (once per 10 items to avoid spam)
                if i % 10 == 0 {
                    tracing::debug!("Rendered item {} at ({}, {})", 
                                   i, object_data.x, object_data.y);
                }
            }
        }
        
        if count > 0 {
            tracing::info!("Rendered {} items from C library", count);
        }
    }

    /// Add a small cube for creatures/items
    fn add_creature_cube(&mut self, x: f32, y: f32, size: f32, color: [f32; 4]) {
        let half = size / 2.0;
        let height = 0.2;  // Slightly above floor
        
        // Bottom face
        let v_start = self.vertices.len() as u16;
        self.add_vertex(x - half, height, y - half, color);
        self.add_vertex(x + half, height, y - half, color);
        self.add_vertex(x + half, height, y + half, color);
        self.add_vertex(x - half, height, y + half, color);
        
        // Top face
        let height_top = height + size;
        self.add_vertex(x - half, height_top, y - half, color);
        self.add_vertex(x + half, height_top, y - half, color);
        self.add_vertex(x + half, height_top, y + half, color);
        self.add_vertex(x - half, height_top, y + half, color);
        
        // Create 12 triangles (2 per face × 6 faces)
        // Bottom: 0,1,2 0,2,3
        self.add_triangle(v_start + 0, v_start + 1, v_start + 2);
        self.add_triangle(v_start + 0, v_start + 2, v_start + 3);
        // Top: 4,6,5 4,7,6
        self.add_triangle(v_start + 4, v_start + 6, v_start + 5);
        self.add_triangle(v_start + 4, v_start + 7, v_start + 6);
        // Front: 0,5,1 0,4,5
        self.add_triangle(v_start + 0, v_start + 5, v_start + 1);
        self.add_triangle(v_start + 0, v_start + 4, v_start + 5);
        // Back: 2,7,3 2,6,7
        self.add_triangle(v_start + 2, v_start + 7, v_start + 3);
        self.add_triangle(v_start + 2, v_start + 6, v_start + 7);
        // Left: 3,4,0 3,7,4
        self.add_triangle(v_start + 3, v_start + 4, v_start + 0);
        self.add_triangle(v_start + 3, v_start + 7, v_start + 4);
        // Right: 1,5,2 2,5,6
        self.add_triangle(v_start + 1, v_start + 5, v_start + 2);
        self.add_triangle(v_start + 2, v_start + 5, v_start + 6);
    }

    /// Add traps from C library
    #[cfg(not(target_arch = "wasm32"))]
    fn add_traps_from_c(&mut self) {
        let count = unsafe { nethack_sys::get_trap_count() };
        
        for i in 0..count {
            let mut trap_data: nethack_sys::trap_data_t = unsafe { std::mem::zeroed() };
            let result = unsafe { nethack_sys::get_trap_by_index(i, &mut trap_data) };
            
            if result != 0 {
                let x = trap_data.x as f32;
                let y = trap_data.y as f32;
                
                // Render as tiny purple cube (distinct from other entities)
                let color = [0.8, 0.0, 0.8, 1.0];  // Purple for traps
                self.add_creature_cube(x, y, 0.15, color);  // Smallest size
                
                // Debug logging (once per 20 traps to avoid spam)
                if i % 20 == 0 {
                    tracing::debug!("Rendered trap {} (type {}) at ({}, {})", 
                                   i, trap_data.trap_type, trap_data.x, trap_data.y);
                }
            }
        }
        
        if count > 0 {
            tracing::info!("Rendered {} traps from C library", count);
        }
    }

    /// Add stairs from C library
    #[cfg(not(target_arch = "wasm32"))]
    fn add_stairs_from_c(&mut self) {
        let count = unsafe { nethack_sys::get_stair_count() };
        
        for i in 0..count {
            let mut stair_data: nethack_sys::stair_data_t = unsafe { std::mem::zeroed() };
            let result = unsafe { nethack_sys::get_stair_by_index(i, &mut stair_data) };
            
            if result != 0 {
                let x = stair_data.x as f32;
                let y = stair_data.y as f32;
                
                // Render as colored cube based on direction
                let color = if stair_data.is_up != 0 {
                    [0.0, 1.0, 0.0, 1.0]  // Green for stairs up
                } else {
                    [0.0, 0.0, 1.0, 1.0]  // Blue for stairs down
                };
                
                self.add_creature_cube(x, y, 0.25, color);  // Medium size
                
                // Debug logging
                let direction = if stair_data.is_up != 0 { "up" } else { "down" };
                let feature = if stair_data.is_ladder != 0 { "ladder" } else { "stairs" };
                tracing::debug!("Rendered {} ({}) at ({}, {})", 
                               feature, direction, stair_data.x, stair_data.y);
            }
        }
        
        if count > 0 {
            tracing::info!("Rendered {} stairs/ladders from C library", count);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_renderer_creation() {
        let renderer = GameRenderer::new();
        assert_eq!(renderer.vertex_count(), 0);
        assert_eq!(renderer.index_count(), 0);
    }

    #[test]
    fn test_player_cube_generation() {
        let mut renderer = GameRenderer::new();
        renderer.update_from_game_state(5, 5, 80, 24);

        // Should have player cube (8 vertices) + floor tiles
        assert!(renderer.vertex_count() > 8);
        assert!(renderer.index_count() > 0);
    }

    #[test]
    fn test_floor_tile_generation() {
        let mut renderer = GameRenderer::new();
        renderer.update_from_game_state(5, 5, 80, 24);

        // Check vertex count is reasonable (player + tiles)
        let vert_count = renderer.vertex_count();
        // Player: 8 verts, Tiles: 4 verts each, 11x11 area = 121 tiles
        // Estimated: 8 + (121 * 4) = 492 vertices
        assert!(vert_count >= 100);
    }
}
