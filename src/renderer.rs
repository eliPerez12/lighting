use crate::{player::*, DebugInfo, ImprovedCamera, WorldMap};
use raylib::prelude::*;

pub struct Renderer {
    pub shader: Shader,
    target: RenderTexture2D,
    floor_tile_sheet: Texture2D,
    wall_tile_sheet: Texture2D,
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

    // Draws the world to the screen
    pub fn draw_world(
        &mut self,
        d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
        player: &Player,
        camera: &Camera2D,
        map: &WorldMap,
        debug_info: &DebugInfo,
    ) {
        // Draw world onto the renderers target
        self.clear_target(d, thread);
        self.draw_floor(d, thread, map, camera);
        self.draw_walls(d, thread, map, camera);
        if debug_info.debug {
            self.draw_debug_grid(thread, d, map, camera);
        }
        self.draw_player(d, thread, camera, player);

        // Render target with shader
        let mut sh = d.begin_shader_mode(&self.shader);
        sh.draw_texture(&self.target, 0, 0, Color::WHITE);
    }

    // Clears the internal target with black background
    fn clear_target(&mut self, d: &mut RaylibDrawHandle, thread: &RaylibThread) {
        d.begin_texture_mode(thread, &mut self.target)
            .clear_background(Color::BLACK);
    }

    // Updates internal renderer target to resize with the window
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

    // Draws all the floors in the world
    fn draw_floor(
        &mut self,
        d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
        map: &WorldMap,
        camera: &Camera2D,
    ) {
        let mut tg = d.begin_texture_mode(thread, &mut self.target);
        for y in 0..map.height {
            for x in 0..map.width {
                let texture = &self.floor_tile_sheet;
                let render_size = 32.0;
                let tile = map.ground[y as usize][x as usize];
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
        }
    }

    // Draws the walls
    fn draw_walls(
        &mut self,
        d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
        map: &WorldMap,
        camera: &Camera2D,
    ) {
        let mut tg = d.begin_texture_mode(thread, &mut self.target);
        for y in 0..map.height {
            for x in 0..map.height {
                let texture = &self.wall_tile_sheet;
                let render_size = 32.0;
                let tile = map.walls[y as usize][x as usize];
                if tile == 0 {
                    continue;
                }

                let tile_x =
                    ((tile as i32 & 0x0FFFFFFF) - 65) % (texture.width / render_size as i32);
                let tile_y =
                    ((tile as i32 & 0x0FFFFFFF) - 65) / (texture.width / render_size as i32);

                let flags = tile >> 28; // Get first byte from tile
                let rot = match flags {
                    0 => 0.0,
                    6 => 270.0,
                    10 => 90.0,
                    12 => 180.0,
                    _ => 0.0,
                };
                let rot_offset = match flags {
                    0 => Vector2::zero(),
                    6 => Vector2::new(0.0, render_size),
                    12 => Vector2::new(render_size, render_size),
                    10 => Vector2::new(render_size, 0.0),
                    _ => Vector2::zero(),
                };
                tg.draw_texture_pro(
                    texture,
                    Rectangle::new(tile_x as f32 * 32.0, tile_y as f32 * 32.0, 32.0, 32.0),
                    Rectangle::new(
                        (x as f32 * render_size + camera.offset.x + rot_offset.x) * camera.zoom,
                        (y as f32 * render_size + camera.offset.y + rot_offset.y) * camera.zoom,
                        render_size * camera.zoom + 0.001 * 32.0,
                        render_size * camera.zoom + 0.001 * 32.0,
                    ),
                    Vector2::zero(),
                    rot,
                    Color::WHITE,
                );
            }
        }
    }

    // Draws debug information about tiles
    fn draw_debug_grid(
        &mut self,
        thread: &RaylibThread,
        d: &mut RaylibDrawHandle,
        map: &WorldMap,
        camera: &Camera2D,
    ) {
        let mut tg = d.begin_texture_mode(thread, &mut self.target);
        // Drawing debug tile grid
        for y in 0..map.height {
            for x in 0..map.height {
                tg.draw_rectangle_lines_ex(
                    Rectangle::new(
                        (x as f32 * 32.0 + camera.offset.x) * camera.zoom,
                        (y as f32 * 32.0 + camera.offset.y) * camera.zoom,
                        32.0 * camera.zoom,
                        32.0 * camera.zoom,
                    ),
                    0.5 * camera.zoom,
                    Color::LIGHTGRAY,
                )
            }
        }
    }

    // Draws the player
    fn draw_player(
        &mut self,
        d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
        camera: &Camera2D,
        player: &Player,
    ) {
        let mut tg = d.begin_texture_mode(thread, &mut self.target);
        let player_screen_pos = camera.to_screen(player.pos);
        let mouse_pos = tg.get_mouse_position();
        let angle_to_mouse = (mouse_pos.y - player_screen_pos.y)
            .atan2(mouse_pos.x - player_screen_pos.x)
            .to_degrees()
            + 90.0;

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
    }
}
