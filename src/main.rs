use raylib::prelude::*;


struct Light {
    position: Vector2,
    radius: f32,
    color: Vector4,
}


// fn update_shader_lights(
//     lights: Vec<Light>,
//     shader: &mut Shader,
//     position_loc: i32,
//     radius_loc: i32,
//     color_loc: i32) {
    
//     for (i, light) in lights.iter().enumerate() {
//         shader.set_shader_value(position_loc, light.position);
//     }

// }

fn main() {
    let (mut rl, thread) = raylib::init().width(1000).height(700).resizable().build();
    let mut shader = rl.load_shader_from_memory(
        &thread,
        None,
        Some(include_str!("../shaders/lighting.fs")),
    );

    let sh_lights_position_loc = shader.get_shader_location("lightsPosition");
    let sh_lights_color_loc = shader.get_shader_location("lightsColor");
    let sh_lights_amount_loc = shader.get_shader_location("lightsAmount");
    let sh_screen_size_loc = shader.get_shader_location("screenSize");
    let mut target = rl.load_render_texture(&thread, rl.get_screen_width() as u32, rl.get_screen_height() as u32).unwrap();
    //let lights = vec![Light {position: Vector2::new(0.0, 0.0), radius: 400.0, color: Vector4::new(1.0, 1.0, 1.0, 1.0)}];


    while !rl.window_should_close() {
        /* ---- Update ---- */


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
        shader.set_shader_value_v(
            sh_lights_position_loc,
            &[Vector2::new(0.0, 0.0), Vector2::new(500.0, 500.0)],
        );
        shader.set_shader_value(
            sh_lights_amount_loc,
            2,
        );
        shader.set_shader_value_v(
            sh_lights_color_loc,
            &[Vector4::new(1.0, 0.5, 0.5, 1.0), Vector4::new(0.5, 0.8, 1.0, 1.0)],
        );


        // Drawing to target
        {
            let mut tg = d.begin_texture_mode(&thread, &mut target);
            let (mouse_x, mouse_y) = (tg.get_mouse_x(), tg.get_mouse_y());
            tg.clear_background(Color::WHITE);
            tg.draw_circle(mouse_x, mouse_y, 50.0, Color::BLUE);
        }

        {
        // Render target with shader
        let mut sh = d.begin_shader_mode(&shader);
        sh.draw_texture(&target, 0, 0, Color::WHITE);
        }
        d.draw_fps(0, 0);
    }
}
