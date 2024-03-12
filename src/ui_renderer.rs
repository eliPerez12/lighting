use crate::DebugInfo;
use raylib::prelude::*;

pub struct UIRenderer;

impl UIRenderer {
    pub fn render_ui(d: &mut RaylibDrawHandle, debug_info: &DebugInfo) {
        debug_info.draw(d);
    }
}
