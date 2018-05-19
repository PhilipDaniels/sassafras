use sass_options::Sass_Options;
use std::path::PathBuf;

// input behaviours
#[derive(Debug)]
#[repr(C)]
pub enum Sass_Input_Style {
    SASS_CONTEXT_NULL,
    SASS_CONTEXT_FILE,
    SASS_CONTEXT_DATA,
    SASS_CONTEXT_FOLDER
}

// base for all contexts
pub struct Sass_Context {
    options: Sass_Options,
    // store context type info
    context_type: Sass_Input_Style,
    // generated output data
    output_string: String,
    // generated source map json
    source_map_string: String,

    // error status
    error_status: i32,
    error_json: String,
    error_text: String,
    error_message: String,

    // error position
    error_file: PathBuf,
    error_line: usize,
    error_column: usize,
    error_src: String,

    // report imported files
    included_files: Vec<PathBuf>
}
