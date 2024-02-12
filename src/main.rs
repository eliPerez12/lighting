use lighting::*;
use player::*;
use rand::Rng;
use raylib::prelude::*;
use world::*;
use renderer::Renderer;

mod lighting;
mod player;
mod renderer;
mod world;

trait ImprovedCamera {
    fn to_screen(&self, world_pos: Vector2) -> Vector2;
    fn to_world(&self, screen_pos: Vector2) -> Vector2;
}

impl ImprovedCamera for Camera2D {
    fn to_screen(&self, world_pos: Vector2) -> Vector2 {
        (world_pos + self.offset) * self.zoom
    }
    fn to_world(&self, screen_pos: Vector2) -> Vector2 {
        (screen_pos / self.zoom) - self.offset
    }
}

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
    let mut day_cycle = DayCycle::new(&mut light_engine);
    let mut camera = Camera2D::default();
    let mut player = Player::new(&mut rl, &thread);
    player.pos += Vector2::new(500.0, 500.0);

    let cone = light_engine.spawn_light(Light::default_cone());
    let mut flashlight_on = true;

    let mut floor_map = vec![];

    for _ in 0..100 {
        let mut line = vec![];
        for _ in 0..100 {
            let tile = rand::thread_rng().gen_range(0..=6);
            line.push(tile);
        }
        floor_map.push(line);
    }

    camera.zoom = 3.5;

    while !rl.window_should_close() {
        /* ---- Update ---- */
        let screen_size = Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32);

        player.handle_movement(&rl);

        if rl.is_key_pressed(KeyboardKey::KEY_F) {
            flashlight_on = !flashlight_on;
        }

        camera.zoom *= 1.0 + rl.get_mouse_wheel_move() / 70.0;

        if rl.is_key_down(KeyboardKey::KEY_MINUS) {
            camera.zoom /= 1.04;
        }
        if rl.is_key_down(KeyboardKey::KEY_EQUAL) {
            camera.zoom *= 1.04;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
            camera.zoom = 1.0;
        }

        camera.offset = -player.pos + screen_size / 2.0 / camera.zoom;

        day_cycle.update(&mut rl);

        light_engine.handle_spawning_light(&mut rl, &camera);

        let player_screen_pos = camera.to_screen(player.pos);
        let mouse_pos = rl.get_mouse_position();
        let dx = mouse_pos.x - player_screen_pos.x;
        let dy = -(mouse_pos.y - player_screen_pos.y);
        let rotation = dy.atan2(dx) + PI as f32;

        light_engine.update_light(
            &cone,
            Light::Cone {
                pos: player.pos + Vector2::new(dx, -dy).normalized() * 5.0,
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

        light_engine.update_light(
            day_cycle.ambient_light_handle(),
            day_cycle.get_ambient_light(),
        );
        renderer.update_target(&mut rl, &thread, screen_size);

        /* ----- Draw ----- */
        let mut d = rl.begin_drawing(&thread);
        light_engine.update_shader_values(&mut renderer.shader, &camera, screen_size);

        // Drawing world
        renderer.draw_world(&mut d, &thread, &player, &camera, &floor_map);

        d.draw_fps(0, 0);
        day_cycle.draw_debug_info(&mut d);
    }
}