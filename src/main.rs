use debug::*;
use lighting::*;
use player::*;
use raylib::prelude::*;
use renderer::*;
use tile::*;
use world_map::*;

mod debug;
mod lighting;
mod player;
mod renderer;
mod tile;
mod world_map;

pub struct Bullet {
    pub pos_history: [Vector2; 3],
    pub pos: Vector2,
    pub vel: Vector2,
}

impl Bullet {
    pub fn new(pos: Vector2, vel: Vector2) -> Bullet {
        Bullet {
            pos,
            vel,
            pos_history: [pos; 3],
        }
    }

    pub fn update_history(&mut self) {
        self.pos_history[2] = self.pos_history[1];
        self.pos_history[1] = self.pos_history[0];
        self.pos_history[0] = self.pos;
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        self.update_history();
        let drag = 35.0;

        self.vel -= self.vel / drag * rl.get_frame_time() * 60.0;
        if self.vel.length() <= 30.0 {
            self.vel = Vector2::zero();
        }

        self.pos += self.vel * rl.get_frame_time();
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .vsync()
        .width(1000)
        .height(700)
        .msaa_4x()
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

    let mut bullets = vec![];

    while !rl.window_should_close() {
        /* ---- Update ---- */
        let screen_size = Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32);

        player.handle_controls(&rl, &map);
        player.update_flashlight(&mut rl, &camera, &mut light_engine);
        light_engine
            .get_mut_light(&player.light)
            .set_pos(player.pos);

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let player_screen_pos = camera.to_screen(player.pos);
            let mouse_pos = rl.get_mouse_position();
            let angle_to_mouse =
                (mouse_pos.y - player_screen_pos.y).atan2(mouse_pos.x - player_screen_pos.x);
            let bullet_speed = 900.0;
            let bullet_vel = Vector2::new(
                angle_to_mouse.cos(),
                angle_to_mouse.sin(),
            );

            bullets.push(Bullet::new(
                player.pos + bullet_vel * 15.0,
                bullet_vel * bullet_speed,
            ));
        }

        for bullet in bullets.iter_mut() {
            bullet.update(&rl);
        }

        bullets.retain(|bullet| bullet.vel != Vector2::zero());
        dbg!(bullets.len());

        camera.handle_player_controls(&mut rl);
        camera.pan_to(&rl, player.pos, screen_size);

        day_cycle.update(&mut rl, &mut light_engine);
        debug_info.update(&mut rl);
        debug_info.add(format!("FPS: {}", rl.get_fps()));
        debug_info.add(format!("Frame time: {}", rl.get_frame_time()));
        debug_info.add(day_cycle.get_debug_info());
        light_engine.handle_spawning_light(&mut rl, &camera);

        renderer.update_target(&mut rl, &thread, screen_size);

        /* ----- Draw ----- */
        let mut d = rl.begin_drawing(&thread);
        light_engine.update_shader_values(&mut renderer.shader, &camera, screen_size);

        // Drawing world
        renderer.draw_world(
            &mut d,
            &thread,
            &player,
            &camera,
            &map,
            &bullets,
            &debug_info,
        );

        // Drawing UI
        debug_info.draw(&mut d);
    }
}
