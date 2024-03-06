use crate::{
    day_cycle::DayCycle, Collider, ImprovedCamera, LightEngine, Line, Player, WorldMap, TILE_SIZE,
};
use raylib::prelude::*;

pub struct Bullet {
    pub pos_history: [Vector2; 3],
    pub pos: Vector2,
    pub vel: Vector2,
    pub collided: bool,
    pub dbg_lines: Vec<Line>,
    pub dbg_line_hit: Option<Line>,
}

impl Bullet {
    pub fn new(pos: Vector2, vel: Vector2) -> Bullet {
        Bullet {
            pos,
            vel,
            collided: false,
            pos_history: [pos; 3],
            dbg_lines: vec![],
            dbg_line_hit: None,
        }
    }

    pub fn update_history(&mut self) {
        self.pos_history[2] = self.pos_history[1];
        self.pos_history[1] = self.pos_history[0];
        self.pos_history[0] = self.pos;
    }

    pub fn update(&mut self, rl: &RaylibHandle, world_map: &WorldMap) {
        self.collided = false;
        self.dbg_line_hit = None;
        self.update_history();
        let drag = 0.0;

        self.vel -= self.vel.normalized() * drag * rl.get_frame_time() * 60.0;
        if self.vel.length() <= 6.0 {
            self.vel = Vector2::zero();
        }
        self.handle_collisions(rl, world_map);
        if !self.collided {
            self.pos += self.vel * rl.get_frame_time();
        }
    }

    pub fn handle_collisions(&mut self, rl: &RaylibHandle, world_map: &WorldMap) {
        let bullet_y_line = Line {
            start: self.pos,
            end: self.pos + self.vel * Vector2::new(0.0, 1.0) * rl.get_frame_time(),
        };
        let bullet_line = Line {
            start: self.pos,
            end: self.pos + self.vel * rl.get_frame_time(),
        };
        self.dbg_lines = vec![
            Line {
                start: self.pos,
                end: self.pos + self.vel * Vector2::new(3.0, 0.0) * rl.get_frame_time(),
            },
            Line {
                start: self.pos,
                end: self.pos + self.vel * Vector2::new(0.0, 3.0) * rl.get_frame_time(),
            },
        ];
        let mut normals = vec![];
        for (y, line) in world_map.walls.iter().enumerate() {
            for (x, wall) in line.iter().enumerate() {
                if let Some(wall) = wall {
                    for rect in wall
                        .get_collider()
                        .with_pos(Vector2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE))
                        .rects
                    {
                        let lines = Line::from_rect(&rect);
                        for line in lines {
                            // Check for collision
                            if let Some(intersection) = line.intersection(&bullet_line) {
                                normals.push((
                                    intersection,
                                    if line.intersection(&bullet_y_line).is_some() {
                                        Vector2::new(0.6, -0.6)
                                    } else {
                                        Vector2::new(-0.6, 0.6)
                                    },
                                    line,
                                ));
                            }
                        }
                    }
                }
            }
        }
        let mut closest = 0usize;
        for (i, normal) in normals.iter().enumerate() {
            let dist = normal.0.length();
            if let Some(best_normal) = normals.get(closest) {
                if best_normal.0.length() > dist {
                    closest = i;
                }
            }
        }
        if let Some(normal) = normals.get(closest) {
            self.collided = true;
            self.vel *= normal.1;
            self.pos = normal.0 + self.vel.normalized() * 0.01;
            self.dbg_line_hit = Some(normal.2.clone());
            println!("Normal {:?}, Pos {:?}", normal.0, self.pos);
        }
    }

    pub fn get_collider(&self) -> Collider {
        Collider {
            rects: vec![Rectangle {
                x: self.pos.x,
                y: self.pos.y,
                width: 0.5,
                height: 0.5,
            }],
            circles: vec![],
        }
    }
}

pub struct World {
    pub map: WorldMap,
    pub day_cycle: DayCycle,
    pub bullets: Vec<Bullet>,
}

impl World {
    pub fn new(light_engine: &mut LightEngine) -> World {
        Self {
            map: WorldMap::load_from_file("assets/maps/map0.tmx", 30, 20),
            day_cycle: DayCycle::new(light_engine),
            bullets: vec![],
        }
    }

    pub fn spawn_bullet(&mut self, rl: &RaylibHandle, camera: &Camera2D, player: &Player) {
        let player_screen_pos = camera.to_screen(player.pos);
        let mouse_pos = rl.get_mouse_position();
        let angle_to_mouse =
            (mouse_pos.y - player_screen_pos.y).atan2(mouse_pos.x - player_screen_pos.x);
        let bullet_speed = 200.0;
        let bullet_vel = Vector2::new(angle_to_mouse.cos(), angle_to_mouse.sin());
        let bullet = Bullet::new(player.pos + bullet_vel * 15.0, bullet_vel * bullet_speed);
        if self
            .map
            .collides_with_wall(&bullet.get_collider())
            .is_none()
        {
            self.bullets.push(bullet);
        }
    }

    pub fn update_bullets(&mut self, rl: &RaylibHandle) {
        // Update bullets
        for bullet in self.bullets.iter_mut() {
            bullet.update(rl, &self.map);
        }
        // Filter bullets that are stopped or are in a wall
        self.bullets.retain(|bullet| bullet.vel != Vector2::zero());
    }
}
