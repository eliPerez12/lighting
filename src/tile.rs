use raylib::prelude::*;

use crate::TILE_SIZE;

pub struct Wall {
    pub varient: u32,
    pub rotation: TileRotation,
    pub colliders: Vec<Rectangle>,
}

impl Wall {
    pub fn get_collider(&self) -> Collider {
        match self.varient {
            _ => Collider {
                rects: vec![self.rotation.get_collider_rect()],
            },
        }
    }
}

pub struct Tile {
    pub varient: u32,
    pub rotation: TileRotation,
}

pub enum TileRotation {
    None,
    One,
    Two,
    Three,
}

pub struct Collider {
    pub rects: Vec<Rectangle>,
}

impl Tile {}

impl TileRotation {
    pub fn from_raw_u32(tile: u32) -> TileRotation {
        let flags = tile >> 28; // Get first byte from tile
        match flags {
            0x0 => Self::None,
            0x6 => Self::One,
            0xA => Self::Two,
            0xC => Self::Three,
            _ => panic!("Tile rotation data corrupted"),
        }
    }

    pub fn get_angle(&self) -> f32 {
        match self {
            TileRotation::None => 0.0,
            TileRotation::One => 270.0,
            TileRotation::Two => 90.0,
            TileRotation::Three => 180.0,
        }
    }

    pub fn get_rotation_offset(&self) -> Vector2 {
        match self {
            TileRotation::None => Vector2::zero(),
            TileRotation::One => Vector2::new(0.0, TILE_SIZE),
            TileRotation::Two => Vector2::new(TILE_SIZE, 0.0),
            TileRotation::Three => Vector2::new(TILE_SIZE, TILE_SIZE),
        }
    }

    fn get_collider_rect(&self) -> Rectangle {
        match self {
            _ => Rectangle {
                x: 0.0,
                y: 0.0,
                width: 8.0,
                height: 32.0,
            },
        }
    }
}
