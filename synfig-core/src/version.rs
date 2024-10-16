use log::error;
use std::mem;

use crate::canvas::Canvas;
use crate::color::Color;
use crate::layer::Layer;
use crate::vector::Vector;

// SYNFIG_VERSION
// Synfig API Version
//
//	SYNFIG_VERSION can be set to ensure
//	compile-time compatibility with future versions
//	of Synfig. The first two digits are the major
//	version, the second two digits are the minor
//	version, and the last two digits are the
//	revision release.
//

pub const SYNFIG_VERSION: &str = env!("CARGO_PKG_VERSION");

// Increment this value whenever
// the library changes in a way
// that breaks library compatibility
pub const SYNFIG_LIBRARY_VERSION: usize = 50;

pub fn synfig_check_version() -> bool {
    check_version_(
        SYNFIG_LIBRARY_VERSION,
        mem::size_of::<Vector>(),
        mem::size_of::<Color>(),
        mem::size_of::<Canvas>(),
        mem::size_of::<Layer>(),
    )
}

// Version checker
// Checks to make sure that the library
// version matches with what the program
// was compiled against.
// see synfig_check_version()

fn check_version_(
    version: usize,
    vec_size: usize,
    color_size: usize,
    canvas_size: usize,
    layer_size: usize,
) -> bool {
    let mut ret: bool = true;

    if version != SYNFIG_LIBRARY_VERSION {
        error!(
            "API Version mismatch (LIB:{}, PROG:{})",
            SYNFIG_LIBRARY_VERSION, version
        );
        ret = false;
    }
    if vec_size != mem::size_of::<Vector>() {
        error!(
            "Size of Vector mismatch (app:{}, lib:{})",
            vec_size,
            mem::size_of::<Vector>()
        );
        ret = false;
    }
    if color_size != mem::size_of::<Color>() {
        error!(
            "Size of Color mismatch (app:{}, lib:{})",
            color_size,
            mem::size_of::<Color>()
        );
        ret = false;
    }
    if canvas_size != mem::size_of::<Canvas>() {
        error!(
            "Size of Canvas mismatch (app:{}, lib:{})",
            canvas_size,
            mem::size_of::<Canvas>()
        );
        ret = false;
    }
    if layer_size != mem::size_of::<Layer>() {
        error!(
            "Size of Layer mismatch (app:{}, lib:{})",
            layer_size,
            mem::size_of::<Layer>()
        );
        ret = false;
    }

    ret
}

pub fn get_version() -> &'static str {
    SYNFIG_VERSION
}

pub fn get_build_date() -> String {
    String::from("build date")
}
