use sass_options::SassOptions;
use std::path::PathBuf;

// input behaviours
#[derive(Debug)]
#[repr(C)]
pub enum SassInputStyle {
    Null,
    File,
    Data,
    Folder,
}

// base for all contexts
pub struct SassContext {
    options: SassOptions,
    // store context type info
    context_type: SassInputStyle,
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
    included_files: Vec<PathBuf>,
}

//// Getters for Sass_Context values
//ADDAPI const char* ADDCALL sass_context_get_output_string (struct Sass_Context* ctx);
//ADDAPI int ADDCALL sass_context_get_error_status (struct Sass_Context* ctx);
//ADDAPI const char* ADDCALL sass_context_get_error_json (struct Sass_Context* ctx);
//ADDAPI const char* ADDCALL sass_context_get_error_text (struct Sass_Context* ctx);
//ADDAPI const char* ADDCALL sass_context_get_error_message (struct Sass_Context* ctx);
//ADDAPI const char* ADDCALL sass_context_get_error_file (struct Sass_Context* ctx);
//ADDAPI const char* ADDCALL sass_context_get_error_src (struct Sass_Context* ctx);
//ADDAPI size_t ADDCALL sass_context_get_error_line (struct Sass_Context* ctx);
//ADDAPI size_t ADDCALL sass_context_get_error_column (struct Sass_Context* ctx);
//ADDAPI const char* ADDCALL sass_context_get_source_map_string (struct Sass_Context* ctx);
//ADDAPI char** ADDCALL sass_context_get_included_files (struct Sass_Context* ctx);

//// Calculate the size of the stored null terminated array
//ADDAPI size_t ADDCALL sass_context_get_included_files_size (struct Sass_Context* ctx);
//
//// Take ownership of memory (value on context is set to 0)
//ADDAPI char* ADDCALL sass_context_take_error_json (struct Sass_Context* ctx);
//ADDAPI char* ADDCALL sass_context_take_error_text (struct Sass_Context* ctx);
//ADDAPI char* ADDCALL sass_context_take_error_message (struct Sass_Context* ctx);
//ADDAPI char* ADDCALL sass_context_take_error_file (struct Sass_Context* ctx);
//ADDAPI char* ADDCALL sass_context_take_output_string (struct Sass_Context* ctx);
//ADDAPI char* ADDCALL sass_context_take_source_map_string (struct Sass_Context* ctx);
//ADDAPI char** ADDCALL sass_context_take_included_files (struct Sass_Context* ctx);

//// Getters for Context_Options from Sass_Context
//ADDAPI struct Sass_Options* ADDCALL sass_context_get_options (struct Sass_Context* ctx);
