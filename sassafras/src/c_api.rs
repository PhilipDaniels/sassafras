use std::path::PathBuf;

// FROM: include/sass/base.h
// Different render styles
#[derive(Debug)]
pub enum Sass_Output_Style {
    SASS_STYLE_NESTED,
    SASS_STYLE_EXPANDED,
    SASS_STYLE_COMPACT,
    SASS_STYLE_COMPRESSED,
    // only used internaly
    SASS_STYLE_INSPECT,
    SASS_STYLE_TO_SASS
}

impl Default for Sass_Output_Style {
    fn default() -> Self {
        Sass_Output_Style::SASS_STYLE_NESTED
    }
}

// FROM: stc/sass.hpp
// input behaviours
#[derive(Debug)]
pub enum Sass_Input_Style {
    SASS_CONTEXT_NULL,
    SASS_CONTEXT_FILE,
    SASS_CONTEXT_DATA,
    SASS_CONTEXT_FOLDER
}

// FROM: src/sass.hpp
// sass config options structure
#[derive(Default, Debug)]
pub struct Sass_Inspect_Options {
    // Output style for the generated css code
    output_style: Sass_Output_Style,

    // Precision for fractional numbers
    precision: i32
}

impl Sass_Inspect_Options {
    // Defaults = Nested, 5.
    pub fn new(style: Sass_Output_Style, precision: i32) -> Self {
        Sass_Inspect_Options { output_style: style, precision }
    }
}

// FROM: src/sass.hpp
#[derive(Default, Debug)]
pub struct Sass_Output_Options {
    inspect_options: Sass_Inspect_Options,
    // String to be used for indentation
    indent: String,
    // String to be used to for line feeds
    linefeed: String,
    // Emit comments in the generated CSS indicating
    // the corresponding source line.
    source_comments: bool
}

// FROM: src/sass.hpp
// sass config options structure
impl Sass_Output_Options {
    // Defaults: indent = two spaces, linefeed = '\n', source_comments = false
    pub fn new_from_options<S>(opt: Sass_Inspect_Options, indent: S, linefeed: S, source_comments: bool) -> Self
        where S: Into<String>
    {
        Sass_Output_Options {
            inspect_options: opt,
            indent: indent.into(),
            linefeed: linefeed.into(),
            source_comments
        }
    }

    // Defaults: indent = two spaces, linefeed = '\n', source_comments = false
    pub fn new<S>(style: Sass_Output_Style, precision: i32, indent: S, linefeed: S, source_comments: bool) -> Self
        where S: Into<String>
    {
        Sass_Output_Options {
            inspect_options: Sass_Inspect_Options::new(style, precision),
            indent: indent.into(),
            linefeed: linefeed.into(),
            source_comments
        }
    }
}

// FROM: src/sass_context.hpp
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
    extensions: Vec<String>,

    // Include paths (linked string list)
    include_paths: Vec<String>,

    // Plugin paths (linked string list)
    plugin_paths: Vec<String>,

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

// FROM: src/sass_context.hpp
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

// FROM: src/sass_context.hpp
// struct for file compilation
pub struct Sass_File_Context {
    context: Sass_Context
    // no additional fields required
    // input_path is already on options
}

// FROM: src/sass_context.hpp
// struct for data compilation
pub struct Sass_Data_Context {
    context: Sass_Context,
    source_string: String,
    srcmap_string: String
}

// FROM: include/sass/context.h
// Compiler states
pub enum Sass_Compiler_State {
    SASS_COMPILER_CREATED,
    SASS_COMPILER_PARSED,
    SASS_COMPILER_EXECUTED
}

// FROM: src/sass_context.hpp
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

// FROM: src/sass_context.cpp (done with a macro).
// Create getter and setters for options
pub fn sass_option_set_precision(options: &mut Sass_Options, precision: i32) {
    options.output_options.inspect_options.precision = precision;
}

pub fn sass_option_set_output_style(options: &mut Sass_Options, output_style: Sass_Output_Style) {
    options.output_options.inspect_options.output_style = output_style;
}

pub fn sass_option_push_import_extension(options: &mut Sass_Options, ext: String) {

}
