use crate::{bullet::Bullet, world::World, ImprovedCamera, LightHandle};
use rand::Rng;
use raylib::prelude::*;

#[derive(Debug)]
pub struct GunBody {
    pub mag: Magazine,
    pub fire_mode: FireMode,
    pub chamber: Chamber,
    pub fire_rate: f32,
    pub time_since_shot: f32,
}

impl GunBody {
    pub fn fire_bullet(&mut self) {
        self.mag.bullets -= 1;
        self.time_since_shot = 0.0;
    }
}

#[derive(Debug)]
pub enum GunItem {
    AR15 { body: GunBody },
    Pistol { body: GunBody},
}

#[derive(Debug)]
pub enum FireMode {
    SemiAutomatic,
    Automatic,
}

#[derive(Debug)]
pub struct Chamber {
    pub bullet: bool,
}

#[derive(Debug)]
pub struct Magazine {
    pub bullets: u32,
    pub max_bullets: u32,
}

impl GunItem {
    pub const DEFAULT_AR15: GunItem = GunItem::AR15 {
        body: GunBody {
            mag: Magazine {
                bullets: 30,
                max_bullets: 30,
            },
            fire_mode: FireMode::Automatic,
            chamber: Chamber { bullet: true },
            fire_rate: 1.0 / 12.0,
            time_since_shot: 0.0,
        },
    };

    pub const DEFAULT_PISTOL: GunItem = GunItem::Pistol {
        body: GunBody {
            mag: Magazine {
                bullets: 7,
                max_bullets: 7,
            },
            fire_mode: FireMode::SemiAutomatic,
            chamber: Chamber {
                bullet: true,
            },
            fire_rate: 1.0 / 12.0,
            time_since_shot: 0.0,
        }
    };

    pub fn get_accuarcy(&self) -> f32 {
        match self {
            GunItem::AR15 { .. } => 120.0,
            GunItem::Pistol { .. } => 95.0,
        }
    }

    pub fn get_gun_body(&mut self) -> &mut GunBody {
        match self {
            GunItem::AR15 { body } => body,
            GunItem::Pistol { body } => body,
        }
    }
}

pub struct FlashLight {
    pub light_handle: LightHandle,
    pub active: bool,
}

pub fn explode(rl: &RaylibHandle, world: &mut World, camera: &Camera2D) {
    let num_shrapnel = 25;
    let num_random_shrapnel = 25;
    let shrapnel_speed = 500.0;
    let shrapnel_speed_margin = 0.3;
    let mut rng = rand::thread_rng();
    let mouse_world_pos = camera.to_world(rl.get_mouse_position());
    for i in 0..num_shrapnel {
        let angle = 2.0 * PI as f32 * (i as f32 / num_shrapnel as f32);
        let vel = Vector2::new(angle.cos(), angle.sin()) * shrapnel_speed;
        let random_vel = rng.gen_range(1.0 - shrapnel_speed_margin..1.0 + shrapnel_speed_margin);
        world
            .bullets
            .push(Bullet::new(mouse_world_pos, vel * random_vel));
    }
    for _ in 0..num_random_shrapnel {
        let angle = rng.gen_range(0.0..2.0 * PI as f32);
        let random_vel = rng.gen_range(1.0 - shrapnel_speed_margin..1.0 + shrapnel_speed_margin);
        let vel = Vector2::new(angle.cos(), angle.sin()) * shrapnel_speed;
        world
            .bullets
            .push(Bullet::new(mouse_world_pos, vel * random_vel));
    }
}
