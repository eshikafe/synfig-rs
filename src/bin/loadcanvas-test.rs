use synfig::loadcanvas::*;

fn main() {

// Test canvas data structure
let mut canvas = CanvasParser::new();

// Test canvas methods
canvas.register_canvas_in_map(32, "c://Documents/synfig".to_string());
canvas.register_canvas_in_map(1, "c://Documents".to_string());
canvas.show_canvas_map("file".to_string(), 1, "text".to_string());

// Test log functions
// canvas.set_allow_errors(true);
canvas.error(1234, "canvas layer missing".to_string());
}