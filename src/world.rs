use crate::{bullet::Bullet, day_cycle::DayCycle, ImprovedCamera, LightEngine, Player, WorldMap};
use rand::Rng;
use raylib::prelude::*;

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
        let accuracy = PI as f32 / player.gun.accuracy;
        let angle_to_mouse = (mouse_pos.y - player_screen_pos.y)
            .atan2(mouse_pos.x - player_screen_pos.x)
            + rand::thread_rng().gen_range(-accuracy..accuracy); // Add shake to shooting
        let bullet_speed = 1000.0;
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
