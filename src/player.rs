use crate::{
    Collider, ImprovedCamera, Light, LightEngine, LightHandle, WorldMap,
};
use raylib::prelude::*;

pub struct Player {
    pub pos: Vector2,
    animation: PlayerAnimation,
    flashlight: FlashLight,
}

struct FlashLight {
    light_handle: LightHandle,
    active: bool,
}

impl Player {
    pub const RENDER_SIZE: Vector2 = Vector2::new(26.0, 42.0);
    pub fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        light_engine: &mut LightEngine,
    ) -> Player {
        Player {
            pos: Vector2::zero(),
            animation: PlayerAnimation::new(rl, thread),
            flashlight: FlashLight {
                light_handle: light_engine.spawn_light(Light::default_cone()),
                active: true,
            },
            
        }
    }

    pub fn get_world_collider(&self) -> Collider {
        Collider {
            rects: vec![Rectangle {
                x: self.pos.x - 6.0,
                y: self.pos.y - 6.0,
                width: 12.0,
                height: 12.0,
            }],
            circles: vec![],
        }
    }

    pub fn get_animation_frame(&self) -> &Texture2D {
        &self.animation.frames[self.animation.current_frame]
    }

    pub fn handle_movement(&mut self, rl: &RaylibHandle, world_map: &WorldMap) {
        let player_speed = 40.0 * rl.get_frame_time();
        if rl.is_key_down(KeyboardKey::KEY_W) {
            self.pos.y -= player_speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            self.pos.y += player_speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            self.pos.x -= player_speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            self.pos.x += player_speed;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_F) {
            self.flashlight.active = !self.flashlight.active;
        }
        
        self.handle_collisions(world_map);
        self.animation.handle_animation(rl);
    }

    pub fn handle_collisions(&mut self, world_map: &WorldMap) {
        for (y, wall_line) in world_map.walls.iter().enumerate() {
            for (x, wall) in wall_line.iter().enumerate() {
                if let Some(wall) = wall {
                    if wall
                        .get_collider()
                        .with_pos(Vector2::new(x as f32 * 32.0, y as f32 * 32.0))
                        .collides(&self.get_world_collider())
                        .is_some()
                    {
                    }
                }
            }
        }
    }

    pub fn handle_flashlight(
        &mut self,
        rl: &mut RaylibHandle,
        camera: &Camera2D,
        light_engine: &mut LightEngine,
    ) {
        let player_screen_pos = camera.to_screen(self.pos);
        let mouse_pos = rl.get_mouse_position();
        let dx = mouse_pos.x - player_screen_pos.x;
        let dy = -(mouse_pos.y - player_screen_pos.y);
        let rotation = dy.atan2(dx) + PI as f32;

        light_engine.update_light(
            &self.flashlight.light_handle,
            Light::Cone {
                pos: self.pos + Vector2::new(dx, -dy).normalized() * 5.0,
                color: if self.flashlight.active {
                    Color::WHEAT.into()
                } else {
                    Color::BLACK.into()
                },
                radius: 250.0,
                angle: PI as f32 / 2.0,
                rotation,
            },
        );
    }
}

struct PlayerAnimation {
    current_frame: usize,
    elapsed_time: f32,
    frames: Vec<Texture2D>,
}

impl PlayerAnimation {
    const FRAME_AMOUNT: usize = 4;
    const FRAME_TIME: f32 = 0.015;

    fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> PlayerAnimation {
        PlayerAnimation {
            frames: (1..=5)
                .map(|i| {
                    rl.load_texture(thread, &format!("assets/player/player_body{i}.png"))
                        .unwrap()
                })
                .collect::<Vec<Texture2D>>(),
            current_frame: 0,
            elapsed_time: 0.0,
        }
    }

    pub fn handle_animation(&mut self, rl: &RaylibHandle) {
        // If player is trying to ADS
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT)
            || rl.is_key_down(KeyboardKey::KEY_SPACE)
        {
            if self.elapsed_time > Self::FRAME_TIME || self.current_frame == 0 {
                if self.current_frame < Self::FRAME_AMOUNT {
                    self.current_frame += 1;
                }
                self.elapsed_time = 0.0;
            }
            self.elapsed_time += rl.get_frame_time();
        } else if self.current_frame > 0 {
            if self.elapsed_time <= -Self::FRAME_TIME || self.current_frame == Self::FRAME_AMOUNT {
                self.current_frame -= 1;
                self.elapsed_time = 0.0;
            }
            self.elapsed_time -= rl.get_frame_time();
        }
    }
}
