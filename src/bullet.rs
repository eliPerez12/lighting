use rand::Rng;
use raylib::prelude::*;

use crate::{Collider, Line, WorldMap, TILE_SIZE};

pub struct Bullet {
    pub pos_history: [Vector2; 3],
    pub pos: Vector2,
    pub vel: Vector2,
    pub collided: Option<Vector2>,
    pub dbg_lines: Vec<Line>,
    pub dbg_line_hit: Option<Line>,
}

impl Bullet {
    pub fn new(pos: Vector2, vel: Vector2) -> Bullet {
        Bullet {
            pos,
            vel,
            pos_history: [pos; 3],
            dbg_lines: vec![],
            dbg_line_hit: None,
            collided: None,
        }
    }

    pub fn update_history(&mut self) {
        self.pos_history[2] = self.pos_history[1];
        self.pos_history[1] = self.pos_history[0];
        self.pos_history[0] = self.pos;
    }

    pub fn update(&mut self, rl: &RaylibHandle, world_map: &WorldMap) {
        self.collided = None;
        self.dbg_line_hit = None;
        self.update_history();
        let drag = 12.0;

        self.vel -= self.vel.normalized() * drag * rl.get_frame_time() * 60.0;
        if self.vel.length() <= 90.0 {
            self.vel = Vector2::zero();
        }
        self.handle_collisions(rl, world_map);
        if self.collided.is_none() {
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
                                let velocity_lost = rand::thread_rng().gen_range(0.0..=0.8);
                                normals.push((
                                    intersection,
                                    if line.intersection(&bullet_y_line).is_some() {
                                        Vector2::new(velocity_lost, -velocity_lost)
                                    } else {
                                        Vector2::new(-velocity_lost, velocity_lost)
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
            self.collided = Some(normal.1);
            self.vel *= normal.1;
            self.pos = normal.0 + self.vel.normalized() * 0.01;
            self.dbg_line_hit = Some(normal.2.clone());
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
