use crate::LightHandle;

pub struct Gun {
    pub mag: Magazine,
    pub accuracy: f32,
    pub time_since_shot: f32, // in milliseconds
}

pub struct Magazine {
    pub bullets: u32,
}

pub struct FlashLight {
    pub light_handle: LightHandle,
    pub active: bool,
}
