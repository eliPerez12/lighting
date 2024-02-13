use crate::{player::*, ImprovedCamera};
use raylib::prelude::*;

pub struct Renderer {
    target: RenderTexture2D,
    background_tile_sheet: Texture2D,
    pub shader: Shader,
}

impl Renderer {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Renderer {
        Renderer {
            shader: rl.load_shader_from_memory(
                thread,
                None,
                Some(include_str!("../assets/shaders/lighting.fs")),
            ),
            target: rl
                .load_render_texture(
                    thread,
                    rl.get_screen_width() as u32,
                    rl.get_screen_height() as u32,
                )
                .unwrap(),
            background_tile_sheet: 
                rl.load_texture(thread, "assets/background/background_tile_sheet.png")
                    .unwrap()
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
        floor_map: &Vec<Vec<i64>>,
    ) {
        let mut tg = d.begin_texture_mode(thread, &mut self.target);
        tg.clear_background(Color::BLACK);

        // Drawing background tile
        (0..floor_map.len()).for_each(|y| {
            (0..floor_map[y].len()).for_each(|x| {
                let texture = &self.background_tile_sheet;
                let render_size = 32.0;
                let tile = floor_map[y][x];
                let tile_x = tile % 30;
                let tile_y = tile / 20;
                tg.draw_texture_pro(
                    texture,
                    Rectangle::new(tile_x as f32 * 32.0,tile_y as f32 * 32.0, 32.0, 32.0),
                    Rectangle::new(
                        (x as f32 * render_size + camera.offset.x) * camera.zoom,
                        (y as f32 * render_size + camera.offset.y) * camera.zoom,
                        render_size * camera.zoom + 0.01 * 32.0,
                        render_size * camera.zoom + 0.01 * 32.0,
                    ),
                    Vector2::zero(),
                    0.0,
                    Color::WHITE,
                )
            });
        });
        let player_screen_pos = camera.to_screen(player.pos);
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
                player_screen_pos.x,
                player_screen_pos.y,
                Player::RENDER_SIZE.x * camera.zoom,
                Player::RENDER_SIZE.y * camera.zoom,
            ),
            (Player::RENDER_SIZE / 2.0) * camera.zoom,
            angle_to_mouse,
            Color::WHITE,
        );
        drop(tg);
        // Render target with shader
        let mut sh = d.begin_shader_mode(&self.shader);
        sh.draw_texture(&self.target, 0, 0, Color::WHITE);
    }
}
