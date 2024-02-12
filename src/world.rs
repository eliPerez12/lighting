use raylib::prelude::*;

use crate::{Light, LightEngine, LightHandle};

pub struct DayCycle {
    time: f32,
    ambient_light_handle: LightHandle,
}

impl DayCycle {
    pub const FULL_CYCLE_LENGTH: f32 = 300.0;
    pub fn new(light_engine: &mut LightEngine) -> DayCycle {
        DayCycle {
            time: 0.25 * DayCycle::FULL_CYCLE_LENGTH,
            ambient_light_handle: light_engine.spawn_light(Light::default_ambient()),
        }
    }
    pub fn update(&mut self, rl: &mut RaylibHandle) {
        self.time += rl.get_frame_time();
        if self.time > Self::FULL_CYCLE_LENGTH {
            self.time -= Self::FULL_CYCLE_LENGTH;
        }
    }
    pub fn ambient_light_handle(&self) -> &LightHandle {
        &self.ambient_light_handle
    }
    pub fn get_ambient_light(&self) -> Light {
        let normilized_time = self.time / Self::FULL_CYCLE_LENGTH;
        let sunrise_length = 1.0 / 8.0;

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

pub trait ImprovedCamera {
    fn to_screen(&self, world_pos: Vector2) -> Vector2;
    fn to_world(&self, screen_pos: Vector2) -> Vector2;
}

impl ImprovedCamera for Camera2D {
    fn to_screen(&self, world_pos: Vector2) -> Vector2 {
        (world_pos + self.offset) * self.zoom
    }
    fn to_world(&self, screen_pos: Vector2) -> Vector2 {
        (screen_pos / self.zoom) - self.offset
    }
}
