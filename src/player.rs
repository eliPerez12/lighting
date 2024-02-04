use raylib::prelude::*;

pub struct Player {
    pub pos: Vector2,
    pub frames: Vec<Texture2D>,
    pub current_frame: usize,
    pub elapsed_time: f32,
}


impl Player {
    pub const RENDER_SIZE: Vector2 = Vector2::new(100.0, 161.0);
    const FRAME_AMOUNT: usize = 4;
    const FRAME_TIME: f32 = 0.018;
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Player {
        Player {
            pos: Vector2::zero(),
            frames: (1..=5)
                .map(|i| {
                    rl.load_texture(thread, &format!("assets/player_body{i}.png"))
                        .unwrap()
                })
                .collect::<Vec<Texture2D>>(),
                current_frame: 0,
                elapsed_time: 0.0,
        }
    }

    pub fn handle_movement(&mut self, rl: &RaylibHandle) {
        let player_speed = 100.0 * rl.get_frame_time();
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


        // If player is trying to ads
        if rl.is_key_down(KeyboardKey::KEY_SPACE) {
            if self.elapsed_time > Self::FRAME_TIME || self.current_frame == 0 {
                if self.current_frame < Self::FRAME_AMOUNT {
                    self.current_frame += 1;
                }
                self.elapsed_time = 0.0;
            }
            self.elapsed_time += rl.get_frame_time();
        } else if self.current_frame > 0 {
            if self.elapsed_time <= - Self::FRAME_TIME || self.current_frame == Self::FRAME_AMOUNT{
                self.current_frame -= 1;
                self.elapsed_time = 0.0;
            }
            self.elapsed_time -= rl.get_frame_time();
        }
    }
}