
use synfig_core::{loadcanvas::*, version::*};

#[test]
fn load_canvas_methods() {
    env_logger::init();
    // Test canvas data structure
    let mut canvas = CanvasParser::new();

    // Test canvas methods
    canvas.register_canvas_in_map(32, "c://Documents/synfig".to_string());
    canvas.register_canvas_in_map(1, "c://Documents".to_string());
    canvas.show_canvas_map("file".to_string(), 1, "text".to_string());

    assert_eq!(0, canvas.error_count());
    // Test log functions
    canvas.set_allow_errors(true);
    canvas.error(1234, "canvas layer missing".to_string());
    assert_eq!(1, canvas.error_count());

    canvas.warning(98, "attribute unknown".to_string());
}

#[test]
fn version_test() {
    assert_eq!(true, synfig_check_version());

    println!("Synfig version: {}",get_version());
    println!("Synfig build info: {}", get_build_date());
}