use raylib::prelude::*;

// Stores debug info about the world
pub struct DebugInfo {
    pub info: Vec<String>,
    pub debug: bool,
}

impl DebugInfo {
    pub fn new() -> DebugInfo {
        DebugInfo {
            info: vec![],
            debug: true,
        }
    }
    pub fn update(&mut self, rl: &mut RaylibHandle) {
        self.info = vec![];
        if rl.is_key_pressed(KeyboardKey::KEY_F1) {
            self.debug = !self.debug;
        }
        self.info.push("(F1 to diable debug info)".to_string());
    }
    pub fn add(&mut self, info: String) {
        self.info.push(info)
    }
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        if self.debug {
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