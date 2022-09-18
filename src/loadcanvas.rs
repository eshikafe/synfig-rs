#![allow(dead_code)]
#![allow(unused_variables)]

use crate::lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

type OpenCanvasMap = HashMap<i32, String>;

lazy_static! {
    static ref OPEN_CANVAS_MAP: Mutex<OpenCanvasMap> = Mutex::new({
        let open_canvas_map = HashMap::new();
        open_canvas_map
    });
}

#[derive(Default)]
pub struct CanvasParser {
    max_warnings: i32,
    total_warnings: i32,
    total_errors: i32,
    allow_errors: bool,
    filename: String,
    path: String,
    errors_text: String,
    warnings_text: String,
    guid: String, // GUID
    in_bones_section: bool,
}

impl CanvasParser {
    // Constructor
    pub fn new() -> Self {
        let mut instance: Self = Default::default();
        instance.max_warnings = 1000;
        instance
    }

    // public:
    // Member functions
    pub fn set_allow_errors(&mut self, value: bool) {
        self.allow_errors = value;
    }

    pub fn set_max_warnings(&mut self, value: i32) {
        self.max_warnings = value;
    }

    pub fn get_max_warnings(&self) -> i32 {
        self.max_warnings
    }

    pub fn error_count(&self) -> i32 {
        self.total_errors
    }

    pub fn warning_count(&self) -> i32 {
        self.total_warnings
    }

    pub fn set_path(&mut self, value: String) {
        self.path = value;
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn get_errors_text(&self) -> String {
        self.errors_text.clone()
    }

    pub fn get_warnings_text(&self) -> String {
        self.warnings_text.clone()
    }

    // Register a canvas in the canvas map
    // canvas - the handle to the canvas to register
    // abs_path - the absolute path to the file that represents the canvas
    pub fn register_canvas_in_map(&self, canvas: i32, abs_path: String) {
        OPEN_CANVAS_MAP.lock().unwrap().insert(canvas, abs_path);
    }

    pub fn show_canvas_map(&self, file: String, line: i32, text: String) {
        println!("  .-----\n  |  {}:{} {}", file, line, text);
        for (key, val) in OPEN_CANVAS_MAP.lock().unwrap().iter() {
            println!("  |    {:>40} : {} ({})\n", val, key, "");
        }
        println!("  `-----\n");
    }

    // Parse a canvas from a file with absolute path
    pub fn parse_from_file_as(&self, identifier: i32, abs_path: String, errors: String) -> i32 {
        0
    }

    // Parse a Canvas from a xmlpp root node
    // pub fn parse_as(&self, node: xmlpp::Element, errors: String) -> i32 {}
}
