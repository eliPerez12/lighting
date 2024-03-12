use crate::{bullet::Bullet, world::World, ImprovedCamera, LightHandle};
use rand::Rng;
use raylib::prelude::*;

pub struct Gun {
    pub mag: Magazine,
    pub accuracy: f32,
    pub time_since_shot: f32, // in milliseconds
}

pub struct Magazine {
    pub bullets: u32,
    pub max_bullets: u32,
}

pub struct FlashLight {
    pub light_handle: LightHandle,
    pub active: bool,
}

impl Gun {
    pub fn new_assult_rifle() -> Self {
        Gun {
            mag: Magazine {
                bullets: 30,
                max_bullets: 9999,
            },
            accuracy: 120.0,
            time_since_shot: 0.0,
        }
    }
}

pub fn explode(rl: &RaylibHandle, world: &mut World, camera: &Camera2D) {
    let num_shrapnel = 25;
    let num_random_shrapnel = 15;
    let shrapnel_speed = 500.0;
    let shrapnel_speed_margin = 0.3;
    for i in 0..num_shrapnel {
        let pos = camera.to_world(rl.get_mouse_position());
        let angle = 2.0 * PI as f32 * (i as f32 / num_shrapnel as f32);
        let vel = Vector2::new(angle.cos(), angle.sin()) * shrapnel_speed;
        let random_vel =
            rand::thread_rng().gen_range(1.0 - shrapnel_speed_margin..1.0 + shrapnel_speed_margin);
        world.bullets.push(Bullet::new(pos, vel * random_vel));
    }
    for _ in 0..num_random_shrapnel {
        let pos = camera.to_world(rl.get_mouse_position());
        let angle = rand::thread_rng().gen_range(0.0..2.0 * PI as f32);
        let random_vel =
            rand::thread_rng().gen_range(1.0 - shrapnel_speed_margin..1.0 + shrapnel_speed_margin);
        let vel = Vector2::new(angle.cos(), angle.sin()) * shrapnel_speed;
        world.bullets.push(Bullet::new(pos, vel * random_vel));
    }
}
