use crate::{Collider, ImprovedCamera, Light, LightEngine, LightHandle, WorldMap};
use raylib::prelude::*;

pub struct Player {
    pub pos: Vector2,
    pub vel: Vector2,
    pub ambient_light: LightHandle,
    animation: PlayerAnimation,
    flashlight: FlashLight,
    pub muzzle_lights: [LightHandle; 3],
}

struct FlashLight {
    light_handle: LightHandle,
    active: bool,
}

impl Player {
    pub const RENDER_SIZE: Vector2 = Vector2::new(26.0, 42.0);
    pub const COLLIDER_SIZE: f32 = 13.0;
    pub const MUZZLE_FLASH_COLOR: Color = Color::new(255, 87, 51, 255);
    pub fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        light_engine: &mut LightEngine,
    ) -> Player {
        Player {
            pos: Vector2::zero(),
            vel: Vector2::zero(),
            animation: PlayerAnimation::new(rl, thread),
            flashlight: FlashLight {
                light_handle: light_engine.spawn_light(Light::default_cone()),
                active: true,
            },
            ambient_light: light_engine.spawn_light(Light::Radial {
                pos: Vector2::zero(),
                color: Vector4::new(1.0, 1.0, 1.0, 0.35),
                radius: 155.0,
            }),
            muzzle_lights: [
                light_engine.spawn_light(Light::Radial {
                    pos: Vector2::zero(),
                    color: Self::MUZZLE_FLASH_COLOR.into(),
                    radius: 100.0,
                }),
                light_engine.spawn_light(Light::Radial {
                    pos: Vector2::zero(),
                    color: Self::MUZZLE_FLASH_COLOR.into(),
                    radius: 25.0,
                }),
                light_engine.spawn_light(Light::Radial {
                    pos: Vector2::zero(),
                    color: Self::MUZZLE_FLASH_COLOR.into(),
                    radius: 25.0,
                }),
            ],
        }
    }

    pub fn get_world_collider(&self) -> Collider {
        Collider {
            rects: vec![Rectangle {
                x: self.pos.x - Self::COLLIDER_SIZE / 2.0,
                y: self.pos.y - Self::COLLIDER_SIZE / 2.0,
                width: Self::COLLIDER_SIZE,
                height: Self::COLLIDER_SIZE,
            }],
            circles: vec![],
        }
    }

    pub fn get_animation_frame(&self) -> &Texture2D {
        &self.animation.frames[self.animation.current_frame]
    }

    pub fn get_angle_to_screen_pos(&self, screen_pos: Vector2, camera: &Camera2D) -> f32 {
        let player_screen_pos = camera.to_screen(self.pos);
        (screen_pos.y - player_screen_pos.y).atan2(screen_pos.x - player_screen_pos.x)
    }

    pub fn get_vector_to_screen_pos(&self, screen_pos: Vector2, camera: &Camera2D) -> Vector2 {
        let angle_to_pos = self.get_angle_to_screen_pos(screen_pos, camera);
        Vector2::new(angle_to_pos.cos(), angle_to_pos.sin())
    }

    pub fn handle_controls(&mut self, rl: &RaylibHandle, world_map: &WorldMap) {
        self.handle_movement_controls(rl);
        self.handle_flashlight_controls(rl);
        self.handle_collisions(world_map);
        self.apply_velocity();
        self.animation.handle_animation(rl);
    }

    fn handle_flashlight_controls(&mut self, rl: &RaylibHandle) {
        // Player flashlight control
        if rl.is_key_pressed(KeyboardKey::KEY_F) {
            self.flashlight.active = !self.flashlight.active;
        }
    }

    fn apply_velocity(&mut self) {
        self.pos += self.vel
    }

    fn handle_movement_controls(&mut self, rl: &RaylibHandle) {
        // Constants that are ajusted with frame time to be consistant across fps
        let player_speed = match rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT)
            | rl.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT)
        {
            true => 50.0 * rl.get_frame_time(),
            false => 35.0 * rl.get_frame_time(),
        };
        let player_acc = 7.0 * rl.get_frame_time();
        let player_deacc = 4.0 * rl.get_frame_time();

        // Player y controls
        if rl.is_key_down(KeyboardKey::KEY_W) {
            self.vel.y = (self.vel.y - player_acc).max(-player_speed);
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            self.vel.y = (self.vel.y + player_acc).min(player_speed);
        }

        // Deaccelerating in the y
        if !rl.is_key_down(KeyboardKey::KEY_S)
            && !rl.is_key_down(KeyboardKey::KEY_W)
            && self.vel.y != 0.0
        {
            if self.vel.y > 0.0 {
                self.vel.y = (self.vel.y - player_deacc).max(0.0);
            } else {
                self.vel.y = (self.vel.y + player_deacc).min(0.0);
            }
        }

        // Player x controls
        if rl.is_key_down(KeyboardKey::KEY_A) {
            self.vel.x = (self.vel.x - player_acc).max(-player_speed);
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            self.vel.x = (self.vel.x + player_acc).min(player_speed);
        }

        // Deaccelerating in the x
        if !rl.is_key_down(KeyboardKey::KEY_A)
            && !rl.is_key_down(KeyboardKey::KEY_D)
            && self.vel.x != 0.0
        {
            if self.vel.x > 0.0 {
                self.vel.x = (self.vel.x - player_deacc).max(0.0);
            } else {
                self.vel.x = (self.vel.x + player_deacc).min(0.0);
            }
        }
    }

    // Prevents player from clipping through colliders
    pub fn handle_collisions(&mut self, world_map: &WorldMap) {
        let player_collider = self.get_world_collider();

        // Iterate over every wall, and every collider rect in each wall collider
        for (y, wall_line) in world_map.walls.iter().enumerate() {
            for (x, wall) in wall_line.iter().enumerate() {
                if let Some(wall) = wall {
                    for collider_rect in wall
                        .get_collider()
                        .with_pos(Vector2::new(x as f32 * 32.0, y as f32 * 32.0))
                        .rects
                        .iter()
                    {
                        // Checks if player will collide with wall in y axis
                        if let Some(_collision_rect) = collider_rect.get_collision_rec(
                            &player_collider
                                .with_pos(Vector2::new(0.0, self.vel.y))
                                .rects[0],
                        ) {
                            // Move player to edge of wall and set vel y to 0
                            self.vel.y = 0.0;
                            if self.pos.y < collider_rect.y + collider_rect.height / 2.0 {
                                self.pos.y =
                                    collider_rect.y - player_collider.rects[0].height / 2.0;
                            } else {
                                self.pos.y = collider_rect.y
                                    + collider_rect.height
                                    + player_collider.rects[0].height / 2.0;
                            }
                        }

                        // Checks if player will collide with wall in x axis
                        if let Some(_collision_rect) = collider_rect.get_collision_rec(
                            &player_collider
                                .with_pos(Vector2::new(self.vel.x, 0.0))
                                .rects[0],
                        ) {
                            // Move player to edge of wall and set vel x to 0
                            self.vel.x = 0.0;
                            if self.pos.x < collider_rect.x + collider_rect.width / 2.0 {
                                self.pos.x = collider_rect.x - player_collider.rects[0].width / 2.0;
                            } else {
                                self.pos.x = collider_rect.x
                                    + collider_rect.width
                                    + player_collider.rects[0].width / 2.0;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn update_flashlight(
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
