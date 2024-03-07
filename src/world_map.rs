use crate::{GroundVarient, Tile, TileRotation, Wall, WallVarient};
use raylib::prelude::*;

pub struct WorldMap {
    pub ground: Vec<Vec<Tile>>,
    pub walls: Vec<Vec<Option<Wall>>>,
    pub width: u32,
    pub height: u32,
}

impl WorldMap {
    // Load a world map from .tmx file from Tiled
    pub fn load_from_file(path: &str, map_width: u32, map_height: u32) -> WorldMap {
        use std::io::BufRead;

        let mut ground = vec![];
        let mut walls = vec![];
        let map = std::fs::File::open(path).unwrap();
        let mut reader = std::io::BufReader::new(map);
        // Skip first 6 lines of map data
        for _ in 0..6 {
            reader.read_line(&mut String::new()).unwrap();
        }
        // Parsing background layer
        for _ in 0..map_height {
            let mut buffer = String::new();
            reader.read_line(&mut buffer).unwrap();
            let buffer = &buffer.replace("\r\n", "");
            let floor_map_line = buffer
                .split(',')
                .filter(|s| s != &"")
                .map(|s| {
                    if let Ok(ground) = s.parse::<u32>() {
                        if let Some(varient) = GroundVarient::from_raw_u32(ground) {
                            Tile {
                                varient, // Removing 64 bit and first byte
                                rotation: TileRotation::from_raw_u32(ground),
                            }
                        } else {
                            panic!("Unable to parse map");
                        }
                    } else {
                        panic!("Unable to parse map");
                    }
                })
                .collect::<Vec<Tile>>();
            ground.push(floor_map_line);
        }
        // Skipping another 4 lines
        for _ in 0..4 {
            reader.read_line(&mut String::new()).unwrap();
        }
        // Parsing wall layer
        for _ in 0..map_height {
            let mut buffer = String::new();
            reader.read_line(&mut buffer).unwrap();
            let buffer = &buffer.replace("\r\n", "");
            let wall_map_line = buffer
                .split(',')
                .filter(|s| s != &"")
                .map(|s| {
                    if let Ok(wall) = s.parse::<u32>() {
                        if wall != 0 {
                            WallVarient::from_raw_u32(wall).map(|varient| Wall {
                                varient, // Removing 64 bit and first byte
                                rotation: TileRotation::from_raw_u32(wall),
                            })
                        } else {
                            None
                        }
                    } else {
                        panic!("Unable to parse map");
                    }
                })
                .collect::<Vec<Option<Wall>>>();
            walls.push(wall_map_line);
        }
        assert!(ground.len() == map_height as usize);
        assert!(walls.len() == map_height as usize);
        WorldMap {
            ground,
            walls,
            width: map_width,
            height: map_height,
        }
    }

    pub fn collides_with_wall(&self, collider: &crate::Collider) -> Option<Rectangle> {
        // Iterate over every wall, and every collider rect in each wall collider
        for (y, wall_line) in self.walls.iter().enumerate() {
            for (x, wall) in wall_line.iter().enumerate() {
                if let Some(wall) = wall {
                    let wall_collider = wall
                        .get_collider()
                        .with_pos(Vector2::new(x as f32 * 32.0, y as f32 * 32.0));
                    if let Some(collision) = wall_collider.collides(collider) {
                        return Some(collision);
                    }
                }
            }
        }
        None
    }
}

// Adding additional methods to raylib camera2d
pub trait ImprovedCamera {
    fn to_screen(&self, world_pos: Vector2) -> Vector2;
    fn to_screen_x(&self, world_pos_x: f32) -> f32;
    fn to_screen_y(&self, world_pos_y: f32) -> f32;
    fn to_screen_rect(&self, rect: &Rectangle) -> Rectangle;
    fn to_world(&self, screen_pos: Vector2) -> Vector2;
    fn handle_player_controls(&mut self, rl: &mut RaylibHandle);
    fn track(&mut self, pos: Vector2, screen_size: Vector2);
    fn pan_to(&mut self, rl: &RaylibHandle, pos: Vector2, screen_size: Vector2);
    fn get_world_pos(&self, offset: Vector2, screen_size: Vector2) -> Vector2;
    fn get_screen_offset(&self, world_pos: Vector2, screen_size: Vector2) -> Vector2;
}

impl ImprovedCamera for Camera2D {
    fn to_screen(&self, world_pos: Vector2) -> Vector2 {
        (world_pos + self.offset) * self.zoom
    }

    fn to_screen_x(&self, world_pos_x: f32) -> f32 {
        (world_pos_x + self.offset.x) * self.zoom
    }

    fn to_screen_y(&self, world_pos_y: f32) -> f32 {
        (world_pos_y + self.offset.y) * self.zoom
    }

    fn to_screen_rect(&self, rect: &Rectangle) -> Rectangle {
        Rectangle {
            x: (rect.x + self.offset.x) * self.zoom,
            y: (rect.y + self.offset.y) * self.zoom,
            width: rect.width * self.zoom,
            height: rect.height * self.zoom,
        }
    }

    fn to_world(&self, screen_pos: Vector2) -> Vector2 {
        (screen_pos / self.zoom) - self.offset
    }

    fn handle_player_controls(&mut self, rl: &mut RaylibHandle) {
        let screen_size = Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32);
        let mouse_wheel_move = rl.get_mouse_wheel_move();

        if mouse_wheel_move != 0.0 {
            let old_world_pos = self.get_world_pos(self.offset, screen_size);
            self.zoom *= 1.0 + rl.get_mouse_wheel_move() / 40.0;
            self.track(old_world_pos, screen_size);
        }
    }

    fn track(&mut self, target_world_pos: Vector2, screen_size: Vector2) {
        self.offset = self.get_screen_offset(target_world_pos, screen_size);
    }

    fn get_world_pos(&self, offset: Vector2, screen_size: Vector2) -> Vector2 {
        -offset + screen_size / (2.0 * self.zoom)
    }

    fn get_screen_offset(&self, world_pos: Vector2, screen_size: Vector2) -> Vector2 {
        -world_pos + screen_size / 2.0 / self.zoom
    }

    fn pan_to(&mut self, rl: &RaylibHandle, target_pos: Vector2, screen_size: Vector2) {
        let camera_pan_time = 9.0;
        let old_pos = self.get_world_pos(self.offset, screen_size);
        let pos = old_pos + (target_pos - old_pos) / (camera_pan_time / rl.get_frame_time() / 60.0);
        self.offset = self.get_screen_offset(pos, screen_size);
    }
}
