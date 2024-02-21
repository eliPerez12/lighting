
pub struct Wall {
    pub varient: u32,
    pub rotation: TileRot,
    pub colliders: Vec<Rectangle>,
}

pub struct Tile {
    pub varient: u32,
    pub rotation: TileRot,
}

pub enum TileRot {
    None,
    One,
    Two,
    Three,
}