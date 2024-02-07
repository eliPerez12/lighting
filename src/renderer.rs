use crate::player::*;
use raylib::prelude::*;

pub struct Renderer {
    target: RenderTexture2D,
    background_textures: Vec<Texture2D>,
    pub shader: Shader,
}

impl Renderer {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Renderer {
        Renderer {
            shader: rl.load_shader_from_memory(
                thread,
                None,
                Some(include_str!("../shaders/lighting.fs")),
            ),
            target: rl
                .load_render_texture(
                    thread,
                    rl.get_screen_width() as u32,
                    rl.get_screen_height() as u32,
                )
                .unwrap(),
            background_textures: (0..5)
                .map(|i| {
                    rl.load_texture(thread, &format!("assets/background/background_{}.png", i))
                        .unwrap()
                })
                .collect(),
        }
    }

    pub fn update_target(
        &mut self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        screen_size: Vector2,
    ) {
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
        map: &Vec<Vec<i32>>,
    ) {
        let mut tg = d.begin_texture_mode(thread, &mut self.target);
        tg.clear_background(Color::WHITE);

        // Drawing world
        for x in 0..map.len() {
            (0..map[0].len()).for_each(|y| {
                let texture = &self.background_textures[map[y][x] as usize];
                let render_size = 80.0;
                tg.draw_texture_pro(
                    texture,
                    Rectangle::new(0.0, 0.0, texture.width as f32, texture.height as f32),
                    Rectangle::new(
                        x as f32 * render_size + camera.offset.x,
                        y as f32 * render_size + camera.offset.y,
                        render_size,
                        render_size,
                    ),
                    Vector2::zero(),
                    0.0,
                    Color::WHITE,
                )
            });
        }
        let player_screen_pos = player.pos + camera.offset;
        let mouse_pos = tg.get_mouse_position();
        let angle_to_mouse = (mouse_pos.y - player_screen_pos.y)
            .atan2(mouse_pos.x - player_screen_pos.x)
            .to_degrees()
            + 90.0;

        // Drawing player
        tg.draw_texture_pro(
            player.get_animation_frame(),
            Rectangle::new(0.0, 0.0, 26.0, 42.0),
            Rectangle::new(
                player_screen_pos.x, // - Player::RENDER_SIZE.x / 2.0,
                player_screen_pos.y, // - Player::RENDER_SIZE.y / 2.0,
                Player::RENDER_SIZE.x,
                Player::RENDER_SIZE.y,
            ),
            Player::RENDER_SIZE / 2.0,
            angle_to_mouse,
            Color::WHITE,
        );
        drop(tg);
        // Render target with shader
        let mut sh = d.begin_shader_mode(&self.shader);
        sh.draw_texture(&self.target, 0, 0, Color::WHITE);
    }
}
