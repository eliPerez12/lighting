#![allow(dead_code)]
use raylib::prelude::*;
use std::collections::HashMap;

pub enum Light {
    Radial {
        pos: Vector2,
        color: Vector4,
        radius: f32,
    },
    Ambient {
        color: Vector4,
    },
}

impl Light {
    pub fn default_radial() -> Light {
        Light::Radial {
            pos: Vector2::new(0.0, 0.0),
            color: Vector4::new(1.0, 1.0, 1.0, 1.0),
            radius: 350.0,
        }
    }
    pub fn color(&self) -> Vector4 {
        match self {
            Light::Radial { color, .. } => *color,
            Light::Ambient { color } => *color,
        }
    }
    pub fn pos(&self) -> Vector2 {
        match self {
            Light::Radial { pos, .. } => *pos,
            Light::Ambient { .. } => Vector2::zero(),
        }
    }
    pub fn radius(&self) -> f32 {
        match self {
            Light::Radial { radius, .. } => *radius,
            Light::Ambient { .. } => 0.0,
        }
    }
    pub fn light_type(&self) -> i32 {
        match self {
            Light::Radial { .. } => 0,
            Light::Ambient { .. } => 1,
        }
    }
}

struct ShaderUniforms {
    position: i32,
    color: i32,
    amount: i32,
    radius: i32,
    light_type: i32,
}

pub struct LightEngine {
    lights: HashMap<u32, Light>,
    light_id: u32,
    shader_uniforms: ShaderUniforms,
}

pub struct LightHandle(u32);

impl LightEngine {
    pub fn new(shader: &mut Shader) -> LightEngine {
        LightEngine {
            lights: HashMap::new(),
            light_id: 0,
            shader_uniforms: ShaderUniforms {
                position: shader.get_shader_location("lightsPosition"),
                color: shader.get_shader_location("lightsColor"),
                amount: shader.get_shader_location("lightsAmount"),
                radius: shader.get_shader_location("lightsRadius"),
                light_type: shader.get_shader_location("lightsType"),
            },
        }
    }
    pub fn spawn_light(&mut self, light: Light) -> LightHandle {
        self.lights.insert(self.light_id, light);
        self.light_id += 1;
        LightHandle(self.light_id - 1)
    }
    pub fn update_light(&mut self, light_handle: &LightHandle, updated_light: Light) {
        self.lights.insert(light_handle.0, updated_light);
    }
    pub fn get_mut_light(&mut self, light_handle: &LightHandle) -> &mut Light {
        self.lights.get_mut(&light_handle.0).unwrap()
    } 
    pub fn update_shader_values(&self, shader: &mut Shader) {
        shader.set_shader_value_v(
            self.shader_uniforms.position,
            self.lights
                .iter()
                .map(|light| light.1.pos())
                .collect::<Vec<Vector2>>()
                .as_slice(),
        );
        shader.set_shader_value_v(
            self.shader_uniforms.color,
            self.lights
                .iter()
                .map(|light| light.1.color())
                .collect::<Vec<Vector4>>()
                .as_slice(),
        );
        shader.set_shader_value(self.shader_uniforms.amount, self.lights.len() as i32);
        shader.set_shader_value_v(
            self.shader_uniforms.radius,
            self.lights
                .iter()
                .map(|light| light.1.radius())
                .collect::<Vec<f32>>()
                .as_slice(),
        );
        shader.set_shader_value_v(
            self.shader_uniforms.light_type,
            self.lights
                .iter()
                .map(|light| light.1.light_type())
                .collect::<Vec<i32>>()
                .as_slice(),
        );
    }
}
