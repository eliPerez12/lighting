use crate::{Light, LightEngine, LightHandle};
use raylib::prelude::*;

pub struct DayCycle {
    pub time: f32,
    ambient_light_handle: LightHandle,
}

impl DayCycle {
    pub const FULL_CYCLE_LENGTH: f32 = 40.0;
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
        let v_normilized_time =
            Vector4::new(normilized_time, normilized_time, normilized_time, 1.0);
        const NOON: f32 = 0.25;
        const SUNRISE_LENGTH: f32 = 0.05;
        const SUNSET_LENGTH: f32 = 0.1;
        const SUNSET: f32 = 0.5;
        const TO_NOON_PHASE_LENGTH: f32 = 0.1;
        const DAY_COLOR: Vector4 = Vector4::new(1.0, 1.0, 1.0, 1.0);
        const SUNRISE_COLOR: Vector4 = Vector4::new(0.50, 0.60, 0.80, 1.00);
        const SUNSET_COLOR: Vector4 = Vector4::new(0.86, 0.52, 0.4, 1.00);
        const NIGHT_COLOR: Vector4 = Vector4::new(0.0, 0.05, 0.1, 1.0);

        let final_color = {
            // Sun rising from ntime 1.0 - sunrise length to 1.0
            if ((1.0 - SUNRISE_LENGTH)..=1.0).contains(&normilized_time) {
                // Differance in color from night to halfway to full sunrise
                let diff_color = sub_vector4(mul_f_vector4(SUNRISE_COLOR, 0.5), NIGHT_COLOR);
                // how far along to halfway to sunrise length
                let step = (normilized_time -(1.0-SUNRISE_LENGTH)) / SUNRISE_LENGTH;
                add_vector4(NIGHT_COLOR, mul_f_vector4(diff_color, step))
            }
            // Sun rising from ntime 0.0 to sunrise length
            else if (0.0..=SUNRISE_LENGTH).contains(&normilized_time) {
                mul_vector4(
                    SUNRISE_COLOR,
                    add_f_vector4(
                        mul_f_vector4(v_normilized_time, 0.5/ SUNRISE_LENGTH),
                        0.5,
                    ),
                )
            }
            // Sun turning to day color after sunrise
            else if (SUNRISE_LENGTH..=NOON - TO_NOON_PHASE_LENGTH).contains(&normilized_time) {
                // Differance in color from sunrise to sunset
                let diff_color = sub_vector4(DAY_COLOR, SUNRISE_COLOR);
                // How far along the change phase
                let step =
                    (normilized_time - SUNRISE_LENGTH) / (NOON - TO_NOON_PHASE_LENGTH - SUNRISE_LENGTH);
                add_vector4(SUNRISE_COLOR, mul_f_vector4(diff_color, step))
            }
            // Sun turning to sunset after daytime
            else if (SUNSET - SUNSET_LENGTH..=SUNSET).contains(&normilized_time) {
                let diff_color = sub_vector4(DAY_COLOR, SUNSET_COLOR);
                // how far along to halfway to sunrset length
                let step = (normilized_time - SUNSET + SUNSET_LENGTH) / SUNSET_LENGTH;
                println!("Step {step}, diff_color: {:?}", diff_color);
                sub_vector4(DAY_COLOR, mul_f_vector4(diff_color, step))
            }
            // Sun turing to night after sunset
            else if (SUNSET..=SUNSET + SUNSET_LENGTH).contains(&normilized_time) {
                // Differance in color from sunset to night
                let diff_color = sub_vector4(SUNSET_COLOR, NIGHT_COLOR);
                // How far along the change phase
                let step =
                    (normilized_time - SUNSET) / (SUNSET_LENGTH);
                println!("Step {step}, diff_color: {:?}", diff_color);
                sub_vector4(SUNSET_COLOR, mul_f_vector4(diff_color, step))
            }
            else if (SUNRISE_LENGTH..(0.5 - SUNRISE_LENGTH)).contains(&normilized_time) {
                DAY_COLOR
            } else {
                NIGHT_COLOR
            }
        };
        Light::Ambient { color: final_color }
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

fn mul_vector4(vector: Vector4, other_vector: Vector4) -> Vector4 {
    Vector4::new(
        vector.x * other_vector.x,
        vector.y * other_vector.y,
        vector.z * other_vector.z,
        vector.w,
    )
}

fn mul_f_vector4(vector: Vector4, float: f32) -> Vector4 {
    Vector4::new(
        vector.x * float,
        vector.y * float,
        vector.z * float,
        vector.w,
    )
}

fn add_f_vector4(vector: Vector4, float: f32) -> Vector4 {
    Vector4::new(
        vector.x + float,
        vector.y + float,
        vector.z + float,
        vector.w,
    )
}

fn add_vector4(vector: Vector4, other_vector: Vector4) -> Vector4 {
    Vector4::new(
        vector.x + other_vector.x,
        vector.y + other_vector.y,
        vector.z + other_vector.z,
        vector.w,
    )
}

fn sub_vector4(vector: Vector4, other_vector: Vector4) -> Vector4 {
    Vector4::new(
        vector.x - other_vector.x,
        vector.y - other_vector.y,
        vector.z - other_vector.z,
        vector.w,
    )
}

