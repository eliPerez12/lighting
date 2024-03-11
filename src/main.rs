use bullet::Bullet;
use day_cycle::DayCycle;
use debug::*;
use lighting::*;
use player::*;
use rand::Rng;
use raylib::prelude::*;
use renderer::*;
use tile::*;
use world::World;
use world_map::*;

mod bullet;
mod day_cycle;
mod debug;
mod items;
mod lighting;
mod player;
mod renderer;
mod tile;
mod world;
mod world_map;

pub fn explode(rl: &RaylibHandle, world: &mut World, camera: &Camera2D) {
    let num_shrapnel = 15;
    let num_random_shrapnel = 10;
    let shrapnel_speed = 600.0;
    let shrapnel_speed_margin = 0.3;
    for i in 0..num_shrapnel {
        let pos = camera.to_world(rl.get_mouse_position());
        let angle = 2.0 * PI as f32 * (i as f32 / num_shrapnel as f32);
        let vel = Vector2::new(angle.cos(), angle.sin()) * shrapnel_speed;
        let random_vel =
            rand::thread_rng().gen_range(1.0 - shrapnel_speed_margin..1.0 + shrapnel_speed_margin);
        world.bullets.push(Bullet::new(pos, vel * random_vel));
    }
    for _ in 0..num_random_shrapnel {
        let pos = camera.to_world(rl.get_mouse_position());
        let angle = rand::thread_rng().gen_range(0.0..2.0 * PI as f32);
        let random_vel =
            rand::thread_rng().gen_range(1.0 - shrapnel_speed_margin..1.0 + shrapnel_speed_margin);
        let vel = Vector2::new(angle.cos(), angle.sin()) * shrapnel_speed;
        world.bullets.push(Bullet::new(pos, vel * random_vel));
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .vsync()
        .size(1600, 900)
        .msaa_4x()
        .title("TDS GAME")
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

        // Only update if player inst freezing time
        if !rl.is_key_down(KeyboardKey::KEY_T) {
            player.handle_controls(&rl, &world.map);
            player.update_flashlight(&mut rl, &camera, &mut light_engine);
            player.handle_shooting(&mut light_engine, &rl, &mut world, &camera);
            world.update_bullets(&rl);

            if rl.is_key_pressed(KeyboardKey::KEY_G) {
                explode(&rl, &mut world, &camera);
            }

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
            debug_info.add(format!("Bullets in mag: {}", player.gun.mag.bullets));
            debug_info.add(format!(
                "Spawned lights {}/400",
                light_engine.spawned_lights()
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
