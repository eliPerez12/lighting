use raylib::prelude::*;
use crate::player::*;


pub struct Renderer {
    target: RenderTexture2D,
    background_texture: Texture2D,
    pub shader: Shader,
}

impl Renderer {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Renderer {
        Renderer {
            shader: rl.load_shader_from_memory(thread, None, Some(include_str!("../shaders/lighting.fs"))),
            target: rl
            .load_render_texture(
                thread,
                rl.get_screen_width() as u32,
                rl.get_screen_height() as u32,
            ).unwrap(),
            background_texture: rl.load_texture(thread, "background.png").unwrap(),
        }
    }

    pub fn update_target(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, screen_size: Vector2) {
        if rl.is_window_resized() {
            self.target = rl
                .load_render_texture(thread, screen_size.x as u32, screen_size.y as u32)
                .unwrap();
        }
    }
    pub fn draw_world(
        &mut self,
        d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
        player: &Player,
        camera: &Camera2D,
    ) {
        let mut tg = d.begin_texture_mode(thread, &mut self.target);
        tg.clear_background(Color::WHITE);
    
        // Drawing world
        for x in 0..25 {
            for y in 0..25 {
                tg.draw_texture_pro(
                    &self.background_texture,
                    Rectangle::new(
                        0.0,
                        0.0,
                        self.background_texture.width as f32,
                        self.background_texture.height as f32,
                    ),
                    Rectangle::new(
                        x as f32 * 100.0 + camera.offset.x,
                        y as f32 * 100.0 + camera.offset.y,
                        100.0,
                        100.0,
                    ),
                    Vector2::zero(),
                    0.0,
                    Color::WHITE,
                )
            }
        }
        let player_screen_pos = player.pos + camera.offset;
        tg.draw_circle_v(
            player_screen_pos,
            10.0,
            Color::BLUE,
        );
        let mouse_pos = tg.get_mouse_position();
        let angle_to_mouse = (mouse_pos.y - player_screen_pos.y).atan2(mouse_pos.x - player_screen_pos.x).to_degrees() + 90.0;
    
        // Drawing player
        tg.draw_texture_pro(
            player.get_animation_frame(),
            Rectangle::new(0.0, 0.0, 26.0, 42.0),
            Rectangle::new(
                player_screen_pos.x,// - Player::RENDER_SIZE.x / 2.0,
                player_screen_pos.y,// - Player::RENDER_SIZE.y / 2.0,
                Player::RENDER_SIZE.x,
                Player::RENDER_SIZE.y,
            ),
            Player::RENDER_SIZE/2.0,
            angle_to_mouse,
            Color::WHITE,
        );
        drop(tg);
        // Render target with shader
        let mut sh = d.begin_shader_mode(&self.shader);
        sh.draw_texture(&self.target, 0, 0, Color::WHITE);
    }
}