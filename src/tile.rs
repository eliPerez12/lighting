use crate::TILE_SIZE;
use raylib::prelude::*;

#[derive(Debug)]
pub struct Wall {
    pub varient: WallVarient,
    pub rotation: TileRotation,
}

#[derive(Debug)]
pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WallVarient {
    Staight = 0,
    Elbow = 1,
    WhiteStraight = 2,
    WhiteElbow = 3,
    TinyElbow = 4,
    WhiteTinyElbow = 5,
    WhitePillar = 6,
}

#[derive(Clone, Copy, Debug)]
pub enum GroundVarient {
    Dirt = 0,
    Wood = 1,
    Grass = 2,
    DirtQuarterEdge = 5,
    DirtThreeQuarterEdge = 6,
    DirtHalfEdge = 7,
}

impl GroundVarient {
    // Used for parsing map data
    pub fn from_raw_u32(ground: u32) -> Option<GroundVarient> {
        match (ground - 1) & 0x0FFFFFBF {
            // Remove first byte and 64 bit
            0 => Some(GroundVarient::Dirt),
            1 => Some(GroundVarient::Wood),
            2 => Some(GroundVarient::Grass),
            5 => Some(GroundVarient::DirtQuarterEdge),
            6 => Some(GroundVarient::DirtThreeQuarterEdge),
            7 => Some(GroundVarient::DirtHalfEdge),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct Line {
    pub start: Vector2,
    pub end: Vector2,
}

pub fn cross(vector: Vector2, other_vector: Vector2) -> f32 {
    vector.x * other_vector.y - vector.y * other_vector.x
}

impl Line {
    pub fn intersection(&self, other: &Line) -> Option<Vector2> {
        let p = self.start;
        let q = other.start;
        let r = self.end - self.start;
        let s = other.end - other.start;

        let r_cross_s = cross(r, s);
        if r_cross_s == 0.0 {
            // Lines are parallel or coincident
            return None;
        }

        let q_minus_p = q - p;
        let t = cross(q_minus_p, s) / r_cross_s;
        let u = cross(q_minus_p, r) / r_cross_s;

        if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u) {
            // Intersection within line segments
            Some(p + r * t)
        } else {
            None
        }
    }

    // Returns lines from a rectangle (Top, Bottom, Left, Right)
    pub fn from_rect(rect: &Rectangle) -> Vec<Line> {
        vec![
            Line {
                // Top
                start: Vector2::new(rect.x, rect.y),
                end: Vector2::new(rect.x + rect.width, rect.y),
            },
            Line {
                // Bottom
                start: Vector2::new(rect.x, rect.y + rect.height),
                end: Vector2::new(rect.x + rect.width, rect.y + rect.height),
            },
            Line {
                // Left
                start: Vector2::new(rect.x, rect.y),
                end: Vector2::new(rect.x, rect.y + rect.height),
            },
            Line {
                // Right
                start: Vector2::new(rect.x + rect.width, rect.y),
                end: Vector2::new(rect.x + rect.width, rect.y + rect.height),
            },
        ]
    }
}

impl WallVarient {
    // Used for parsing map data
    pub fn from_raw_u32(wall: u32) -> Option<WallVarient> {
        match (wall - 1) & 0x0FFFFFBF {
            // Remove first byte and 64 bit
            0 => Some(WallVarient::Staight),
            1 => Some(WallVarient::Elbow),
            2 => Some(WallVarient::WhiteStraight),
            3 => Some(WallVarient::WhiteElbow),
            4 => Some(WallVarient::TinyElbow),
            5 => Some(WallVarient::WhiteTinyElbow),
            6 => Some(WallVarient::WhitePillar),
            _ => None,
        }
    }
}

impl Wall {
    // TODO: Rework all of this
    pub fn get_collider(&self) -> Collider {
        match self.varient {
            WallVarient::Staight | WallVarient::WhiteStraight => Collider {
                rects: vec![self.rotation.get_collider_rect()],
            },
            WallVarient::Elbow | WallVarient::WhiteElbow => Collider {
                rects: vec![
                    self.rotation.get_collider_rect(),
                    self.rotation.rotate().get_collider_rect(),
                ],
            },
            WallVarient::TinyElbow | WallVarient::WhiteTinyElbow => Collider {
                rects: vec![match self.rotation {
                    TileRotation::None => Rectangle::new(0.0, 22.0, 10.0, 10.0),
                    TileRotation::One => Rectangle::new(22.0, 22.0, 10.0, 10.0),
                    TileRotation::Two => Rectangle::new(0.0, 0.0, 10.0, 10.0),
                    TileRotation::Three => Rectangle::new(22.0, 0.0, 10.0, 10.0),
                }],
            },
            WallVarient::WhitePillar => Collider {
                rects: vec![Rectangle::new(9.0, 9.0, 14.0, 14.0)],
            },
        }
    }
}

pub struct Ground {
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
}

impl Collider {
    pub fn collides(&self, other_collider: &Self) -> Option<Rectangle> {
        for self_rect in self.rects.iter() {
            for other_rect in other_collider.rects.iter() {
                let collision = self_rect.get_collision_rec(other_rect);
                if let Some(collision) = collision {
                    return Some(collision);
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
        }
    }
}

impl TileRotation {
    pub fn from_raw_u32(tile: u32) -> TileRotation {
        let flags = tile >> 28; // Get first byte from tile
        match flags {
            0x0 => Self::None,
            0x6 => Self::One,
            0xA => Self::Two,
            0xC => Self::Three,
            _ => panic!("Tile rotation data corrupted, flags: {:#02x}", flags),
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
