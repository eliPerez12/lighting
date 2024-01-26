use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init().width(630).height(480).resizable().build();

    let mut shader = rl.load_shader_from_memory(
        &thread,
        // Some(include_str!("lighting.vs")),
        None,
        Some(include_str!("lighting.fs")),
    );
    let mut target = rl.load_render_texture(&thread, 640, 480).unwrap();
    let sh_screen_size_loc = shader.get_shader_location("screenSize");

    while !rl.window_should_close() {
        let (screen_width, screen_height) = (rl.get_screen_width(), rl.get_screen_height());
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // Update shaders
        shader.set_shader_value(sh_screen_size_loc, Vector2::new(screen_width as f32, screen_height as f32));
        
        // Draw

        {
            let mut tg = d.begin_texture_mode(&thread, &mut target);
            tg.draw_circle(50, 50, 20.0, Color::BLUE);
        }

        let mut sh = d.begin_shader_mode(&shader);
        
        sh.draw_rectangle(0, 0, screen_width, screen_height, Color::BLUE);
        
    }
}
