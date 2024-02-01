use lighting::*;
use player::*;
use raylib::prelude::*;

mod lighting;
mod player;

fn main() {
    let (mut rl, thread) = raylib::init()
        .vsync()
        .width(1000)
        .height(700)
        .resizable()
        .build();
    let mut shader =
        rl.load_shader_from_memory(&thread, None, Some(include_str!("../shaders/lighting.fs")));

    let mut target = rl
        .load_render_texture(
            &thread,
            rl.get_screen_width() as u32,
            rl.get_screen_height() as u32,
        )
        .unwrap();

    let mut light_engine = LightEngine::new(&mut shader);
    light_engine.spawn_light(Light::Ambient {
        color: Vector4::new(1.0, 1.0, 1.0, 0.0),
    });
    let mut camera = Camera2D {
        offset: Vector2::zero(),
        ..Default::default()
    };
    let mouse_light = light_engine.spawn_light(Light::Radial {
        pos: Vector2::zero(),
        color: Color::WHITE.into(),
        radius: 350.0,
    });
    let _ambient_light = light_engine.spawn_light(Light::Ambient {
        color: Vector4::new(1.0, 1.0, 1.0, 0.3),
    });

    let background_texture = rl.load_texture(&thread, "background.png").unwrap();

    let mut player = Player::new(&mut rl, &thread);

    let mut rot = 0.0;

    while !rl.window_should_close() {
        /* ---- Update ---- */
        let screen_size = Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32);
        light_engine.handle_spawning_light(&mut rl, &camera);
        light_engine.handle_mouse_light(&mut rl, &mouse_light, &camera);

        player.handle_movement(&rl);
        camera.offset = -player.pos + screen_size / 2.0;

        /* ----- Draw ----- */
        if rl.is_window_resized() {
            target = rl
                .load_render_texture(&thread, screen_size.x as u32, screen_size.y as u32)
                .unwrap();
        }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        light_engine.update_shader_values(&mut shader, &camera, screen_size);

        // Drawing to target
        {
            let mut tg = d.begin_texture_mode(&thread, &mut target);
            //let (mouse_x, mouse_y) = (tg.get_mouse_x(), tg.get_mouse_y());
            tg.clear_background(Color::WHITE);
            //tg.draw_circle(mouse_x, mouse_y, 50.0, Color::BLUE);

            // Drawing world
            for x in 0..10 {
                for y in 0..10 {
                    tg.draw_texture_pro(
                        &background_texture,
                        Rectangle::new(
                            0.0,
                            0.0,
                            background_texture.width as f32,
                            background_texture.height as f32,
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
                &player.frames[0],
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
            rot += 100.0 * tg.get_frame_time();
            if rot > 360.0 {
                rot = 0.0
            }
        }
        {
            // Render target with shader
            let mut sh = d.begin_shader_mode(&shader);
            sh.draw_texture(&target, 0, 0, Color::WHITE);
        }
        d.draw_fps(0, 0);
        dbg!(player.pos);
    }
}
