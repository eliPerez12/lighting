use crate::{
    items::*, world::World, Collider, ImprovedCamera, Light, LightEngine, LightHandle, WorldMap,
};
use raylib::prelude::*;

pub struct Player {
    pub pos: Vector2,
    pub vel: Vector2,
    pub ambient_light: LightHandle,
    animation: PlayerAnimation,
    flashlight: FlashLight,
    pub muzzle_light: LightHandle,
    pub gun: Gun,
    pub is_sprinting: bool,
}

impl Player {
    pub const RENDER_SIZE: Vector2 = Vector2::new(26.0, 42.0);
    pub const COLLIDER_SIZE: f32 = 13.0;
    pub const MUZZLE_FLASH_COLOR: Color = Color::new(255, 87, 51, 255);
    const SPRINT_SPEED: f32 = 60.0;
    const WALK_SPEED: f32 = 30.0;
    const WALK_ACC: f32 = 3.8;
    const WALK_DEACC: f32 = 2.3;

    pub fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        light_engine: &mut LightEngine,
    ) -> Player {
        Player {
            pos: Vector2::zero(),
            vel: Vector2::zero(),
            animation: PlayerAnimation::new(rl, thread),
            is_sprinting: false,
            flashlight: FlashLight {
                light_handle: light_engine.spawn_light(Light::default_cone()).unwrap(),
                active: false,
            },
            gun: Gun::new_assult_rifle(),
            ambient_light: light_engine
                .spawn_light(Light::Radial {
                    pos: Vector2::zero(),
                    color: Vector4::new(1.0, 1.0, 1.0, 0.35),
                    radius: 110.0,
                })
                .unwrap(),
            muzzle_light: light_engine
                .spawn_light(Light::Radial {
                    pos: Vector2::zero(),
                    color: Self::MUZZLE_FLASH_COLOR.into(),
                    radius: 90.0,
                })
                .unwrap(),
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
        world_map.handle_player_collisions(self);
        self.apply_velocity();
        self.animation.handle_animation(rl);
    }

    fn handle_flashlight_controls(&mut self, rl: &RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_F) {
            self.flashlight.active = !self.flashlight.active;
        }
    }

    fn apply_velocity(&mut self) {
        self.pos += self.vel
    }
    fn handle_lighting(
        &mut self,
        light_engine: &mut LightEngine,
        rl: &RaylibHandle,
        camera: &Camera2D,
        player_shooting: bool,
    ) {
        // Ambient light
        light_engine
            .get_mut_light(&self.ambient_light)
            .set_pos(self.pos);

        // If player is trying to shoot
        if player_shooting {
            // Set muzzle light to on and to the end of the players gun
            light_engine
                .get_mut_light(&self.muzzle_light)
                .set_pos(
                    self.pos
                        + self.get_vector_to_screen_pos(rl.get_mouse_position(), camera) * 15.0,
                )
                .set_color(Vector4::new(1.0, 0.73, 0.41, 1.5));
        // Else reduce the brightness of the muzzle light
        } else {
            let light = light_engine.get_mut_light(&self.muzzle_light).set_pos(
                self.pos + self.get_vector_to_screen_pos(rl.get_mouse_position(), camera) * 15.0,
            );
            let old_color = light.color();
            light.set_color(Vector4::new(
                old_color.x,
                old_color.y,
                old_color.w,
                (old_color.z - (25.0 * rl.get_frame_time())).max(0.0),
            ));
        }
    }

    pub fn handle_shooting(
        &mut self,
        light_engine: &mut LightEngine,
        rl: &RaylibHandle,
        world: &mut World,
        camera: &Camera2D,
    ) {
        self.gun.time_since_shot += rl.get_frame_time(); // Update time since shot

        // If player is trying to reload
        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            self.gun.mag.bullets = self.gun.mag.max_bullets;
        };
        // If player is trying to shoot
        let is_shooting = match rl.is_key_down(KeyboardKey::KEY_SPACE) {
            true =>
            // If mag isnt empty
            {
                if self.gun.mag.bullets > 0
                    && self.gun.time_since_shot > 0.1
                    && self.animation.current_frame == PlayerAnimation::FRAME_AMOUNT
                {
                    // Shoot bullet
                    world.spawn_bullet(rl, camera, self);
                    self.gun.mag.bullets -= 1;
                    self.gun.time_since_shot = 0.0;
                    true
                } else {
                    false
                }
            }
            false => false,
        };
        self.handle_lighting(light_engine, rl, camera, is_shooting);
    }

    fn handle_movement_controls(&mut self, rl: &RaylibHandle) {
        // Constants that are adjusted with frame time to be consistent across fps
        let player_speed = match rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
            true => {
                self.is_sprinting = true;
                Self::SPRINT_SPEED * rl.get_frame_time()
            }
            false => {
                self.is_sprinting = false;
                Self::WALK_SPEED * rl.get_frame_time()
            }
        };
        let player_acc = Self::WALK_ACC * rl.get_frame_time();
        let player_deacc = Self::WALK_DEACC * rl.get_frame_time();

        // Calculate direction vectors for movement
        let mut direction = Vector2::new(0.0, 0.0);
        if rl.is_key_down(KeyboardKey::KEY_W) {
            direction.y -= 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            direction.y += 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            direction.x -= 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            direction.x += 1.0;
        }

        // Normalize the direction vector if it's not zero
        if direction.x != 0.0 || direction.y != 0.0 {
            let length = (direction.x * direction.x + direction.y * direction.y).sqrt();
            direction.x /= length;
            direction.y /= length;
        }

        // Apply acceleration and deacceleration based on the normalized direction
        self.vel.x += direction.x * player_acc;
        self.vel.y += direction.y * player_acc;

        // Limit velocity to player_speed
        let velocity_length = (self.vel.x * self.vel.x + self.vel.y * self.vel.y).sqrt();
        if velocity_length > player_speed {
            let scale = player_speed / velocity_length;
            self.vel.x *= scale;
            self.vel.y *= scale;
        }

        // Apply deacceleration if no keys are pressed
        if direction.x == 0.0 && direction.y == 0.0 && velocity_length != 0.0 {
            let scale = (velocity_length - player_deacc).max(0.0) / velocity_length;
            self.vel.x *= scale;
            self.vel.y *= scale;
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
