use crate::{
    bullet::Bullet, day_cycle, player::*, world::*, DebugInfo, ImprovedCamera, Line, WorldMap,
};
use raylib::prelude::*;

pub const TILE_SIZE: f32 = 32.0;

pub struct Renderer {
    pub shader: Shader,
    target: RenderTexture2D,
    shadow_target: RenderTexture2D,
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
            shadow_target: rl
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
            self.shadow_target = rl
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
        self.draw_wall_shadows(d, thread, world, camera);
        self.draw_walls(d, thread, &world.map, camera);
        self.draw_player(d, thread, camera, world, player);
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
        world: &World,
        player: &Player,
    ) {
        let mut tg = d.begin_texture_mode(thread, &mut self.target);
        let player_screen_pos = camera.to_screen(player.pos);

        // Draw player's shadow
        tg.draw_circle(
            player_screen_pos.x as i32,
            player_screen_pos.y as i32,
            8.0 * camera.zoom,
            world.day_cycle.get_shadow_color(),
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
            player
                .get_angle_to_screen_pos(tg.get_mouse_position(), camera)
                .to_degrees()
                + 90.0,
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
                        TILE_SIZE * camera.zoom + 0.001 * TILE_SIZE,
                        TILE_SIZE * camera.zoom + 0.001 * TILE_SIZE,
                    ),
                    Vector2::zero(),
                    tile.rotation.get_angle(),
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
                camera.to_screen(bullet.pos_history[0]),
                camera.to_screen(bullet.pos_history[2]),
                5.0,
                Color::new(0, 0, 0, 100),
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

                        for line in Line::from_rect(rect) {
                            tg.draw_line_ex(
                                camera.to_screen(line.start),
                                camera.to_screen(line.end),
                                3.0,
                                Color::GREEN,
                            );
                            tg.draw_line_ex(
                                camera.to_screen(line.start),
                                camera.to_screen(line.end),
                                3.0,
                                Color::GREEN,
                            );
                        }
                    }
                }
            }
        }

        // Drawing player collider
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

        // Drawing bullet debug info
        for bullet in world.bullets.iter() {
            if let Some(line) = &bullet.dbg_line_hit {
                tg.draw_line_ex(
                    camera.to_screen(line.start),
                    camera.to_screen(line.end),
                    20.0,
                    Color::ORANGERED,
                )
            }
        }
    }

    fn draw_wall_shadows(
        &mut self,
        d: &mut RaylibDrawHandle,
        thread: &RaylibThread,
        world: &World,
        camera: &Camera2D,
    ) {
        let screen_size = Vector2::new(d.get_screen_width() as f32, d.get_screen_height() as f32);

        let normilized_time = world.day_cycle.get_normilized_time();
        let shadow_color = world.day_cycle.get_shadow_color();
        let (shadow_x_length, shadow_y_length) = (12.0, 8.0);

        let mut shd = d.begin_texture_mode(thread, &mut self.shadow_target);
        shd.clear_background(Color::new(0, 0, 0, 0));

        // Drawing debug colliders for walls
        for y in 0..world.map.height {
            for x in 0..world.map.height {
                if let Some(wall) = &world.map.walls[y as usize][x as usize] {
                    for rect in &wall
                        .get_collider()
                        .with_pos(Vector2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE))
                        .rects
                    {
                        // Before noon
                        if !(day_cycle::NOON..=1.0 - day_cycle::SUNRISE_LENGTH)
                            .contains(&normilized_time)
                        {
                            let shadow_width = if normilized_time > 1.0 - day_cycle::SUNRISE_LENGTH
                            {
                                1.0
                            } else {
                                1.0 - normilized_time.max(day_cycle::SUNRISE) / 0.25
                            };
                            let screen_rect = Rectangle::new(
                                screen_size.x - camera.to_screen_x(rect.x + rect.width),
                                camera.to_screen_y(rect.y - shadow_width * shadow_y_length),
                                (rect.width + shadow_width * shadow_x_length) * camera.zoom,
                                (rect.height + shadow_width * shadow_y_length) * camera.zoom,
                            );
                            shd.draw_rectangle_rec(screen_rect, Color::new(255, 255, 255, 255));
                        }
                        // After noon
                        else if normilized_time > day_cycle::NOON {
                            let shadow_width =
                                (normilized_time.min(day_cycle::SUNSET) - day_cycle::NOON) * 4.0;
                            let screen_rect_width =
                                (rect.width + shadow_width * shadow_x_length) * camera.zoom;
                            let screen_rect = Rectangle::new(
                                (screen_size.x - camera.to_screen_x(rect.x)) - screen_rect_width,
                                camera.to_screen_y(rect.y),
                                screen_rect_width,
                                (rect.height + shadow_width * shadow_y_length) * camera.zoom,
                            );
                            shd.draw_rectangle_rec(screen_rect, Color::new(255, 255, 255, 255));
                        }
                    }
                }
            }
        }
        drop(shd);
        let mut tg = d.begin_texture_mode(thread, &mut self.target);
        tg.draw_texture_ex(&self.shadow_target, screen_size, 180.0, 1.0, shadow_color)
    }
}
