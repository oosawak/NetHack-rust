use serde::{Deserialize, Serialize};

/// Tile type in a dungeon level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tile {
    Empty,
    Wall,
    Floor,
    Corridor,
    Stairs,
    Trap,
}

/// Dungeon level representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Level {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Tile>>,
}

impl Level {
    /// Create a new empty level
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tiles: vec![vec![Tile::Empty; width]; height],
        }
    }

    /// Get tile at position
    pub fn get(&self, x: usize, y: usize) -> Option<Tile> {
        self.tiles.get(y).and_then(|row| row.get(x)).copied()
    }

    /// Set tile at position
    pub fn set(&mut self, x: usize, y: usize, tile: Tile) {
        if y < self.height && x < self.width {
            self.tiles[y][x] = tile;
        }
    }

    /// Check if a tile is walkable
    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        matches!(
            self.get(x, y),
            Some(Tile::Floor) | Some(Tile::Corridor) | Some(Tile::Stairs)
        )
    }
}
