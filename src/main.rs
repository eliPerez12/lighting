use raylib::prelude::*;
use lighting::*;

mod lighting;

fn main() {
    let (mut rl, thread) = raylib::init().width(1000).height(700).resizable().build();
    let mut shader = rl.load_shader_from_memory(
        &thread,
        None,
        Some(include_str!("../shaders/lighting.fs")),
    );

    let sh_screen_size_loc = shader.get_shader_location("screenSize");
    let mut target = rl.load_render_texture(&thread, rl.get_screen_width() as u32, rl.get_screen_height() as u32).unwrap();
    
    let mut light_engine = LightEngine::new(&mut shader);
    //light_engine.spawn_light(Light::Radial { pos: Vector2::new(500.0, 500.0), color: Color::BLUE.into(), radius: 250.0});
    light_engine.spawn_light(Light::Ambient { color: Vector4::new(1.0, 1.0, 1.0, 0.9)});

    while !rl.window_should_close() {
        /* ---- Update ---- */

        //light_engine.update_light(&light0, Light::Radial { pos: rl.get_mouse_position(), color: Color::WHITE.into() });

        if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
            light_engine.spawn_light(Light::Radial { pos: rl.get_mouse_position(), color: Color::RED.into(), radius: 350.0});
        }
        if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
            light_engine.spawn_light(Light::Radial { pos: rl.get_mouse_position(), color: Color::BLUE.into(), radius: 350.0});
        }
        if rl.is_key_pressed(KeyboardKey::KEY_THREE) {
            light_engine.spawn_light(Light::Radial { pos: rl.get_mouse_position(), color: Color::YELLOW.into(), radius: 350.0});
        }

        /* ----- Draw ----- */
        let screen_size = Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32);
        if rl.is_window_resized() {
            target = rl.load_render_texture(&thread, screen_size.x as u32, screen_size.y as u32).unwrap();
        }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

    
        // Update shader with screen size
        shader.set_shader_value(
            sh_screen_size_loc,
            screen_size,
        );

        light_engine.update_shader_values(&mut shader);

        // Drawing to target
        {
            let mut tg = d.begin_texture_mode(&thread, &mut target);
            //let (mouse_x, mouse_y) = (tg.get_mouse_x(), tg.get_mouse_y());
            tg.clear_background(Color::WHITE);
            //tg.draw_circle(mouse_x, mouse_y, 50.0, Color::BLUE);
        }

        {
        // Render target with shader
        let mut sh = d.begin_shader_mode(&shader);
        sh.draw_texture(&target, 0, 0, Color::WHITE);
        }
        d.draw_fps(0, 0);
    }
}
