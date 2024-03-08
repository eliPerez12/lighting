use crate::LightHandle;

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
                max_bullets: 30,
            },
            accuracy: 120.0,
            time_since_shot: 0.0,
        }
    }
}
