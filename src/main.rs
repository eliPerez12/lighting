use raylib::prelude::*;


fn main() {
    let (mut rl, thread) = raylib::init().width(630).height(480).resizable().build();
    let mut shader = rl.load_shader_from_memory(
        &thread,
        None,
        Some(include_str!("lighting.fs")),
    );

    let sh_screen_size_loc = shader.get_shader_location("screenSize");
    let mut target = rl.load_render_texture(&thread, rl.get_screen_width() as u32, rl.get_screen_height() as u32).unwrap();

    while !rl.window_should_close() {
        /* ---- Update ---- */
        let screen_size = Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32);


        /* ----- Draw ----- */
        if rl.is_window_resized() {
            target = rl.load_render_texture(&thread, rl.get_screen_width() as u32, rl.get_screen_height() as u32).unwrap();
        }
        let mut d = rl.begin_drawing(&thread);


        d.clear_background(Color::BLACK);

        // Update shader with screen size
        shader.set_shader_value(
            sh_screen_size_loc,
            screen_size,
        );

        // Drawing to target
        {
            let mut tg = d.begin_texture_mode(&thread, &mut target);
            tg.clear_background(Color::BLACK);
            tg.draw_circle(50, 50, 20.0, Color::BLUE);
        }

        // Render target with shader
        let mut sh = d.begin_shader_mode(&shader);
        sh.draw_texture(&target, 0, 0, Color::WHITE);
    }
}
