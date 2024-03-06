use day_cycle::DayCycle;
use debug::*;
use lighting::*;
use player::*;
use raylib::prelude::*;
use renderer::*;
use tile::*;
use world::World;
use world_map::*;

mod day_cycle;
mod debug;
mod lighting;
mod player;
mod renderer;
mod tile;
mod world;
mod world_map;

fn main() {
    let (mut rl, thread) = raylib::init()
        .vsync()
        .width(1600)
        .height(900)
        .msaa_4x()
        .title("Lighting")
        .resizable()
        .build();
    let mut renderer = Renderer::new(&mut rl, &thread);
    let mut light_engine = LightEngine::new(&mut renderer.shader);
    let mut camera = Camera2D::default();
    let mut player = Player::new(&mut rl, &thread, &mut light_engine);
    let mut debug_info = DebugInfo::new();
    let mut world = World::new(&mut light_engine);

    camera.zoom = 3.5;
    player.pos = Vector2::new(64.0, 64.0);
    camera.track(
        player.pos,
        Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32),
    );

    while !rl.window_should_close() {
        /* ---- Update ---- */
        let screen_size = Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32);

        if !rl.is_key_down(KeyboardKey::KEY_T) {
            player.handle_controls(&rl, &world.map);
            player.update_flashlight(&mut rl, &camera, &mut light_engine);

            // Ambient light
            light_engine
                .get_mut_light(&player.ambient_light)
                .set_pos(player.pos);

            if rl.is_key_pressed(KeyboardKey::KEY_G) {
                world.spawn_bullet(&rl, &camera, &player);
                for light in player.muzzle_lights.iter() {
                    light_engine
                        .get_mut_light(light)
                        .set_pos(
                            player.pos
                                + player.get_vector_to_screen_pos(rl.get_mouse_position(), &camera)
                                    * 15.0,
                        )
                        .set_color(Color::new(255, 212, 80, 255).into());
                }
            } else {
                for light in player.muzzle_lights.iter() {
                    let light = light_engine.get_mut_light(light).set_pos(
                        player.pos
                            + player.get_vector_to_screen_pos(rl.get_mouse_position(), &camera)
                                * 15.0,
                    );

                    let old_color = light.color();
                    light.set_color(Vector4::new(
                        old_color.x,
                        old_color.y,
                        old_color.w,
                        (old_color.z - (25.0 * rl.get_frame_time())).max(0.0),
                    ));
                }
            }

            world.update_bullets(&rl);

            camera.handle_player_controls(&mut rl);
            camera.pan_to(&rl, player.pos, screen_size);

            world.day_cycle.update(&mut rl, &mut light_engine);
            debug_info.update(&mut rl);
            debug_info.add(format!("FPS: {}", rl.get_fps()));
            debug_info.add(format!("Frame time: {}", rl.get_frame_time()));
            debug_info.add(world.day_cycle.get_debug_info());
            debug_info.add(format!(
                "Norm Time: {}",
                world.day_cycle.time / DayCycle::FULL_CYCLE_LENGTH
            ));
            light_engine.handle_spawning_light(&mut rl, &camera);

            renderer.update_target(&mut rl, &thread, screen_size);
        }
        /* ----- Draw ----- */
        let mut d = rl.begin_drawing(&thread);
        light_engine.update_shader_values(&mut renderer.shader, &camera, screen_size);

        // Drawing world
        renderer.draw_world(&mut d, &thread, &player, &camera, &world, &debug_info);

        // Drawing UI
        debug_info.draw(&mut d);
    }
}
