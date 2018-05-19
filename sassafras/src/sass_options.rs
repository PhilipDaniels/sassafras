use std::path::PathBuf;
use sass_output_options::{Sass_Output_Style, Sass_Output_Options};

// sass config options structure
#[derive(Default, Debug)]
#[repr(C)]
pub struct Sass_Options {
    output_options: Sass_Output_Options,

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

impl Sass_Options {
    pub fn new() -> Self {
        let mut options = Sass_Options::default();
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
impl Drop for Sass_Options {
    fn drop(&mut self) {
        println!("Dropping Sass_Options `{:#?}`!", self);
    }
}

fn unpack_ptr<'a>(options_ptr: *mut Sass_Options) -> &'a mut Sass_Options {
    assert!(!options_ptr.is_null());

    unsafe {
        &mut *options_ptr
    }
}

//// FROM: src/sass_context.cpp.
#[no_mangle]
pub extern fn sass_make_options() -> *mut Sass_Options {
    let mut options = Sass_Options::new();
    // Box::new() places the options struct onto the heap, then
    // into_raw() ensures it is not cleaned up.
    Box::into_raw(Box::new(options))
}

#[no_mangle]
pub extern fn sass_delete_options(options_ptr: *mut Sass_Options) {
    if !options_ptr.is_null() {
        unsafe {
            // from_raw() constructs a box, which is then automatically dropped
            // at the end of the scope, calling drop() on the struct.
            Box::from_raw(options_ptr);
        }
    }
}

#[no_mangle]
pub extern fn sass_option_set_precision(options_ptr: *mut Sass_Options, precision: u8) {
    let options = unpack_ptr(options_ptr);
    options.output_options.inspect_options.precision = precision;
}

#[no_mangle]
pub fn sass_option_set_output_style(options_ptr: *mut Sass_Options, output_style: Sass_Output_Style) {
    let options = unpack_ptr(options_ptr);
    options.output_options.inspect_options.output_style = output_style;
}

#[no_mangle]
pub fn sass_option_push_import_extension(options_ptr: *mut Sass_Options, ext: PathBuf) {
    let options = unpack_ptr(options_ptr);
    // TODO: These methods that push to these vectors should check for existence and not push if already there.
    options.extensions.push(ext);
}

#[no_mangle]
pub fn sass_option_push_include_path(options_ptr: *mut Sass_Options, path: PathBuf) {
    let options = unpack_ptr(options_ptr);
    options.include_paths.push(path);
}

#[no_mangle]
pub fn sass_option_push_plugin_path(options_ptr: *mut Sass_Options, path: PathBuf) {
    let options = unpack_ptr(options_ptr);
    options.plugin_paths.push(path);
}

#[no_mangle]
pub fn sass_option_set_source_comments(options_ptr: *mut Sass_Options, source_comments: bool) {
    let options = unpack_ptr(options_ptr);
    options.output_options.source_comments = source_comments;
}

#[no_mangle]
pub fn sass_option_set_omit_source_map_url(options_ptr: *mut Sass_Options, omit_source_map_url: bool) {
    let options = unpack_ptr(options_ptr);
    options.omit_source_map_url = omit_source_map_url;
}

#[no_mangle]
pub fn sass_option_set_is_indented_syntax_src(options_ptr: *mut Sass_Options, is_indented_syntax_src: bool) {
    let options = unpack_ptr(options_ptr);
    options.is_indented_syntax_src = is_indented_syntax_src;
}

#[no_mangle]
pub fn sass_option_set_source_map_embed(options_ptr: *mut Sass_Options, is_indented_syntax_src: bool) {
    let options = unpack_ptr(options_ptr);
    options.source_map_embed = is_indented_syntax_src;
}

#[no_mangle]
pub fn sass_option_set_source_map_file(options_ptr: *mut Sass_Options, source_map_file: PathBuf) {
    let options = unpack_ptr(options_ptr);
    options.source_map_file = source_map_file;
}

// For debugging.
#[no_mangle]
pub fn sass_option_print(options_ptr: *mut Sass_Options) {
    let options = unpack_ptr(options_ptr);
    println!("{:#?}", options);
}
