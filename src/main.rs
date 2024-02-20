use debug::*;
use lighting::*;
use player::*;
use raylib::prelude::*;
use renderer::*;
use world::*;

mod debug;
mod lighting;
mod player;
mod renderer;
mod world;

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
    let mut player = Player::new(&mut rl, &thread, &mut light_engine);
    let mut debug_info = DebugInfo::new();
    let map = WorldMap::load_from_file("assets/maps/map0.tmx", 30, 20);

    camera.zoom = 3.5;
    player.pos = Vector2::new(64.0, 64.0);

    while !rl.window_should_close() {
        /* ---- Update ---- */
        let screen_size = Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32);

        player.handle_movement(&rl);
        player.handle_flashlight(&mut rl, &camera, &mut light_engine);
        camera.handle_player_controls(&mut rl);
        camera.track(player.pos, screen_size);

        day_cycle.update(&mut rl, &mut light_engine);
        debug_info.update(&mut rl);
        debug_info.add(format!("FPS: {}", rl.get_fps()));
        debug_info.add(format!("Frame time: {}", rl.get_frame_time()));
        debug_info.add(day_cycle.get_debug_info());
        debug_info.add(format!("Player Pos: {:?}", player.pos));
        debug_info.add(format!(
            "Camera Offset: {:?}",
            camera.get_world_pos(screen_size)
        ));
        debug_info.add(format!("Camera Zoom: {:?}", camera.zoom));
        light_engine.handle_spawning_light(&mut rl, &camera);

        renderer.update_target(&mut rl, &thread, screen_size);

        /* ----- Draw ----- */
        let mut d = rl.begin_drawing(&thread);
        light_engine.update_shader_values(&mut renderer.shader, &camera, screen_size);

        // Drawing world
        renderer.draw_world(&mut d, &thread, &player, &camera, &map, &debug_info);

        // Drawing UI
        debug_info.draw(&mut d);
    }
}
