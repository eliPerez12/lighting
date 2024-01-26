use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init().width(630).height(480).resizable().build();

    let mut shader = rl.load_shader_from_memory(
        &thread,
        // Some(include_str!("lighting.vs")),
        None,
        Some(include_str!("lighting.fs")),
    );

    let sh_screen_size_loc = shader.get_shader_location("screenSize");

    while !rl.window_should_close() {
        /* ---- Update ---- */
        let (screen_width, screen_height) = (rl.get_screen_width(), rl.get_screen_height());
        let mut target = rl.load_render_texture(&thread, 640, 480).unwrap();

        /* ----- Draw ----- */
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // Update shaders
        shader.set_shader_value(
            sh_screen_size_loc,
            Vector2::new(screen_width as f32, screen_height as f32),
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
