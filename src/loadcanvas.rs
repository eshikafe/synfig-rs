// Reference files:
// https://github.com/synfig/synfig/blob/master/synfig-core/src/synfig/loadcanvas.cpp
// https://github.com/synfig/synfig/blob/master/synfig-core/src/synfig/loadcanvas.h

#![allow(dead_code)]
#![allow(unused_variables)]

use crate::lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use log::{error, warn};

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

    // Set of absolute file names of the canvases currently being parsed
    pub loading: Vec<i32>,
}

impl CanvasParser {
    // Constructor
    pub fn new() -> Self {
        env_logger::init();
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
        // TODO: Implement this function
        0
    }

    // Parse a Canvas from a xmlpp root node
    pub fn parse_as(&self, node: i32, errors: String) -> i32 {
       // TODO: Implement this function 
        0
    }

    // Error handling function
    // TODO: Use type xmlpp::Node for element
    pub fn error(&mut self, element: i32, text: String) { 
        let err = format!("{}:<{}>:{}: error: {}", self.filename, element,element, text);
        self.total_errors += 1;
        if self.allow_errors {
            error!("{}",err);
        }
    }

    fn fatal_error(&self, element: i32, text: String) {
        let err = format!("{}:<{}>:{}: error: {}", self.filename, element,element, text);
        error!("{}",err);
    }

    pub fn warning(&mut self, element: i32, text: String) {
        let msg = format!("{}:<{}>:{}: {}",self.filename,element,element,text);
        warn!("{}", msg);

        self.total_warnings += 1;
        self.warnings_text = format!("{} * {} \n", self.warnings_text, msg);
        if self.total_warnings >= self.max_warnings {
            self.fatal_error(element, "Too many warnings".to_string());
        }
    }

}
