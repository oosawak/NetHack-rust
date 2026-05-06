use serde::{Deserialize, Serialize};
use crate::rng::Rng;

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

    /// Generate a dungeon with rooms and connecting corridors
    pub fn generate(&mut self, rng: &mut Rng) {
        // Fill everything with empty
        for row in &mut self.tiles {
            for tile in row.iter_mut() {
                *tile = Tile::Empty;
            }
        }

        let mut rooms: Vec<(i32, i32, i32, i32)> = Vec::new(); // (x, y, w, h)

        // Try to place up to 12 rooms
        for _ in 0..50 {
            let rw = 6 + rng.next_u32(10) as i32;
            let rh = 4 + rng.next_u32(5) as i32;
            let max_rx = (self.width as i32 - rw - 2).max(1);
            let max_ry = (self.height as i32 - rh - 2).max(1);
            let rx = 1 + rng.next_u32(max_rx as u32) as i32;
            let ry = 1 + rng.next_u32(max_ry as u32) as i32;

            // Check overlap with existing rooms (1 tile gap)
            let overlaps = rooms.iter().any(|&(ox, oy, ow, oh)| {
                rx < ox + ow + 2 && rx + rw + 2 > ox && ry < oy + oh + 2 && ry + rh + 2 > oy
            });

            if !overlaps {
                rooms.push((rx, ry, rw, rh));
                if rooms.len() >= 12 {
                    break;
                }
            }
        }

        // Draw rooms: wall border + floor interior
        for &(rx, ry, rw, rh) in &rooms {
            for dx in 0..rw {
                for dy in 0..rh {
                    let x = (rx + dx) as usize;
                    let y = (ry + dy) as usize;
                    let on_edge = dx == 0 || dx == rw - 1 || dy == 0 || dy == rh - 1;
                    self.tiles[y][x] = if on_edge { Tile::Wall } else { Tile::Floor };
                }
            }
        }

        // Connect rooms with L-shaped corridors
        for i in 1..rooms.len() {
            let (x1, y1, w1, h1) = rooms[i - 1];
            let (x2, y2, w2, h2) = rooms[i];
            let cx1 = x1 + w1 / 2;
            let cy1 = y1 + h1 / 2;
            let cx2 = x2 + w2 / 2;
            let cy2 = y2 + h2 / 2;

            // Open exactly one wall tile on room1 (horizontal exit toward room2)
            let door1_x: i32 = if cx2 >= cx1 { x1 + w1 - 1 } else { x1 };
            self.tiles[cy1 as usize][door1_x as usize] = Tile::Floor;

            // Open exactly one wall tile on room2 (vertical entry from cy1 direction)
            let door2_y: i32 = if cy2 >= cy1 { y2 } else { y2 + h2 - 1 };
            self.tiles[door2_y as usize][cx2 as usize] = Tile::Floor;

            // Draw horizontal corridor through empty space only (from door1 to cx2 column)
            let h_start: i32 = if cx2 >= cx1 { door1_x + 1 } else { door1_x - 1 };
            let (hsx, hex) = if h_start <= cx2 { (h_start, cx2) } else { (cx2, h_start) };
            for x in hsx..=hex {
                let tile = &mut self.tiles[cy1 as usize][x as usize];
                if *tile == Tile::Empty {
                    *tile = Tile::Corridor;
                }
            }

            // Draw vertical corridor through empty space only (from cy1 to door2)
            let v_end: i32 = if cy2 >= cy1 { door2_y - 1 } else { door2_y + 1 };
            let (vsy, vey) = if cy1 <= v_end { (cy1, v_end) } else { (v_end, cy1) };
            for y in vsy..=vey {
                let tile = &mut self.tiles[y as usize][cx2 as usize];
                if *tile == Tile::Empty {
                    *tile = Tile::Corridor;
                }
            }
        }

        // Add stairs in last room
        if let Some(&(rx, ry, rw, rh)) = rooms.last() {
            let sx = (rx + rw / 2) as usize;
            let sy = (ry + rh / 2) as usize;
            self.tiles[sy][sx] = Tile::Stairs;
        }
    }

    /// Return the center of the first room (for player start position)
    pub fn first_room_center(&self) -> (usize, usize) {
        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                if self.tiles[y][x] == Tile::Floor {
                    return (x, y);
                }
            }
        }
        (self.width / 2, self.height / 2)
    }
}
