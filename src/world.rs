use crate::{Collider, DayCycle, ImprovedCamera, LightEngine, Player, WorldMap};
use raylib::prelude::*;

pub struct Bullet {
    pub pos_history: [Vector2; 3],
    pub pos: Vector2,
    pub vel: Vector2,
}

impl Bullet {
    pub fn new(pos: Vector2, vel: Vector2) -> Bullet {
        Bullet {
            pos,
            vel,
            pos_history: [pos; 3],
        }
    }

    pub fn update_history(&mut self) {
        self.pos_history[2] = self.pos_history[1];
        self.pos_history[1] = self.pos_history[0];
        self.pos_history[0] = self.pos;
    }

    pub fn update(&mut self, rl: &RaylibHandle, world_map: &WorldMap) {
        self.update_history();
        let drag = 35.0;

        self.vel -= self.vel / drag * rl.get_frame_time() * 60.0;
        if self.vel.length() <= 30.0 {
            self.vel = Vector2::zero();
        }

        self.pos += self.vel * rl.get_frame_time();

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
        if rl.is_key_pressed(KeyboardKey::KEY_G) {
            let player_screen_pos = camera.to_screen(player.pos);
            let mouse_pos = rl.get_mouse_position();
            let angle_to_mouse =
                (mouse_pos.y - player_screen_pos.y).atan2(mouse_pos.x - player_screen_pos.x);
            let bullet_speed = 1000.0;
            let bullet_vel = Vector2::new(angle_to_mouse.cos(), angle_to_mouse.sin());

            self.bullets.push(Bullet::new(
                player.pos + bullet_vel * 15.0,
                bullet_vel * bullet_speed,
            ));
        }
    }

    pub fn update_bullets(&mut self, rl: &RaylibHandle) {
        // Update bullets
        for bullet in self.bullets.iter_mut() {
            bullet.update(rl, &self.map);
        }
        // Filter bullets that are stopped or are colldiding with a wall
        self.bullets.retain(|bullet| bullet.vel != Vector2::zero());

        // self.bullets.retain(|bullet| {
        //     self.map
        //         .collides_with_wall(&bullet.get_collider())
        //         .is_none()
        // });
    }
}
