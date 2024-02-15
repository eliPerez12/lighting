use crate::{player::*, DebugInfo, ImprovedCamera, WorldMap};
use raylib::prelude::*;

pub struct Renderer {
    target: RenderTexture2D,
    floor_tile_sheet: Texture2D,
    wall_tile_sheet: Texture2D,
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
            floor_tile_sheet: rl
                .load_texture(thread, "assets/background/floor_tile_sheet.png")
                .unwrap(),
            wall_tile_sheet: rl
                .load_texture(thread, "assets/background/wall_tile_sheet.png")
                .unwrap(),
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
        map: &WorldMap,
        debug_info: &DebugInfo,
    ) {
        let mut tg = d.begin_texture_mode(thread, &mut self.target);
        tg.clear_background(Color::BLACK);

        // Drawing background tiles
        (0..map.ground.len()).for_each(|y| {
            for x in 0..map.ground[y].len() {
                let texture = &self.floor_tile_sheet;
                let render_size = 32.0;
                let tile = map.ground[y][x];
                if tile == 0 {
                    continue;
                }
                let texture_width = self.floor_tile_sheet.width() as u32 / render_size as u32;
                let tile_x = (tile - 1) % texture_width;
                let tile_y = (tile - 1) / texture_width;
                tg.draw_texture_pro(
                    texture,
                    Rectangle::new(tile_x as f32 * 32.0, tile_y as f32 * 32.0, 32.0, 32.0),
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
            }
        });
        // Drawing wall tile
        (0..map.walls.len()).for_each(|y| {
            for x in 0..map.walls[y].len() {
                let texture = &self.wall_tile_sheet;
                let render_size = 32.0;
                let tile = map.walls[y][x];
                if tile == 0 {
                    continue;
                }

                let texture_width = self.wall_tile_sheet.width() as u32 / render_size as u32;
                let tile_x = ((tile & 0x0FFFFFFF) - 65) % texture_width;
                let tile_y = ((tile & 0x0FFFFFFF) - 65) / texture_width;
                //dbg!(tile & 0x0FFFFFFF);
                tg.draw_texture_pro(
                    texture,
                    Rectangle::new(tile_x as f32 * 32.0, tile_y as f32 * 32.0, 32.0, 32.0),
                    Rectangle::new(
                        (x as f32 * render_size + camera.offset.x) * camera.zoom,
                        (y as f32 * render_size + camera.offset.y) * camera.zoom,
                        render_size * camera.zoom + 0.001 * 32.0,
                        render_size * camera.zoom + 0.001 * 32.0,
                    ),
                    Vector2::new(0.0, 0.0),
                    get_rot(tile),
                    Color::WHITE,
                );
            }
        });

        if debug_info.debug {
            // Drawing debug tile grid
            (0..map.ground.len()).for_each(|y| {
                for x in 0..map.ground[y].len() {
                    tg.draw_rectangle_lines_ex(
                        Rectangle::new(
                            (x as f32 * 32.0 + camera.offset.x) * camera.zoom,
                            (y as f32 * 32.0 + camera.offset.y) * camera.zoom,
                            32.0 * camera.zoom, 
                            32.0 * camera.zoom
                        ),
                        0.5 * camera.zoom,
                        Color::LIGHTGRAY,
                    )
                }
            });
        }

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


fn get_rot(tile: u32) -> f32 {
    match (
        (tile & 0x20000000) != 0, // Flip diagonaly
        (tile & 0x80000000) != 0, // Flip x
        (tile & 0x40000000) != 0, // Flip y
    ) {
        (false, true, true) => 0.0,
        (true, true, false) => 90.0,
        (true, false, true) => 270.0,
        _ => 0.0,

    }
}