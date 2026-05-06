//! 3D World state - unified representation for all view modes
//! 
//! All entities exist in 3D space. Camera position/angle determines visual style.

use crate::camera::{Camera3D, ViewMode};
use crate::dungeon::Level;
use crate::rng::Rng;
use glam::Vec3;
use serde::{Deserialize, Serialize};

/// Entity in the 3D world
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub position: Vec3,
    pub glyph: char,  // ASCII representation
    pub color: [f32; 4],
    pub name: String,
}

impl Entity {
    pub fn new(x: f32, y: f32, z: f32, glyph: char) -> Self {
        Self {
            position: Vec3::new(x, y, z),
            glyph,
            color: [1.0, 1.0, 1.0, 1.0],
            name: glyph.to_string(),
        }
    }
}

/// 3D World - unified state for all view modes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World3D {
    /// Current dungeon level
    pub level: Level,
    
    /// All entities in the world
    pub entities: Vec<Entity>,
    
    /// Player entity index
    pub player_idx: usize,
    
    /// Current depth (for multi-level dungeons)
    pub depth: u32,
    
    /// Turn count
    pub turns: u64,
}

impl World3D {
    /// Create a new world with a generated dungeon
    pub fn new(width: usize, height: usize) -> Self {
        let mut level = Level::new(width, height);
        let mut rng = Rng::new(42);
        level.generate(&mut rng);

        // Place player at center of first room
        let (px, py) = level.first_room_center();
        let player_pos = Vec3::new(px as f32, 0.0, py as f32);

        let entities = vec![
            Entity {
                position: player_pos,
                glyph: '@',
                color: [1.0, 1.0, 0.0, 1.0],
                name: "Player".to_string(),
            }
        ];

        Self {
            level,
            entities,
            player_idx: 0,
            depth: 1,
            turns: 0,
        }
    }
    
    /// Get player entity
    pub fn player(&self) -> &Entity {
        &self.entities[self.player_idx]
    }
    
    /// Get mutable player entity
    pub fn player_mut(&mut self) -> &mut Entity {
        &mut self.entities[self.player_idx]
    }
    
    /// Move player in 3D space (with wall collision)
    pub fn move_player(&mut self, dx: f32, dy: f32, dz: f32) {
        let new_pos = self.player().position + Vec3::new(dx, dy, dz);
        let nx = new_pos.x as usize;
        let nz = new_pos.z as usize;

        // Only move onto walkable tiles within bounds
        if new_pos.x >= 0.0 && new_pos.x < self.level.width as f32
            && new_pos.z >= 0.0 && new_pos.z < self.level.height as f32
            && self.level.is_walkable(nx, nz)
        {
            self.player_mut().position = new_pos;
            self.turns += 1;
        }
    }
    
    /// Add entity to world
    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }
    
    /// Get camera for a view mode (automatically positioned at player)
    pub fn get_camera(&self, mode: ViewMode) -> Camera3D {
        let player = self.player();
        Camera3D::new(
            player.position.x,
            player.position.y,
            player.position.z,
            mode,
        )
    }
    
    /// Switch view mode (returns new camera)
    pub fn switch_view(&self, mode: ViewMode) -> Camera3D {
        self.get_camera(mode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_creation() {
        let world = World3D::new(80, 25);
        assert_eq!(world.depth, 1);
        assert_eq!(world.entities.len(), 1);
        assert_eq!(world.player().glyph, '@');
    }

    #[test]
    fn test_player_movement() {
        let mut world = World3D::new(80, 25);
        let initial_pos = world.player().position;
        
        world.move_player(1.0, 0.0, 0.0);
        
        assert_eq!(world.player().position.x, initial_pos.x + 1.0);
        assert_eq!(world.turns, 1);
    }

    #[test]
    fn test_camera_switching() {
        let world = World3D::new(80, 25);
        
        let cam_topdown = world.get_camera(ViewMode::TopDown);
        assert_eq!(cam_topdown.mode, ViewMode::TopDown);
        
        let cam_iso = world.get_camera(ViewMode::Isometric);
        assert_eq!(cam_iso.mode, ViewMode::Isometric);
    }
}
