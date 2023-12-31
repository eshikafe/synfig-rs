use std::collections::HashMap;

pub const CURRENT_CANVAS_VERSION: &str = "1.2";

pub type Handle = i32;
pub type LooseHandle = i32;
pub type ConstHandle = i32;
pub type Children = Vec<Handle>;

pub struct Canvas {
    // Contains the ID string for the Canvas
	//	see get_id(), set_id() 
	id_: String,

	// Contains the name of the Canvas
	//	see set_name(), get_name()
	name_: String,

	// Contains a description of the Canvas
	//	see set_description(), get_description()
	description_: String,

	// Contains the canvas' version string
	//	see set_version(), get_version() 
	version_: String,

	// Contains the author's name
	//	see set_author(), get_author()
	 author_: String,

	// File name of Canvas
	// see get_file_name(), set_file_name()
	file_name_: String,

	// File identifier of Canvas
	// see get_identifier(), set_identifier()
	// identifier_: FileSystem::Identifier,

	// Metadata map for Canvas.
	// see get_meta_data(), set_meta_data(), erase_meta_data()
	meta_data_: HashMap<String, String>,

	// Contains a list of ValueNodes that are in this Canvas
	//	see value_node_list(), find_value_node()
	//  value_node_list: ValueNodeList,

	// Contains a list of Keyframes that are in the Canvas
	// see keyframe_list()
	// keyframe_list_: KeyframeList,

	// A handle to the parent canvas of this canvas.
	//	If canvas is a root canvas, then this handle is empty
	//	see parent()
	// parent_: LooseHandle,

	// List containing any child Canvases
	//	see children() */
	// children_: Children,

	// Render Description for Canvas
	//	see rend_desc() */
	// desc_: RendDesc,

	// Contains the value of the last call to set_time()
	// cur_time_: Time,

	// Map of external Canvases used in this Canvas
	// mutable std::map<String,Handle> externals_;

	// This flag is set if this canvas is "inline"
	is_inline_: bool,

	// True if the Canvas properties has changed
	is_dirty_: bool,

	// Layer Group database
	// std::map<String,std::set<etl::handle<Layer> > > group_db_;

	// Layer Signal Connection database.
	// Required to properly disconnect them when a layer is removed from canvas
	// (and not necessarily deleted).
	// std::map<etl::loose_handle<Layer>,std::vector<sigc::connection> > connections_;

	// Value to store temporarily the grow value for the child outline type layers
	// \see get_grow_value set_grow_value */
	outline_grow: f64,

}
