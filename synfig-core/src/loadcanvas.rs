#![allow(dead_code)]
#![allow(unused_variables)]

// use crate::lazy_static::lazy_static;
use guid_create::GUID;
use log::{error, warn};
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Mutex;

use crate::canvas::*;
use crate::color::*;
use crate::layer::*;
use crate::segment::Segment;
use crate::time::Time;
use crate::valuenode;
use crate::vector::*;
use crate::filesystem;
use lazy_static::lazy_static;

// Loads a canvas from current xmlpp Element
// returns the Canvas's handle on success, an empty handle on failure
pub fn open_canvas(node: &xmlpp::Element, errors: String, warnings: String) -> Box<Canvas> {
    unimplemented!()
}

// Loads a canvas from a filename and its absolute path
// returns the Canvas's handle on success, an None on failure
pub fn open_canvas_as(
    identifier: filesystem::Identifier,
    as_: String,
    errors: String,
    warnings: String,
) -> Option<Box<Canvas>> {
    let filename = filesystem::fix_slashes(as_);

	if CanvasParser::loading.contains(&identifier)
	{
		let warning = format!("cannot load '{}' recursively", identifier.filename.as_str());
		warn!("{}", warning);
		let warnings = format!("  * {}\n", warning);
        let mut canvas = Canvas::create();
		canvas.set_identifier(identifier);
		canvas.set_file_name(filename);
	// 	Layer::Handle paste(Layer_Group::create());
	// 	canvas->push_back(paste);
	// 	paste->set_description(warning);
		return Some(canvas);
	}

    let mut canvas: Box<Canvas>;
    let mut parser = CanvasParser::new();

    Some(canvas)
}

// Returns the Open Canvases Map.
type OpenCanvasMap = HashMap<i32, String>;
lazy_static! {
    static ref GET_OPEN_CANVAS_MAP: Mutex<OpenCanvasMap> = Mutex::new({
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
    // static std::set<FileSystem::Identifier> loading_;
    // use std::collections::HashSet;
    pub loading: HashSet<i32>,
}

impl Default for CanvasParser {
    fn default() -> Self {
        Self {
            max_warnings: 1000,
            total_warnings: 0,
            total_errors: 0,
            allow_errors: false,
            ..Default::default()
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
        GET_OPEN_CANVAS_MAP.lock().unwrap().insert(canvas, abs_path);
    }

    pub fn show_canvas_map(&self, file: String, line: i32, text: String) {
        println!("  .-----\n  |  {}:{} {}", file, line, text);
        for (key, val) in GET_OPEN_CANVAS_MAP.lock().unwrap().iter() {
            println!("  |    {:>40} : {} ({})\n", val, key, "");
        }
        println!("  `-----\n");
    }

    // Parse a canvas from a file with absolute path
    // Returns canvas::Handle => Result<Box<Canvas>, String>
    pub fn parse_from_file_as(&mut self, identifier: i32, abs_path: String, errors:&mut String) -> i32 {
        // TODO: Implement this function
        unimplemented!()
    }

    // Parse a Canvas from a xmlpp root node
    pub fn parse_as(&self, node: i32, errors: String) -> i32 {
        // TODO: Implement this function
        unimplemented!()
    }

    // Error handling function
    pub fn error(&mut self, element: xmlpp::Node, text: &str) {
        let err_str = format!(
            "{}:<{:?}>:{:?}: error: {}",
            self.filename, element, element, text
        );
        self.total_errors += 1;
        self.errors_text = format!("{} * {}\n", self.errors_text, err_str);
        if self.allow_errors {
            error!("{}", err_str);
        }
    }

    // Fatal Error handling function
    fn fatal_error(&self, element: i32, text: &str) {
        let err_str = format!(
            "{}:<{}>:{}: error: {}",
            self.filename, element, element, text
        );
        error!("{}", err_str);
    }

    // Warning handling function
    pub fn warning(&mut self, element: i32, text: &str) {
        let warn_str = format!("{}:<{}>:{}: {}", self.filename, element, element, text);
        warn!("{}", warn_str);
        self.total_warnings += 1;
        self.warnings_text = format!("{} * {}\n", self.warnings_text, warn_str);
        if self.total_warnings >= self.max_warnings {
            self.fatal_error(element, "Too many warnings");
        }
    }

    // Unexpected element error handling function
    fn error_unexpected_element(&mut self, node: xmlpp::Node, got: String, expected: String) {}

    // Canvas Parsing Function
    fn parse_canvas(
        &mut self,
        node: &xmlpp::Element,
        parent: Box<Canvas>,
        inline_: bool,
        //identifier: &FileSystem::Identifier,
        path: String,
    ) -> Result<Box<Canvas>, String> {
        // parent = 0;
        // inline_ = false;
        // identifier = = FileSystemNative::instance()->get_identifier(std::string());
        // path = String::from(".");
        unimplemented!()
    }

    // Canvas definitions Parsing Function (exported value nodes and exported canvases)
    fn parse_canvas_defs(&mut self, node: &xmlpp::Element, canvas: Box<Canvas>) {
        unimplemented!();
    }

    fn parse_canvas_bones(
        &mut self,
        node: &xmlpp::Element,
        canvas: Box<Canvas>,
    ) -> Vec<valuenode::Handle> {
        unimplemented!()
    }

    // Layer Parsing Function
    // todo -> etl::handle<Layer>
    fn parse_layer(&mut self, node: &xmlpp::Element, canvas: Box<Canvas>) -> Vec<Layer> {
        vec![Layer {
            active: false,
            optimized: false,
            exclude_from_rendering: false,
            description: String::from(""),
            group: String::from(""),
        }]
    }

    // Generic Value Base Parsing Function
    fn parse_value(&mut self, node: &xmlpp::Element, canvas: Box<Canvas>) -> i32 /* ValueBase */
    {
        unimplemented!()
    }

    // Generic Value Node Parsing Function
    fn parse_value_node(
        &mut self,
        node: &xmlpp::Element,
        canvas: Box<Canvas>,
    ) -> valuenode::Handle {
        unimplemented!()
    }

    // Real Value Base Parsing Function
    fn parse_real(&mut self, node: &xmlpp::Element) -> f64 {
        0.0
    }

    // Time Value Base Parsing Function
    fn parse_time(&mut self, node: &xmlpp::Element, canvas: Box<Canvas>) -> Time {
        Time::new()
    }

    // Integer Value Base Parsing Function
    fn parse_integer(&mut self, node: &xmlpp::Element) -> i32 {
        0
    }

    // Vector Value Base Parsing Function
    fn parse_vector(&mut self, node: &xmlpp::Element) -> Vector {
        Vector { x: 0.0, y: 0.0 }
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
    fn parse_list(&mut self, node: &xmlpp::Element, canvas: Box<Canvas>) -> i32 /* ValueBase */ {
        unimplemented!()
    }

    // Weighted Value Base Parsing Function
    fn parse_weighted_value(
        &mut self,
        node: &xmlpp::Element,
        // _type: &types_namespace::TypeWeightedValueBase,
        canvas: Box<Canvas>,
    ) -> i32 /* ValueBase */ {
        unimplemented!()
    }

    // Pair Value Base Parsing Function
    fn parse_pair(
        &mut self,
        node: &xmlpp::Element,
        // _type: &types_namespace::TypePairBase,
        canvas: Box<Canvas>,
    ) -> i32 /* ValueBase */ {
        unimplemented!()
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
        unimplemented!()
    }

    // Dash Item Value Base Parsing Function
    fn parse_dash_item(&mut self, node: &xmlpp::Element) -> i32 /* DashItem */ {
        unimplemented!()
    }

    // Keyframe Parsing Function
    fn parse_keyframe(&mut self, node: &xmlpp::Element, canvas: Box<Canvas>) -> i32 /* Keyframe */
    {
        unimplemented!()
    }

    // ValueNode Animated Parsing Function
    fn parse_animated(&mut self, node: &xmlpp::Element, canvas: Box<Canvas>) -> i32 /* etl::handle<ValueNode_Animated> */
    {
        0
    }

    // Linkable ValueNode Parsing Function
    fn parse_linkable_value_node(&mut self, node: &xmlpp::Element, canvas: Box<Canvas>) -> i32 /*etl::handle<LinkableValueNode>*/
    {
        unimplemented!()
    }

    // Static List Parsnig Function
    fn parse_static_list(&mut self, node: &xmlpp::Element, canvas: Box<Canvas>) -> i32 /* etl::handle<ValueNode_StaticList> */
    {
        unimplemented!()
    }

    // Dynamic List Parsnig Function
    fn parse_dynamic_list(&mut self, node: &xmlpp::Element, canvas: Box<Canvas>) -> i32 /* etl::handle<ValueNode_DynamicList> */
    {
        unimplemented!()
    }

    // Interpolation option for ValueBase parsing function
    fn parse_interpolation(&mut self, node: &xmlpp::Element, attribute: String) -> i32 /* Interpolation */
    {
        unimplemented!()
    }

    // Static option for ValueBase parsing function
    fn parse_static(&mut self, node: &xmlpp::Element) -> bool {
        false
    }
}
