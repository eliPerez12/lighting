use lighting::*;
use player::*;
use renderer::Renderer;
use raylib::prelude::*;

mod lighting;
mod player;
mod renderer;


fn main() {
    let (mut rl, thread) = raylib::init()
        .vsync()
        .width(1000)
        .height(700)
        .title("Lighting")
        .resizable()
        .build();
    let mut renderer = Renderer::new(&mut rl, &thread);
    let mut light_engine = LightEngine::new(&mut renderer.shader);
    let ambient_light = light_engine.spawn_light(AMBIENT_LIGHT_SUNRISE);
    let mut camera = Camera2D::default();
    let mouse_light = light_engine.spawn_light(Light::Radial {
        pos: Vector2::zero(),
        color: Color::WHITE.into(),
        radius: 150.0,
    });
    let mut player = Player::new(&mut rl, &thread);

    while !rl.window_should_close() {
        /* ---- Update ---- */
        let screen_size = Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32);
        light_engine.handle_spawning_light(&mut rl, &camera, &ambient_light);
        light_engine.handle_mouse_light(&mut rl, &mouse_light, &camera);

        player.handle_movement(&rl);
        camera.offset = -player.pos + screen_size / 2.0;

        renderer.update_target(&mut rl, &thread, screen_size);
        
        /* ----- Draw ----- */
        let mut d = rl.begin_drawing(&thread);
        light_engine.update_shader_values(&mut renderer.shader, &camera, screen_size);

        // Drawing world
        renderer.draw_world(&mut d, &thread, &player, &camera);

        d.draw_fps(0, 0);
    }
}
