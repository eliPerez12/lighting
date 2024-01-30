use lighting::*;
use raylib::prelude::*;

mod lighting;

fn handle_spawning_light(rl: &mut RaylibHandle, light_engine: &mut LightEngine, camera: &Camera2D) {
    let light_radius = 800.0;
    if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
        light_engine.spawn_light(Light::Radial {
            pos: rl.get_mouse_position() - camera.offset,
            color: Color::RED.into(),
            radius: light_radius,
        });
    }
    if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
        light_engine.spawn_light(Light::Radial {
            pos: rl.get_mouse_position() - camera.offset,
            color: Color::BLUE.into(),
            radius: light_radius,
        });
    }
    if rl.is_key_pressed(KeyboardKey::KEY_THREE) {
        light_engine.spawn_light(Light::Radial {
            pos: rl.get_mouse_position() - camera.offset,
            color: Color::YELLOW.into(),
            radius: light_radius,
        });
    }
    if rl.is_key_pressed(KeyboardKey::KEY_FOUR) {
        light_engine.spawn_light(Light::Radial {
            pos: rl.get_mouse_position() - camera.offset,
            color: Color::WHITE.into(),
            radius: light_radius,
        });
    }
}

fn handle_mouse_light(rl: &mut RaylibHandle, light: &LightHandle, light_engine: &mut LightEngine, camera: &Camera2D) {
    let light = light_engine.get_mut_light(light);
    if let Light::Radial{pos, ..} = light {
        *pos = rl.get_mouse_position() - camera.offset;
    }
}

fn handle_camera_controls(rl: &mut RaylibHandle, camera: &mut Camera2D) {
    let camera_speed = 100.0 * rl.get_frame_time();
    if rl.is_key_down(KeyboardKey::KEY_W) {
        camera.offset.y += camera_speed;
    }
    if rl.is_key_down(KeyboardKey::KEY_S) {
        camera.offset.y -= camera_speed;
    }
    if rl.is_key_down(KeyboardKey::KEY_A) {
        camera.offset.x += camera_speed;
    }
    if rl.is_key_down(KeyboardKey::KEY_D) {
        camera.offset.x -= camera_speed;
    }
}


fn main() {
    let (mut rl, thread) = raylib::init().width(1000).height(700).resizable().build();
    let mut shader =
        rl.load_shader_from_memory(&thread, None, Some(include_str!("../shaders/lighting.fs")));

    let sh_screen_size_loc = shader.get_shader_location("screenSize");
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
    let mut camera = Camera2D {offset: Vector2::zero(), ..Default::default()};
    let mouse_light = light_engine.spawn_light(Light::Radial { pos: Vector2::zero(), color: Color::WHITE.into(), radius: 350.0 });

    let background_texture = rl.load_texture(&thread, "background.png").unwrap();

    while !rl.window_should_close() {
        /* ---- Update ---- */
        handle_spawning_light(&mut rl, &mut light_engine, &camera);
        handle_mouse_light(&mut rl, &mouse_light, &mut light_engine, &camera);
        handle_camera_controls(&mut rl, &mut camera);

        /* ----- Draw ----- */
        let screen_size = Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32);
        if rl.is_window_resized() {
            target = rl
                .load_render_texture(&thread, screen_size.x as u32, screen_size.y as u32)
                .unwrap();
        }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // Update shader with screen size
        shader.set_shader_value(sh_screen_size_loc, screen_size);

        light_engine.update_shader_values(&mut shader, &camera);

        // Drawing to target
        {
            let mut tg = d.begin_texture_mode(&thread, &mut target);
            //let (mouse_x, mouse_y) = (tg.get_mouse_x(), tg.get_mouse_y());
            tg.clear_background(Color::WHITE);
            //tg.draw_circle(mouse_x, mouse_y, 50.0, Color::BLUE);

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
                            100.0
                        ),
                        Vector2::zero(),
                        0.0,
                        Color::WHITE,
                    )
                }
            }
        }

        {
            // Render target with shader
            let mut sh = d.begin_shader_mode(&shader);
            sh.draw_texture(&target, 0, 0, Color::WHITE);
        }
        d.draw_fps(0, 0);
    }
}
