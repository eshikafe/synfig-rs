use synfig_rs::loadcanvas::*;

fn main() {
let canvas = CanvasParser::new();
canvas.register_canvas_in_map(32, "c://Documents/synfig".to_string());
canvas.register_canvas_in_map(1, "c://Documents".to_string());
canvas.show_canvas_map("file".to_string(), 1, "text".to_string());
}