use crate::{player::*, world::*, DebugInfo, ImprovedCamera, Line, WorldMap};
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
                Some(include_str!("../shaders/lighting.fs")),
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
        world: &World,
        debug_info: &DebugInfo,
    ) {
        // Draw world onto the renderers target
        self.clear_target(d, thread);
        self.draw_floor(d, thread, &world.map, camera);
        self.draw_walls(d, thread, &world.map, camera);
        self.draw_player(d, thread, camera, player);
        self.draw_bullets(&world.bullets, d, thread, camera);

        if debug_info.debug {
            self.draw_debug_colliders(thread, d, player, world, camera);
        }

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
                let tile_x = (tile.varient as u32) % texture_width;
                let tile_y = (tile.varient as u32) / texture_width;

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
                        camera.to_screen_y(y as f32 * TILE_SIZE + rot_offset.y),
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

    pub fn draw_bullets(
        &mut self,
        bullets: &[Bullet],
        d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
        camera: &Camera2D,
    ) {
        let mut tg = d.begin_texture_mode(thread, &mut self.target);
        for bullet in bullets.iter() {
            // Long transparent trail
            tg.draw_line_ex(
                camera.to_screen(bullet.pos_history[1]),
                camera.to_screen(bullet.pos_history[2]),
                2.0,
                Color::new(20, 20, 20, 255),
            );

            tg.draw_line_ex(
                camera.to_screen(bullet.pos_history[0]),
                camera.to_screen(bullet.pos_history[1]),
                2.0,
                Color::new(120, 120, 120, 255),
            );

            // Bullet
            tg.draw_line_ex(
                camera.to_screen(bullet.pos),
                camera.to_screen(bullet.pos_history[0]),
                2.0,
                Color::WHITE,
            );
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
                if let Some(tile) = &map.walls[y as usize][x as usize] {
                    let tile_x = (tile.varient as i32) % (texture.width / TILE_SIZE as i32);
                    let tile_y = (tile.varient as i32) / (texture.width / TILE_SIZE as i32);
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
                            camera.to_screen_y(y as f32 * TILE_SIZE + rot_offset.y),
                            TILE_SIZE * camera.zoom + 0.01 * TILE_SIZE,
                            TILE_SIZE * camera.zoom + 0.01 * TILE_SIZE,
                        ),
                        Vector2::zero(),
                        tile.rotation.get_angle(),
                        Color::WHITE,
                    );
                }
            }
        }
    }

    // Draws debug information about tiles
    fn _draw_debug_grid(
        &mut self,
        thread: &RaylibThread,
        d: &mut RaylibDrawHandle,
        map: &WorldMap,
        camera: &Camera2D,
    ) {
        let mut tg = d.begin_texture_mode(thread, &mut self.target);
        // Drawing debug tile grid
        for y in 0..map.height {
            for x in 0..map.width {
                tg.draw_rectangle_lines_ex(
                    Rectangle::new(
                        camera.to_screen_x(x as f32 * TILE_SIZE),
                        camera.to_screen_y(y as f32 * TILE_SIZE),
                        TILE_SIZE * camera.zoom,
                        TILE_SIZE * camera.zoom,
                    ),
                    0.17 * camera.zoom,
                    Color::YELLOW,
                )
            }
        }
    }

    fn draw_debug_colliders(
        &mut self,
        thread: &RaylibThread,
        d: &mut RaylibDrawHandle,
        player: &Player,
        world: &World,
        camera: &Camera2D,
    ) {
        let mut tg = d.begin_texture_mode(thread, &mut self.target);
        // Drawing debug colliders for walls
        for y in 0..world.map.height {
            for x in 0..world.map.height {
                if let Some(wall) = &world.map.walls[y as usize][x as usize] {
                    for rect in &wall
                        .get_collider()
                        .with_pos(Vector2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE))
                        .rects
                    {
                        tg.draw_rectangle(
                            camera.to_screen_x(rect.x) as i32,
                            camera.to_screen_y(rect.y) as i32,
                            (rect.width * camera.zoom) as i32,
                            (rect.height * camera.zoom) as i32,
                            Color::BLUE,
                        );

                        tg.draw_triangle(
                            camera.to_screen(Vector2::new(rect.x, rect.y)),
                            camera.to_screen(Vector2::new(rect.x + rect.width, rect.y)),
                            camera.to_screen(Vector2::new(rect.x, rect.y + rect.height)),
                            Color::GRAY,
                        );

                        for line in Line::from_rect(rect) {
                            tg.draw_line_ex(
                                camera.to_screen(line.start),
                                camera.to_screen(line.end),
                                3.0,
                                Color::GREEN
                            );
                            tg.draw_line_ex(
                                camera.to_screen(line.start),
                                camera.to_screen(line.end),
                                3.0,
                                Color::GREEN
                            );
                            for bullet in world.bullets.iter() {
                                for dbg_line in bullet.dbg_lines.iter() {
                                    tg.draw_line_ex(
                                        camera.to_screen(dbg_line.start),
                                        camera.to_screen(dbg_line.end),
                                        5.0,
                                        Color::GREEN
                                    )
                                }
                            }
                        }
                    }
                }
            }
        }

        tg.draw_rectangle_rec(
            camera.to_screen_rect(&player.get_world_collider().rects[0]),
            Color::RED,
        );

        for (y, wall_line) in world.map.walls.iter().enumerate() {
            for (x, wall) in wall_line.iter().enumerate() {
                if let Some(wall) = wall {
                    if let Some(collider) = wall
                        .get_collider()
                        .with_pos(Vector2::new(x as f32 * 32.0, y as f32 * 32.0))
                        .collides(&player.get_world_collider())
                    {
                        tg.draw_rectangle_rec(camera.to_screen_rect(&collider), Color::WHITE);
                    }
                }
            }
        }
    }
}
