use crate::{
    GroundVarient, Light, LightEngine, LightHandle, Tile, TileRotation, Wall, WallVarient,
};
use raylib::prelude::*;

pub struct WorldMap {
    pub ground: Vec<Vec<Tile>>,
    pub walls: Vec<Vec<Option<Wall>>>,
    pub width: u32,
    pub height: u32,
}

impl WorldMap {
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
        for y in 0..map_height {
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
                        dbg!(s, y, buffer);
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
        for y in 0..map_height {
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
                        dbg!(s, y, buffer);
                        panic!("Unable to parse map");
                    }
                })
                .collect::<Vec<Option<Wall>>>();
            walls.push(wall_map_line);
        }
        assert!(ground.len() == map_height as usize);
        dbg!(&walls);
        WorldMap {
            ground,
            walls,
            width: map_width,
            height: map_height,
        }
    }
}

pub struct DayCycle {
    time: f32,
    ambient_light_handle: LightHandle,
}

impl DayCycle {
    pub const FULL_CYCLE_LENGTH: f32 = 800.0;
    pub fn new(light_engine: &mut LightEngine) -> DayCycle {
        DayCycle {
            time: 0.25 * DayCycle::FULL_CYCLE_LENGTH,
            ambient_light_handle: light_engine.spawn_light(Light::default_ambient()),
        }
    }
    pub fn update(&mut self, rl: &mut RaylibHandle, light_engine: &mut LightEngine) {
        self.time += rl.get_frame_time();
        if self.time > Self::FULL_CYCLE_LENGTH {
            self.time -= Self::FULL_CYCLE_LENGTH;
        };
        if rl.is_key_pressed(KeyboardKey::KEY_SEVEN) {
            self.time = Self::FULL_CYCLE_LENGTH * 0.0;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_EIGHT) {
            self.time = Self::FULL_CYCLE_LENGTH * 0.25;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_NINE) {
            self.time = Self::FULL_CYCLE_LENGTH * 0.5;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_ZERO) {
            self.time = Self::FULL_CYCLE_LENGTH * 0.75;
        }
        light_engine.update_light(self.ambient_light_handle(), self.get_ambient_light());
    }
    pub fn ambient_light_handle(&self) -> &LightHandle {
        &self.ambient_light_handle
    }
    pub fn get_ambient_light(&self) -> Light {
        let normilized_time = self.time / Self::FULL_CYCLE_LENGTH;
        let sunrise_length = 1.0 / 10.0;

        //  Sun rising
        if self.time < Self::FULL_CYCLE_LENGTH * sunrise_length {
            let light_level = normilized_time / sunrise_length;
            Light::Ambient {
                color: Vector4::new(1.0, 1.0, 1.0, light_level),
            }
        }
        // Sun setting
        else if self.time
            >= Self::FULL_CYCLE_LENGTH * sunrise_length * (1.0 / sunrise_length - 2.0) / 2.0
            && normilized_time < 0.5
        {
            let light_level =
                (1.0 - normilized_time / sunrise_length) + (1.0 / sunrise_length - 2.0) / 2.0;
            Light::Ambient {
                color: Vector4::new(1.0, 1.0, 1.0, light_level),
            }
        }
        // Sun risen
        else if normilized_time < 0.5 {
            Light::Ambient {
                color: Color::WHITE.into(),
            }
        }
        // Sun set
        else {
            Light::Ambient {
                color: Color::BLACK.into(),
            }
        }
    }
    pub fn get_debug_info(&self) -> String {
        let hour = ((self.time / DayCycle::FULL_CYCLE_LENGTH + 0.25) * 24.0) as i32;
        let minute = (self.time / DayCycle::FULL_CYCLE_LENGTH * 24.0 * 60.0 % 60.0) as i32;
        format!(
            "Game Time: {}:{}{} {}",
            if hour % 12 == 0 { 12 } else { hour % 12 },
            if minute < 10 { "0" } else { "" },
            minute,
            if hour % 24 < 12 { "AM" } else { "PM" }
        )
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
    fn pan_to(&mut self, pos: Vector2, screen_size: Vector2);
    fn get_world_pos(&self, screen_size: Vector2) -> Vector2;
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
        let mut zoom = self.zoom;
        zoom *= 1.0 + rl.get_mouse_wheel_move() / 40.0;

        if rl.is_key_down(KeyboardKey::KEY_MINUS) {
            zoom /= 1.04;
        }
        if rl.is_key_down(KeyboardKey::KEY_EQUAL) {
            zoom *= 1.04;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
            zoom = 1.0;
        }
        self.zoom = zoom;
    }

    fn track(&mut self, world_pos: Vector2, screen_size: Vector2) {
        self.offset = -world_pos + screen_size / 2.0 / self.zoom;
    }

    fn get_world_pos(&self, screen_size: Vector2) -> Vector2 {
        -self.offset + screen_size / (2.0 * self.zoom)
    }

    fn pan_to(&mut self, world_pos: Vector2, screen_size: Vector2) {
        let dist = world_pos - self.get_world_pos(screen_size);
        self.track(self.get_world_pos(screen_size) + dist / 10.0, screen_size);
    }
}
