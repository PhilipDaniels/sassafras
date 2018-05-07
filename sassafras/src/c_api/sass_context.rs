// FROM: src/sass_context.hpp


use std::path::PathBuf;
use super::*;

// sass config options structure
#[derive(Default, Debug)]
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
    fn init(&mut self) {
        self.output_options.inspect_options.precision = 5;
        self.output_options.indent = "  ".to_string();
        self.output_options.linefeed = "\n".to_string()
    }
}

// base for all contexts
struct Sass_Context {
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

// struct for file compilation
pub struct Sass_File_Context {
    context: Sass_Context
    // no additional fields required
    // input_path is already on options
}

// struct for data compilation
pub struct Sass_Data_Context {
    context: Sass_Context,
    source_string: String,
    srcmap_string: String
}

// link c and cpp context
pub struct Sass_Compiler {
    // progress status
    state: Sass_Compiler_State,
    // original c context
    c_ctx: Sass_Context,
    // Sass::Context
    //cpp_ctx: Context,
    // Sass::Block
    //root: Block_Obj
}

// FROM: src/sass_context.cpp.
pub fn sass_make_options() -> Sass_Options {
    let mut options = Sass_Options::default();
    options.init();
    options
}

// Create getter and setters for options
pub fn sass_option_set_precision(options: &mut Sass_Options, precision: u8) {
    options.output_options.inspect_options.precision = precision;
}

pub fn sass_option_set_output_style(options: &mut Sass_Options, output_style: Sass_Output_Style) {
    options.output_options.inspect_options.output_style = output_style;
}

pub fn sass_option_push_import_extension(options: &mut Sass_Options, ext: PathBuf) {
    // TODO: These methods that push to these vectors should check for existence and not push if already there.
    options.extensions.push(ext);
}

// TODO: Use Into<PathBuf>? Is this same for a C API?
pub fn sass_option_push_include_path(options: &mut Sass_Options, path: PathBuf) {
    options.include_paths.push(path);
}

pub fn sass_option_push_plugin_path(options: &mut Sass_Options, path: PathBuf) {
    options.plugin_paths.push(path);
}

pub fn sass_option_set_source_comments(options: &mut Sass_Options, source_comments: bool) {
    options.output_options.source_comments = source_comments;
}

pub fn sass_option_set_omit_source_map_url(options: &mut Sass_Options, omit_source_map_url: bool) {
    options.omit_source_map_url = omit_source_map_url;
}

pub fn sass_option_set_is_indented_syntax_src(options: &mut Sass_Options, is_indented_syntax_src: bool) {
    options.is_indented_syntax_src = is_indented_syntax_src;
}

pub fn sass_option_set_source_map_embed(options: &mut Sass_Options, is_indented_syntax_src: bool) {
    options.source_map_embed = is_indented_syntax_src;
}

pub fn sass_option_set_source_map_file<P: Into<PathBuf>>(options: &mut Sass_Options, source_map_file: P) {
    options.source_map_file = source_map_file.into();
}

pub fn sass_delete_options(options: &mut Sass_Options) {

}