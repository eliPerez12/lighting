use lighting::*;
use player::*;
use raylib::prelude::*;
use renderer::Renderer;

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
    let _smouse_light = light_engine.spawn_light(Light::Radial {
        pos: Vector2::zero(),
        color: Color::WHITE.into(),
        radius: 150.0,
    });
    let mut player = Player::new(&mut rl, &thread);
    player.pos += Vector2::new(500.0, 500.0);

    let cone = light_engine.spawn_light(Light::default_cone());
    let mut flashlight_on = true;

    let map = vec![
        vec![0, 1, 2, 3, 4],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
    ];

    while !rl.window_should_close() {
        /* ---- Update ---- */
        let screen_size = Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32);

        player.handle_movement(&rl);
        camera.offset = -player.pos + screen_size / 2.0;

        if rl.is_key_pressed(KeyboardKey::KEY_F) {
            flashlight_on = !flashlight_on;
        }

        light_engine.handle_spawning_light(&mut rl, &camera, &ambient_light);
        //light_engine.handle_mouse_light(&mut rl, &mouse_light, &camera);

        let player_screen_pos = player.pos + camera.offset;
        let mouse_pos = rl.get_mouse_position();

        camera.zoom = 0.5;

        let dx = mouse_pos.x - player_screen_pos.x;
        let dy = -(mouse_pos.y - player_screen_pos.y);

        let rotation = dy.atan2(dx) + PI as f32;

        light_engine.update_light(
            &cone,
            Light::Cone {
                pos: player.pos + Vector2::new(dx, -dy).normalized() * 21.0,
                color: if flashlight_on {
                    Color::WHEAT.into()
                } else {
                    Color::BLACK.into()
                },
                radius: 550.0,
                angle: PI as f32 / 2.0,
                rotation,
            },
        );
        renderer.update_target(&mut rl, &thread, screen_size);

        /* ----- Draw ----- */
        let mut d = rl.begin_drawing(&thread);
        light_engine.update_shader_values(&mut renderer.shader, &camera, screen_size);

        // Drawing world
        renderer.draw_world(&mut d, &thread, &player, &camera, &map);

        d.draw_fps(0, 0);
    }
}
