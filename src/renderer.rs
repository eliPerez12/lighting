use crate::{player::*, DebugInfo, ImprovedCamera, WorldMap};
use raylib::prelude::*;

pub const TILE_SIZE: f32 = 32.0;

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

    // Clears the internal target with black background
    fn clear_target(&mut self, d: &mut RaylibDrawHandle, thread: &RaylibThread) {
        d.begin_texture_mode(thread, &mut self.target)
            .clear_background(Color::BLACK);
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
            //self.draw_debug_colliders(thread, d, map, camera);
        }
        self.draw_player(d, thread, camera, player);

        // Render target with shader
        let mut sh = d.begin_shader_mode(&self.shader);
        sh.draw_texture(&self.target, 0, 0, Color::WHITE);
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

        // Draw player's shadow
        tg.draw_circle(
            player_screen_pos.x as i32,
            player_screen_pos.y as i32,
            8.0 * camera.zoom,
            Color::new(0, 0, 0, 50),
        );
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
                let tile = &map.ground[y as usize][x as usize];
                let texture_width = self.floor_tile_sheet.width() as u32 / TILE_SIZE as u32;
                let tile_x = (tile.varient - 1) % texture_width;
                let tile_y = (tile.varient - 1) / texture_width;

                tg.draw_texture_pro(
                    texture,
                    Rectangle::new(
                        tile_x as f32 * TILE_SIZE,
                        tile_y as f32 * TILE_SIZE,
                        TILE_SIZE,
                        TILE_SIZE,
                    ),
                    Rectangle::new(
                        camera.to_screen_x(x as f32 * TILE_SIZE),
                        camera.to_screen_y(y as f32 * TILE_SIZE),
                        TILE_SIZE * camera.zoom + 0.01 * TILE_SIZE,
                        TILE_SIZE * camera.zoom + 0.01 * TILE_SIZE,
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
                let tile = &map.walls[y as usize][x as usize];

                let tile_x = (tile.varient as i32 - 1) % (texture.width / TILE_SIZE as i32);
                let tile_y = (tile.varient as i32 - 1) / (texture.width / TILE_SIZE as i32);
                let rot_offset = tile.rotation.get_rotation_offset();

                tg.draw_texture_pro(
                    texture,
                    Rectangle::new(
                        tile_x as f32 * TILE_SIZE,
                        tile_y as f32 * TILE_SIZE,
                        TILE_SIZE,
                        TILE_SIZE,
                    ),
                    Rectangle::new(
                        camera.to_screen_x(x as f32 * TILE_SIZE + rot_offset.x),
                        camera.to_screen_x(y as f32 * TILE_SIZE + rot_offset.y),
                        TILE_SIZE * camera.zoom + 0.001 * TILE_SIZE,
                        TILE_SIZE * camera.zoom + 0.001 * TILE_SIZE,
                    ),
                    Vector2::zero(),
                    tile.rotation.get_angle(),
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
                        camera.to_screen_x(x as f32 * TILE_SIZE),
                        camera.to_screen_y(y as f32 * TILE_SIZE),
                        TILE_SIZE * camera.zoom,
                        TILE_SIZE * camera.zoom,
                    ),
                    0.33 * camera.zoom,
                    Color::DARKGREEN,
                )
            }
        }
    }

    fn _draw_debug_colliders(
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
                let tile = &map.walls[y as usize][x as usize];
                if tile.varient == 0 {
                    continue;
                }
                let rect = match tile.varient {
                    0 => Rectangle::new(0.0, 0.0, 8.0, 32.0),
                    _ => Rectangle::new(0.0, 0.0, 0.0, 0.0),
                };
                tg.draw_rectangle(
                    camera.to_screen_x(rect.x + x as f32 * TILE_SIZE) as i32,
                    camera.to_screen_y(rect.y + y as f32 * TILE_SIZE) as i32,
                    (rect.width * camera.zoom) as i32,
                    (rect.height * camera.zoom) as i32,
                    Color::BLUE,
                )
            }
        }
    }
}
