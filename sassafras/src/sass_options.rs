use std::path::PathBuf;
use sass_output_options::{SassOutputStyle, SassOutputOptions};
use std::os::raw::c_char;
use std::ffi::{CStr, CString, OsStr, OsString};
use std::slice;
use libc;

// sass config options structure
#[derive(Default, Debug)]
#[repr(C)]
pub struct SassOptions {
    output_options: SassOutputOptions,

    // embed sourceMappingUrl as data uri
    source_map_embed: bool,

    // embed include contents in maps
    source_map_contents: bool,

    // create file urls for sources
    source_map_file_urls: bool,

    // Disable sourceMappingUrl in css output
    omit_source_map_url: bool,

    // Treat source_string as sass (as opposed to scss)
    is_indented_syntax_src: bool,

    // The input path is used for source map
    // generation. It can be used to define
    // something with string compilation or to
    // overload the input file path. It is
    // set to "stdin" for data contexts and
    // to the input file on file contexts.
    input_path: PathBuf,

    // The output path is used for source map
    // generation. LibSass will not write to
    // this file, it is just used to create
    // information in source-maps etc.
    output_path: PathBuf,

    // Colon-separated list of paths
    // Semicolon-separated on Windows
    // Maybe use array interface instead?
    extension: String,
    include_path: PathBuf,
    plugin_path: PathBuf,

    // Extensions (linked string list)
    extensions: Vec<PathBuf>,

    // Include paths (linked string list)
    include_paths: Vec<PathBuf>,

    // Plugin paths (linked string list)
    plugin_paths: Vec<PathBuf>,

    // Path to source map file
    // Enables source map generation
    // Used to create sourceMappingUrl
    source_map_file: PathBuf,

    // Directly inserted in source maps
    source_map_root: String,

    // Custom functions that can be called from sccs code
    //c_functions: Sass_Function_List,

    // List of custom importers
    //c_importers: Sass_Importer_List,

    // List of custom headers
    //c_headers: Sass_Importer_List
}

impl SassOptions {
    pub fn new() -> Self {
        let mut options = SassOptions::default();
        options.init();
        options
    }

    pub fn init(&mut self) {
        self.output_options.inspect_options.precision = 5;
        self.output_options.indent = "  ".to_string();
        self.output_options.linefeed = "\n".to_string()
    }
}

// For debugging, to show that something is actually dropped.
impl Drop for SassOptions {
    fn drop(&mut self) {
        println!("Dropping Sass_Options `{:#?}`!", self);
    }
}

// ---------------------------------------------------------------------------------

fn unpack_ptr<'a>(options_ptr: *mut SassOptions) -> &'a mut SassOptions {
    assert!(!options_ptr.is_null());

    unsafe {
        &mut *options_ptr
    }
}

fn c_char_ptr_to_cstr<'a>(ptr: *const c_char) -> &'a CStr {
    assert!(!ptr.is_null());
    unsafe { CStr::from_ptr(ptr) }
}

fn c_char_ptr_to_cstring(ptr: *const c_char) -> CString {
    let slice = c_char_ptr_to_cstr(ptr);
    CString::from(slice)
}

fn c_char_ptr_to_string(ptr: *const c_char) -> String {
    let slice = c_char_ptr_to_cstr(ptr);
    slice.to_string_lossy().into_owned()
}

fn c_char_ptr_to_vec(ptr: *const c_char) -> Vec<u8> {
    let s = c_char_ptr_to_cstring(ptr);
    s.to_bytes().to_vec()
}

#[cfg(unix)]
fn c_char_ptr_to_osstring(ptr: *const c_char) -> OsString {
    use std::os::unix::ffi::OsStrExt;

    let slice = c_char_ptr_to_cstr(ptr);
    let bytes = slice.to_bytes();
    OsString::from(OsStr::from_bytes(bytes))
//    assert!(!ptr.is_null());
//    let cstr = c_char_ptr_to_cstring(ptr);
//    let bytes = cstr.into_bytes();
//    OsString::from_vec(bytes)
}

#[cfg(windows)]
fn c_char_ptr_to_osstring(ptr: *const c_char) -> OsString {
    let bytes = c_char_ptr_to_vec(ptr);
    OsString::from_vec(bytes)
}

fn c_char_ptr_to_pathbuf(ptr: *const c_char) -> PathBuf {
    let osstr = c_char_ptr_to_osstring(ptr);
    PathBuf::from(osstr)
}








// For debugging.
#[no_mangle]
pub fn sass_option_print(options_ptr: *mut SassOptions) {
    let options = unpack_ptr(options_ptr);
    println!("{:#?}", options);
}

// ---------------------------------------------------------------------------------

//// FROM: src/sass_context.cpp.
#[no_mangle]
pub extern fn sass_make_options() -> *mut SassOptions {
    let mut options = SassOptions::new();
    // Box::new() places the options struct onto the heap, then
    // into_raw() ensures it is not cleaned up.
    Box::into_raw(Box::new(options))
}

#[no_mangle]
pub extern fn sass_delete_options(options_ptr: *mut SassOptions) {
    if !options_ptr.is_null() {
        unsafe {
            // from_raw() constructs a box, which is then automatically dropped
            // at the end of the scope, calling drop() on the struct.
            Box::from_raw(options_ptr);
        }
    }
}

#[no_mangle]
pub extern fn sass_option_set_precision(options_ptr: *mut SassOptions, precision: u8) {
    let options = unpack_ptr(options_ptr);
    options.output_options.inspect_options.precision = precision;
}

#[no_mangle]
pub extern fn sass_option_set_output_style(options_ptr: *mut SassOptions, output_style: SassOutputStyle) {
    let options = unpack_ptr(options_ptr);
    options.output_options.inspect_options.output_style = output_style;
}

#[no_mangle]
pub extern fn sass_option_push_import_extension(options_ptr: *mut SassOptions, ext: *const c_char) {
    let options = unpack_ptr(options_ptr);
    // TODO: These methods that push to these vectors should check for existence and not push if already there.
    let pb = c_char_ptr_to_pathbuf(ext);
    options.extensions.push(pb);
}

#[no_mangle]
pub extern fn sass_option_push_include_path(options_ptr: *mut SassOptions, path: PathBuf) {
    let options = unpack_ptr(options_ptr);
    options.include_paths.push(path);
}

#[no_mangle]
pub extern fn sass_option_push_plugin_path(options_ptr: *mut SassOptions, path: PathBuf) {
    let options = unpack_ptr(options_ptr);
    options.plugin_paths.push(path);
}

#[no_mangle]
pub extern fn sass_option_set_source_comments(options_ptr: *mut SassOptions, source_comments: bool) {
    let options = unpack_ptr(options_ptr);
    options.output_options.source_comments = source_comments;
}

#[no_mangle]
pub extern fn sass_option_set_omit_source_map_url(options_ptr: *mut SassOptions, omit_source_map_url: bool) {
    let options = unpack_ptr(options_ptr);
    options.omit_source_map_url = omit_source_map_url;
}

#[no_mangle]
pub extern fn sass_option_set_is_indented_syntax_src(options_ptr: *mut SassOptions, is_indented_syntax_src: bool) {
    let options = unpack_ptr(options_ptr);
    options.is_indented_syntax_src = is_indented_syntax_src;
}

#[no_mangle]
pub extern fn sass_option_set_source_map_embed(options_ptr: *mut SassOptions, is_indented_syntax_src: bool) {
    let options = unpack_ptr(options_ptr);
    options.source_map_embed = is_indented_syntax_src;
}

#[no_mangle]
pub extern fn sass_option_set_source_map_file(options_ptr: *mut SassOptions, source_map_file: PathBuf) {
    let options = unpack_ptr(options_ptr);
    options.source_map_file = source_map_file;
}

//
//// Getters for Context_Option values
//ADDAPI int ADDCALL sass_option_get_precision (struct Sass_Options* options);
//ADDAPI enum Sass_Output_Style ADDCALL sass_option_get_output_style (struct Sass_Options* options);
//ADDAPI bool ADDCALL sass_option_get_source_comments (struct Sass_Options* options);
//ADDAPI bool ADDCALL sass_option_get_source_map_embed (struct Sass_Options* options);
//ADDAPI bool ADDCALL sass_option_get_source_map_contents (struct Sass_Options* options);
//ADDAPI bool ADDCALL sass_option_get_source_map_file_urls (struct Sass_Options* options);
//ADDAPI bool ADDCALL sass_option_get_omit_source_map_url (struct Sass_Options* options);
//ADDAPI bool ADDCALL sass_option_get_is_indented_syntax_src (struct Sass_Options* options);
//ADDAPI const char* ADDCALL sass_option_get_indent (struct Sass_Options* options);
//ADDAPI const char* ADDCALL sass_option_get_linefeed (struct Sass_Options* options);
//ADDAPI const char* ADDCALL sass_option_get_input_path (struct Sass_Options* options);
//ADDAPI const char* ADDCALL sass_option_get_output_path (struct Sass_Options* options);
//ADDAPI const char* ADDCALL sass_option_get_source_map_file (struct Sass_Options* options);
//ADDAPI const char* ADDCALL sass_option_get_source_map_root (struct Sass_Options* options);
//ADDAPI Sass_Importer_List ADDCALL sass_option_get_c_headers (struct Sass_Options* options);
//ADDAPI Sass_Importer_List ADDCALL sass_option_get_c_importers (struct Sass_Options* options);
//ADDAPI Sass_Function_List ADDCALL sass_option_get_c_functions (struct Sass_Options* options);
//
//// Setters for Context_Option values
//ADDAPI void ADDCALL sass_option_set_source_map_contents (struct Sass_Options* options, bool source_map_contents);
//ADDAPI void ADDCALL sass_option_set_source_map_file_urls (struct Sass_Options* options, bool source_map_file_urls);
//ADDAPI void ADDCALL sass_option_set_indent (struct Sass_Options* options, const char* indent);
//ADDAPI void ADDCALL sass_option_set_linefeed (struct Sass_Options* options, const char* linefeed);
//ADDAPI void ADDCALL sass_option_set_input_path (struct Sass_Options* options, const char* input_path);
//ADDAPI void ADDCALL sass_option_set_output_path (struct Sass_Options* options, const char* output_path);
//ADDAPI void ADDCALL sass_option_set_plugin_path (struct Sass_Options* options, const char* plugin_path);
//ADDAPI void ADDCALL sass_option_set_include_path (struct Sass_Options* options, const char* include_path);
//ADDAPI void ADDCALL sass_option_set_source_map_root (struct Sass_Options* options, const char* source_map_root);
//ADDAPI void ADDCALL sass_option_set_c_headers (struct Sass_Options* options, Sass_Importer_List c_headers);
//ADDAPI void ADDCALL sass_option_set_c_importers (struct Sass_Options* options, Sass_Importer_List c_importers);
//ADDAPI void ADDCALL sass_option_set_c_functions (struct Sass_Options* options, Sass_Function_List c_functions);

//// Getters for options include path array
//ADDAPI size_t ADDCALL sass_option_get_include_path_size(struct Sass_Options* options);
//ADDAPI const char* ADDCALL sass_option_get_include_path(struct Sass_Options* options, size_t i);


//// Resolve a file via the given include paths in the sass option struct
//// find_file looks for the exact file name while find_include does a regular sass include
//ADDAPI char* ADDCALL sass_find_file (const char* path, struct Sass_Options* opt);
//ADDAPI char* ADDCALL sass_find_include (const char* path, struct Sass_Options* opt);

