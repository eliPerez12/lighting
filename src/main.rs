use debug::*;
use lighting::*;
use player::*;
use raylib::prelude::*;
use renderer::*;
use tile::*;
use world::World;
use world_map::*;

mod debug;
mod lighting;
mod player;
mod renderer;
mod tile;
mod world;
mod world_map;

#[derive(Clone)]
pub struct Line {
    pub start: Vector2,
    pub end: Vector2,
}

impl Line {
    // Function to calculate intersection point between two lines
    pub fn intersection(&self, other: &Line) -> Option<Vector2> {
        let denominator = (other.end.y - other.start.y) * (self.end.x - self.start.x)
            - (other.end.x - other.start.x) * (self.end.y - self.start.y);
        // Check if the lines are parallel
        if denominator.abs() < 0.0001 {
            return None;
        }

        let ua = ((other.end.x - other.start.x) * (self.start.y - other.start.y)
            - (other.end.y - other.start.y) * (self.start.x - other.start.x))
            / denominator;
        let ub = ((self.end.x - self.start.x) * (self.start.y - other.start.y)
            - (self.end.y - self.start.y) * (self.start.x - other.start.x))
            / denominator;

        // Check if the intersection point is within the line segments
        if (0.0..=1.0).contains(&ua) && (0.0..=1.0).contains(&ub) {
            let x = self.start.x + ua * (self.end.x - self.start.x);
            let y = self.start.y + ua * (self.end.y - self.start.y);
            println!("Intersect: x: {x}, y: {y}");
            Some(Vector2 { x, y })
        } else {
            None
        }
    }

    // Returns lines from a rectangle (Top, Bottom, Left, Right)
    pub fn from_rect(rect: &Rectangle) -> Vec<Line> {
        vec!(
            Line { // Top
                start: Vector2::new(rect.x, rect.y),
                end: Vector2::new(rect.x + rect.width, rect.y),
            },
            Line { // Bottom
                start: Vector2::new(rect.x, rect.y + rect.height),
                end: Vector2::new(rect.x + rect.width, rect.y + rect.height),
            },
            Line { // Left
                start: Vector2::new(rect.x, rect.y),
                end: Vector2::new(rect.x, rect.y + rect.height),
            },
            Line { // Right
                start: Vector2::new(rect.x + rect.width, rect.y),
                end: Vector2::new(rect.x + rect.width, rect.y + rect.height),
            },
        )
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .vsync()
        .width(1280)
        .height(720)
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

        player.handle_controls(&rl, &world.map);
        player.update_flashlight(&mut rl, &camera, &mut light_engine);
        light_engine
            .get_mut_light(&player.light)
            .set_pos(player.pos);

        if rl.is_key_pressed(KeyboardKey::KEY_G) {
            world.spawn_bullet(&rl, &camera, &player);
        }
        world.update_bullets(&rl);

        camera.handle_player_controls(&mut rl);
        camera.pan_to(&rl, player.pos, screen_size);

        world.day_cycle.update(&mut rl, &mut light_engine);
        debug_info.update(&mut rl);
        debug_info.add(format!("FPS: {}", rl.get_fps()));
        debug_info.add(format!("Frame time: {}", rl.get_frame_time()));
        debug_info.add(world.day_cycle.get_debug_info());
        light_engine.handle_spawning_light(&mut rl, &camera);

        renderer.update_target(&mut rl, &thread, screen_size);

        /* ----- Draw ----- */
        let mut d = rl.begin_drawing(&thread);
        light_engine.update_shader_values(&mut renderer.shader, &camera, screen_size);

        // Drawing world
        renderer.draw_world(&mut d, &thread, &player, &camera, &world, &debug_info);

        // Drawing UI
        debug_info.draw(&mut d);
    }
}
