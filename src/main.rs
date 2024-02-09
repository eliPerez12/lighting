use lighting::*;
use player::*;
use rand::Rng;
use raylib::prelude::*;
use renderer::Renderer;

mod lighting;
mod player;
mod renderer;

struct DayCycle {
    time: f32,
    ambient_light_handle: LightHandle,
}

impl DayCycle {
    const FULL_CYCLE_LENGTH: f32 = 60.0;
    fn update(&mut self, rl: &mut RaylibHandle) {
        self.time += rl.get_frame_time();
        if self.time > Self::FULL_CYCLE_LENGTH {
            self.time -= Self::FULL_CYCLE_LENGTH;
        }
    }
    fn get_ambient_light(&self) -> Light {
        let normilized_time = self.time / Self::FULL_CYCLE_LENGTH;
        let sunrise_length = 1.0 / 14.0;

        //  Sun rising
        if self.time < Self::FULL_CYCLE_LENGTH * sunrise_length {
            let light_level = normilized_time / sunrise_length;
            Light::Ambient {
                color: Vector4::new(1.0, 1.0, 1.0, light_level),
            }
        } 
        // Sun setting
        else if self.time >= Self::FULL_CYCLE_LENGTH * sunrise_length * (1.0/sunrise_length-2.0)/2.0
        && normilized_time < 0.5 {
            dbg!("setting");
            let light_level = (1.0 - normilized_time / sunrise_length) + (1.0/sunrise_length-2.0)/2.0;
            Light::Ambient {
                color: Vector4::new(1.0, 1.0, 1.0, light_level),
            }
        }
        // Sun risen
        else if normilized_time < 0.5 {
            Light::Ambient {
                color: Color::WHITE.into(),
            }
        } 
        // Sun set
        else {
            Light::Ambient {
                color: Color::BLACK.into(),
            }
        }
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
    let mut day_cycle = DayCycle {
        time: 0.0,
        ambient_light_handle: light_engine.spawn_light(Light::default_ambient()),
    };
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

    let mut floor_map = vec![];


    for _ in 0..100 {
        let mut line = vec![];
        for _ in 0..100 {
            let tile = rand::thread_rng().gen_range(0..5);
            line.push(tile);
        }
        floor_map.push(line);
    }

    camera.zoom = 1.0;

    while !rl.window_should_close() {
        /* ---- Update ---- */
        let screen_size = Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32);

        player.handle_movement(&rl);

        if rl.is_key_pressed(KeyboardKey::KEY_F) {
            flashlight_on = !flashlight_on;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_MINUS) {
            camera.zoom /= 1.1;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_EQUAL) {
            camera.zoom *= 1.1;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
            camera.zoom = 1.0;
        }

        camera.offset = -player.pos + screen_size/2.0 / camera.zoom;

        day_cycle.update(&mut rl);

        light_engine.handle_spawning_light(&mut rl, &camera);

        let player_screen_pos = (player.pos + camera.offset) * camera.zoom;
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
            &day_cycle.ambient_light_handle,
            day_cycle.get_ambient_light(),
        );
        renderer.update_target(&mut rl, &thread, screen_size);

        /* ----- Draw ----- */
        let mut d = rl.begin_drawing(&thread);
        light_engine.update_shader_values(&mut renderer.shader, &camera, screen_size);

        // Drawing world
        renderer.draw_world(&mut d, &thread, &player, &camera, &floor_map);

        d.draw_fps(0, 0);
        let hour = (day_cycle.time / DayCycle::FULL_CYCLE_LENGTH * 24.0) as i32;
        let minute = (day_cycle.time / DayCycle::FULL_CYCLE_LENGTH * 24.0 * 60.0 % 60.0) as i32;
        d.draw_text(
            &format!(
                "Time: {}:{}{} {}",
                if hour % 12 == 0 { 12 } else { hour % 12 },
                if minute < 10 { "0" } else { "" },
                minute,
                if hour < 12 { "AM" } else { "PM" }
            ),
            0,
            0,
            50,
            Color::WHITE,
        );
        d.draw_text(
            &format!("Raw Time: {}", day_cycle.time / DayCycle::FULL_CYCLE_LENGTH),
            0,
            60,
            50,
            Color::WHITE,
        );
    }
}


#[test]
fn test() {
    let sunrise_length = 1.0/10.0;
    dbg!((1.0/sunrise_length-2.0)/2.0);
}