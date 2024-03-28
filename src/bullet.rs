use rand::Rng;
use raylib::prelude::*;

use crate::{Collider, Line, WorldMap, TILE_SIZE};

pub struct Bullet {
    pub pos_history: [Vector2; 3],
    pub pos: Vector2,
    pub vel: Vector2,
    pub collided: Option<Vector2>,
    pub dbg_line_hit: Option<Line>,
    pub drag: f32,
}

impl Bullet {
    pub fn new(pos: Vector2, vel: Vector2) -> Bullet {
        Bullet {
            pos,
            vel,
            pos_history: [pos; 3],
            dbg_line_hit: None,
            collided: None,
            drag: 12.0,
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

        self.vel -= self.vel.normalized() * self.drag * rl.get_frame_time() * 60.0;
        if self.vel.length() <= 20.0 {
            self.vel = Vector2::zero();
        }
        self.handle_collisions(rl, world_map);
        if self.collided.is_none() {
            self.pos += self.vel * rl.get_frame_time();
        }
    }

    pub fn handle_collisions(&mut self, rl: &RaylibHandle, world_map: &WorldMap) {
        let frame_time = rl.get_frame_time();
        let min_velocity_lost = 0.3;

        // Calculate bullet lines
        let bullet_y_line = Line {
            start: self.pos,
            end: self.pos + self.vel * Vector2::new(0.0, 1.0) * frame_time,
        };
        let bullet_line = Line {
            start: self.pos,
            end: self.pos + self.vel * frame_time,
        };

        // Get all normals from every collision
        let mut normals = Vec::new();
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
                                let velocity_lost =
                                    rand::thread_rng().gen_range(0.0..=min_velocity_lost);
                                let normal = if line.intersection(&bullet_y_line).is_some() {
                                    Vector2::new(velocity_lost, -velocity_lost)
                                } else {
                                    Vector2::new(-velocity_lost, velocity_lost)
                                };
                                normals.push((intersection, normal, line));
                            }
                        }
                    }
                }
            }
        }

        // Find closest normal
        if let Some((closest_index, _)) =
            normals
                .iter()
                .enumerate()
                .min_by(|(_, normal1), (_, normal2)| {
                    normal1.0.length().partial_cmp(&normal2.0.length()).unwrap()
                })
        {
            let (intersection, normal, line) = &normals[closest_index];
            self.collided = Some(*normal);
            self.vel *= *normal;
            self.pos = *intersection + self.vel.normalized() * -(f32::EPSILON - 1.0);
            self.dbg_line_hit = Some(line.clone());
        }
    }

    pub fn get_collider(&self) -> Collider {
        Collider {
            rects: vec![Rectangle {
                x: self.pos.x,
                y: self.pos.y,
                width: 1.0,
                height: 1.0,
            }],
        }
    }
}
