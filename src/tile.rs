use crate::TILE_SIZE;
use raylib::prelude::*;

#[derive(Debug)]
pub struct Wall {
    pub varient: WallVarient,
    pub rotation: TileRotation,
}

#[derive(Debug, Clone, Copy)]
pub enum WallVarient {
    Staight,
    Elbow,
}

#[derive(Clone, Copy, Debug)]
pub enum GroundVarient {
    Dirt,
    Wood,
}

impl GroundVarient {
    pub fn from_raw_u32(ground: u32) -> Option<GroundVarient> {
        match (ground - 1) & 0x0FFFFFBF {
            // Remove first byte and 64 bit
            0 => Some(GroundVarient::Dirt),
            1 => Some(GroundVarient::Wood),
            _ => None,
        }
    }
}

impl WallVarient {
    pub fn from_raw_u32(wall: u32) -> Option<WallVarient> {
        match (wall - 1) & 0x0FFFFFBF {
            // Remove first byte and 64 bit
            0 => Some(WallVarient::Staight),
            1 => Some(WallVarient::Elbow),
            _ => None,
        }
    }
}

impl Wall {
    pub fn get_collider(&self) -> Collider {
        match self.varient {
            WallVarient::Staight => Collider {
                rects: vec![self.rotation.get_collider_rect()],
                circles: vec![],
            },
            WallVarient::Elbow => Collider {
                rects: vec![
                    self.rotation.get_collider_rect(),
                    self.rotation.rotate().get_collider_rect(),
                ],
                circles: vec![],
            },
        }
    }
}

pub struct Tile {
    pub varient: GroundVarient,
    pub rotation: TileRotation,
}

#[derive(Debug)]
pub enum TileRotation {
    None,
    One,
    Two,
    Three,
}

#[derive(Debug)]
pub struct Collider {
    pub rects: Vec<Rectangle>,
    pub circles: Vec<Circle>,
}

impl Collider {
    pub fn collides(&self, other_collider: &Self) -> Option<Rectangle> {
        for self_rect in self.rects.iter() {
            for other_rect in other_collider.rects.iter() {
                let collision = self_rect.get_collision_rec(other_rect);
                if collision.is_some() {
                    return collision;
                }
            }
        }
        None
    }

    pub fn with_pos(&self, pos: Vector2) -> Collider {
        Collider {
            rects: self
                .rects
                .iter()
                .map(|rect| Rectangle {
                    x: rect.x + pos.x,
                    y: rect.y + pos.y,
                    width: rect.width,
                    height: rect.height,
                })
                .collect(),

            circles: self
                .circles
                .iter()
                .map(|circle| Circle {
                    x: circle.x + pos.x,
                    y: circle.y + pos.y,
                    radius: circle.radius,
                })
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

impl Circle {}

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
            TileRotation::None => Rectangle {
                x: 0.0,
                y: 0.0,
                width: 10.0,
                height: 32.0,
            },
            TileRotation::Two => Rectangle {
                x: 0.0,
                y: 0.0,
                width: 32.0,
                height: 10.0,
            },
            TileRotation::One => Rectangle {
                x: 0.0,
                y: 22.0,
                width: 32.0,
                height: 10.0,
            },
            TileRotation::Three => Rectangle {
                x: 22.0,
                y: 0.0,
                width: 10.0,
                height: 32.0,
            },
        }
    }

    fn rotate(&self) -> TileRotation {
        match self {
            TileRotation::None => TileRotation::One,
            TileRotation::One => TileRotation::Three,
            TileRotation::Two => TileRotation::None,
            TileRotation::Three => TileRotation::Two,
        }
    }
}
