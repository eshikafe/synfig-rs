#![allow(dead_code)]
#![allow(unused_variables)]

// use crate::lazy_static::lazy_static;
use guid_create::GUID;
use log::{error, warn};
use std::collections::HashMap;
use std::sync::Mutex;

use lazy_static::lazy_static;
use crate::canvas;
use crate::color::*;
use crate::valuenode;
use crate::time::Time;
use crate::vector::*;
use crate::segment::Segment;
use crate::layer::*;

// Returns the Open Canvases Map.
// using OpenCanvasMap = std::map<Canvas::LooseHandle, std::string>;
type OpenCanvasMap = HashMap<i32, String>;

lazy_static! {
    static ref OPEN_CANVAS_MAP: Mutex<OpenCanvasMap> = Mutex::new({
        let open_canvas_map = HashMap::new();
        open_canvas_map
    });
}

// TODO: Replace with a quick_xml crate
mod xmlpp {
    pub type Node = i32;

    #[derive(Debug)]
    pub struct Element;
}

/// CanvasParser converts xml elements from a sif file to Synfig objects
pub struct CanvasParser {
    max_warnings: i32,
    total_warnings: i32,
    total_errors: i32,
    allow_errors: bool,
    filename: String,
    path: String,
    errors_text: String,
    warnings_text: String,
    guid: GUID,
    in_bones_section: bool,
    // Set of absolute file names of the canvases currently being parsed
    pub loading: Vec<i32>,
}

impl Default for CanvasParser {
    fn default() -> Self {
        Self {
            max_warnings: 1000,
            total_warnings: 0,
            total_errors: 0,
            allow_errors: false,
            filename: String::from(""),
            path: String::from(""),
            errors_text: String::from(""),
            warnings_text: String::from(""),
            guid: GUID::rand(),
            in_bones_section: false,
            loading: vec![0],
        }
    }
}

impl CanvasParser {
    pub fn new() -> Self {
        Default::default()
    }

    // Sets allow errors variable
    pub fn set_allow_errors(&mut self, value: bool) {
        self.allow_errors = value;
    }

    // Sets the maximum number of warnings before a fatal error is thrown
    pub fn set_max_warnings(&mut self, value: i32) {
        self.max_warnings = value;
    }

    // Returns the maximum number of warnings before a fatal_error is thrown
    pub fn get_max_warnings(&self) -> i32 {
        self.max_warnings
    }

    // Returns the number of errors in the last parse
    pub fn error_count(&self) -> i32 {
        self.total_errors
    }

    // Returns the number of warnings in the last parse
    pub fn warning_count(&self) -> i32 {
        self.total_warnings
    }

    // Sets the path of the file to parse
    pub fn set_path(&mut self, value: String) {
        self.path = value;
    }

    // Gets the path of the file to parse
    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    // Gets error text string
    pub fn get_errors_text(&self) -> String {
        self.errors_text.clone()
    }

    // Gets warning text string
    pub fn get_warnings_text(&self) -> String {
        self.warnings_text.clone()
    }

    // Register a canvas in the canvas map
    // `canvas` is the handle to the canvas to register
    // `abs_path` is the absolute path to the file that represents the canvas
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
    // Returns canvas::Handle
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
    pub fn error(&mut self, element: xmlpp::Node, text: String) {
        let err = format!(
            "{}:<{:?}>:{:?}: error: {}",
            self.filename, element, element, text
        );
        self.total_errors += 1;
        if self.allow_errors {
            error!("{}", err);
        }
    }

    // Fatal Error handling function
    fn fatal_error(&self, element: i32, text: String) {
        let err = format!(
            "{}:<{}>:{}: error: {}",
            self.filename, element, element, text
        );
        error!("{}", err);
    }

    // Warning handling function
    pub fn warning(&mut self, element: i32, text: String) {
        let msg = format!("{}:<{}>:{}: {}", self.filename, element, element, text);
        warn!("{}", msg);

        self.total_warnings += 1;
        self.warnings_text = format!("{} * {} \n", self.warnings_text, msg);
        if self.total_warnings >= self.max_warnings {
            self.fatal_error(element, "Too many warnings".to_string());
        }
    }

    // Unexpected element error handling function
    fn error_unexpected_element(&mut self, node: xmlpp::Node, got: String, expected: String) {}

    // Canvas Parsing Function
    fn parse_canvas(
        &mut self,
        node: &xmlpp::Element,
        parent: canvas::Handle,
        inline_: bool,
        //identifier: &FileSystem::Identifier,
        path: String,
    ) -> canvas::Handle {
        // parent = 0;
        // inline_ = false;
        // identifier = = FileSystemNative::instance()->get_identifier(std::string());
        // path = String::from(".");
        0 
    }

    // Canvas definitions Parsing Function (exported value nodes and exported canvases)
    fn parse_canvas_defs(&mut self, node: &xmlpp::Element, canvas: canvas::Handle) {}

    fn parse_canvas_bones(
        &mut self,
        node: &xmlpp::Element,
        canvas: canvas::Handle,
    ) -> Vec<valuenode::Handle> {
        vec![0]
    }

    // Layer Parsing Function
    // todo -> etl::handle<Layer>
    fn parse_layer(&mut self, node: &xmlpp::Element, canvas: canvas::Handle) -> Vec<Layer> {
        vec![Layer{active: false, optimized: false, exclude_from_rendering: false,
            description: String::from(""),
            group: String::from(""),
        }]
    }

    // Generic Value Base Parsing Function
    fn parse_value(&mut self, node: &xmlpp::Element, canvas: canvas::Handle) -> i32 /* ValueBase */ {
        0
    }

    // Generic Value Node Parsing Function
    fn parse_value_node(
        &mut self,
        node: &xmlpp::Element,
        canvas: canvas::Handle
    ) -> valuenode::Handle {
        0
    }

    // Real Value Base Parsing Function
    fn parse_real(&mut self, node: &xmlpp::Element) -> f64 {
        0.0
    }

    // Time Value Base Parsing Function
    fn parse_time(&mut self, node: &xmlpp::Element, canvas: canvas::Handle) -> Time {
        Time::new()
    }

    // Integer Value Base Parsing Function
    fn parse_integer(&mut self, node: &xmlpp::Element) -> i32 {
        0
    }

    // Vector Value Base Parsing Function
    fn parse_vector(&mut self, node: &xmlpp::Element) -> Vector {
        Vector{x: 0.0, y:0.0}
    }

    // Color Value Base Parsing Function
    fn parse_color(&mut self, node: &xmlpp::Element) -> Color {
        Color {
            r: 0.1,
            g: 2.4,
            b: 22.0,
            a: 1.0,
        }
    }

    // Angle Value Base Parsing Function
    fn parse_angle(&mut self, node: &xmlpp::Element) -> i32 /* Angle */ {
        0
    }

    // String Value Base Parsing Function
    fn parse_string(&mut self, node: &xmlpp::Element) -> String {
        String::from("todo")
    }

    // Bool Value Base Parsing Function
    fn parse_bool(&mut self, node: &xmlpp::Element) -> bool {
        false
    }

    // Segment Value Base Parsing Function
    fn parse_segment(&mut self, node: &xmlpp::Element) -> i32 /* Segment */ {
        0
    }

    // List Value Base Parsing Function
    fn parse_list(&mut self, node: &xmlpp::Element, canvas: canvas::Handle) -> i32 /* ValueBase */ {
        0
    }

    // Weighted Value Base Parsing Function
    fn parse_weighted_value(
        &mut self,
        node: &xmlpp::Element,
        // _type: &types_namespace::TypeWeightedValueBase,
        canvas: canvas::Handle,
    ) -> i32 /* ValueBase */ {
        0
    }

    // Pair Value Base Parsing Function
    fn parse_pair(
        &mut self,
        node: &xmlpp::Element,
       // _type: &types_namespace::TypePairBase,
        canvas: canvas::Handle,
    ) -> i32 /* ValueBase */ {
        0
    }

    // Gradient Value Base Parsing Function
    fn parse_gradient(&mut self, node: &xmlpp::Element) -> i32 /* Gradient */ {
        0
    }

    // Bline Point Value Base Parsing Function
    fn parse_bline_point(&mut self, node: &xmlpp::Element) -> i32 /* BLinePoint */ {
        0
    }

    // Transformation Value Base Parsing Function
    fn parse_transformation(&mut self, node: &xmlpp::Element) -> i32 /* Transformation */ {
        0
    }

    fn parse_guid(&mut self, node: &xmlpp::Element) -> GUID {
        self.guid
    }

    /// Width Point Value Base Parsing Function
    fn parse_width_point(&mut self, node: &xmlpp::Element) -> i32 /* WidthPoint */ {
        0
    }

    // Dash Item Value Base Parsing Function
    fn parse_dash_item(&mut self, node: &xmlpp::Element) -> i32 /* DashItem */ {
        0
    }

    // Keyframe Parsing Function
    fn parse_keyframe(&mut self, node: &xmlpp::Element, canvas: canvas::Handle) -> i32 /* Keyframe */ {
        0
    }

    // ValueNode Animated Parsing Function
    fn parse_animated(
        &mut self,
        node: &xmlpp::Element,
        canvas: canvas::Handle,
    ) -> i32 /* etl::handle<ValueNode_Animated> */ {
        0
    }

    // Linkable ValueNode Parsing Function
    fn parse_linkable_value_node(
        &mut self,
        node: &xmlpp::Element,
        canvas: canvas::Handle,
    ) -> i32 /*etl::handle<LinkableValueNode>*/ {
        0
    }

    // Static List Parsnig Function
    fn parse_static_list(
        &mut self,
        node: &xmlpp::Element,
        canvas: canvas::Handle,
    ) -> i32 /* etl::handle<ValueNode_StaticList> */ {
        0
    }

    // Dynamic List Parsnig Function
    fn parse_dynamic_list(
        &mut self,
        node: &xmlpp::Element,
        canvas: canvas::Handle,
    ) -> i32 /* etl::handle<ValueNode_DynamicList> */ {
        0
    }

    // Interpolation option for ValueBase parsing function
    fn parse_interpolation(&mut self, node: &xmlpp::Element, attribute: String) -> i32 /* Interpolation */ {
        0
    }

    // Static option for ValueBase parsing function
    fn parse_static(&mut self, node: &xmlpp::Element) -> bool {
        false
    }
}
