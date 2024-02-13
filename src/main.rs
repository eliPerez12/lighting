use lighting::*;
use player::*;
use raylib::prelude::*;
use renderer::Renderer;
use world::*;

mod lighting;
mod player;
mod renderer;
mod world;

struct DebugInfo {
    info: Vec<String>,
    drawing: bool,
}

impl DebugInfo {
    fn update(&mut self, rl: &mut RaylibHandle) {
        self.info = vec![];
        if rl.is_key_pressed(KeyboardKey::KEY_F1) {
            self.drawing = !self.drawing;
        }
        self.info.push("(F1 to diable debug info)".to_string());
    }
    fn add(&mut self, info: String) {
        self.info.push(info)
    }
    fn draw(&self, d: &mut RaylibDrawHandle) {
        if self.drawing {
            let font_size = 40;
            for (i, info) in self.info.iter().enumerate() {
                d.draw_text(
                    info,
                    font_size / 5,
                    i as i32 * font_size + 1 + font_size / 10,
                    font_size,
                    Color::WHITE,
                );
            }
        }
    }
}


fn load_map(path: &str) -> Vec<Vec<i64>> {
    use std::io::BufRead;

    let mut floor_map = vec![];
    let map = std::fs::File::open(path).unwrap();
    let mut reader = std::io::BufReader::new(map);
    // Skip first 6 lines of map data
    for _ in 0..6 {
        reader.read_line(&mut String::new()).unwrap();
    }
    // Parsing background layer
    for y in 0..20 {
        let mut floor_map_line = vec![];
        let mut buffer = String::new();
        reader.read_line(&mut buffer).unwrap();
        let buffer = &buffer.replace("\r\n", "");
        let line = buffer
            .split(',')
            .filter(|s| s != &"")
            .map(|s| 
            if let Ok(tile) = s.parse::<i64>() {
                tile - 1
            } else {
                dbg!(s, y, buffer);
                panic!("Unable to parse map");
            })
            .collect::<Vec<i64>>();
        (0..30).for_each(|x| {
            floor_map_line.push(line[x])
        });
        floor_map.push(floor_map_line);
    }
    floor_map
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
    let mut debug_info = DebugInfo {
        info: vec![],
        drawing: true,
    };
    player.pos += Vector2::new(500.0, 500.0);

    let cone = light_engine.spawn_light(Light::default_cone());
    let mut flashlight_on = true;

    let floor_map= load_map("assets/maps/map0.tmx");

    camera.zoom = 3.5;

    while !rl.window_should_close() {
        /* ---- Update ---- */
        let screen_size = Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32);

        player.handle_movement(&rl);
        camera.handle_player_controls(&mut rl);

        camera.offset = -player.pos + screen_size / 2.0 / camera.zoom;
        
        if rl.is_key_pressed(KeyboardKey::KEY_F) {
            flashlight_on = !flashlight_on;
        }

        day_cycle.update(&mut rl, &mut light_engine);
        debug_info.update(&mut rl);
        debug_info.add(format!("FPS: {}", rl.get_fps()));
        debug_info.add(format!("Frame time: {}", rl.get_frame_time()));
        debug_info.add(day_cycle.get_debug_info());
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

        renderer.update_target(&mut rl, &thread, screen_size);

        /* ----- Draw ----- */
        let mut d = rl.begin_drawing(&thread);
        light_engine.update_shader_values(&mut renderer.shader, &camera, screen_size);

        // Drawing world
        renderer.draw_world(&mut d, &thread, &player, &camera, &floor_map);

        // Drawing UI
        debug_info.draw(&mut d);
    }
}
