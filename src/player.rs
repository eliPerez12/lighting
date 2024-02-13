use raylib::prelude::*;


pub struct Player {
    pub pos: Vector2,
    animation: PlayerAnimation,
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
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) ||
        rl.is_key_down(KeyboardKey::KEY_SPACE) {
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

impl Player {
    pub const RENDER_SIZE: Vector2 = Vector2::new(26.0, 42.0);
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Player {
        Player {
            pos: Vector2::zero(),
            animation: PlayerAnimation::new(rl, thread),
        }
    }

    pub fn get_animation_frame(&self) -> &Texture2D {
        &self.animation.frames[self.animation.current_frame]
    }

    pub fn handle_movement(&mut self, rl: &RaylibHandle) {
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
        self.animation.handle_animation(rl);
    }
}
