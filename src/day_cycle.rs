use crate::{Light, LightEngine, LightHandle};
use raylib::prelude::*;

pub struct DayCycle {
    pub time: f32,
    ambient_light_handle: LightHandle,
}

impl DayCycle {
    pub const FULL_CYCLE_LENGTH: f32 = 100.0;
    pub fn new(light_engine: &mut LightEngine) -> DayCycle {
        DayCycle {
            time: 0.25 * DayCycle::FULL_CYCLE_LENGTH,
            ambient_light_handle: light_engine.spawn_light(Light::default_ambient()).unwrap(),
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
        else if normilized_time >= 0.5 {
            Light::Ambient {
                color: Color::BLACK.into(),
            }
        } else {
            Light::Ambient {
                color: Color::PURPLE.into(),
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
