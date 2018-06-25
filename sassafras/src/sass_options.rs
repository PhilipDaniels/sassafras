use std::path::PathBuf;
use sass_output_options::{SassOutputStyle, SassOutputOptions};
use std::os::raw::c_char;
use c_api_helpers::*;

// sass config options structure
#[derive(Default, Debug)]
#[repr(C)]
pub struct SassOptions {
    pub output_options: SassOutputOptions,

    // embed sourceMappingUrl as data uri
    pub source_map_embed: bool,

    // embed include contents in maps
    pub source_map_contents: bool,

    // create file urls for sources
    pub source_map_file_urls: bool,

    // Disable sourceMappingUrl in css output
    pub omit_source_map_url: bool,

    // Treat source_string as sass (as opposed to scss)
    pub is_indented_syntax_src: bool,

    // The input path is used for source map
    // generation. It can be used to define
    // something with string compilation or to
    // overload the input file path. It is
    // set to "stdin" for data contexts and
    // to the input file on file contexts.
    pub input_path: PathBuf,

    // The output path is used for source map
    // generation. LibSass will not write to
    // this file, it is just used to create
    // information in source-maps etc.
    pub output_path: PathBuf,

    // Colon-separated list of paths
    // Semicolon-separated on Windows
    // Maybe use array interface instead?
    pub extension: String,
    pub include_path: PathBuf,
    pub plugin_path: PathBuf,

    // Extensions (linked string list)
    extensions: Vec<PathBuf>,

    // Include paths (linked string list)
    include_paths: Vec<PathBuf>,

    // Plugin paths (linked string list)
    plugin_paths: Vec<PathBuf>,

    // Path to source map file
    // Enables source map generation
    // Used to create sourceMappingUrl
    pub source_map_file: PathBuf,

    // Directly inserted in source maps
    pub source_map_root: String,

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

    pub fn push_import_extension<P: Into<PathBuf>>(&mut self, path: P) {
        let path = path.into();
        if !self.extensions.contains(&path) {
            self.extensions.push(path);
        }
    }

    pub fn push_include_path<P: Into<PathBuf>>(&mut self, path: P) {
        let path = path.into();
        if !self.include_paths.contains(&path) {
            self.include_paths.push(path);
        }
    }

    pub fn push_plugin_path<P: Into<PathBuf>>(&mut self, path: P) {
        let path = path.into();
        if !self.plugin_paths.contains(&path) {
            self.plugin_paths.push(path);
        }
    }
}

// For debugging, to show that something is actually dropped.
impl Drop for SassOptions {
    fn drop(&mut self) {
        //println!("Dropping Sass_Options `{:#?}`", self);
    }
}

// ---------------------------------------------------------------------------------

//// FROM: src/sass_context.cpp.
#[no_mangle]
pub extern fn sass_make_options() -> *mut SassOptions {
    let options = SassOptions::new();
    box_to_raw_ptr(options)
}

#[no_mangle]
pub extern fn sass_delete_options(options_ptr: *mut SassOptions) {
    drop_raw_ptr(options_ptr);
}

#[no_mangle]
pub extern fn sass_option_set_precision(options_ptr: *mut SassOptions, precision: u8) {
    let options = ptr_to_ref(options_ptr);
    options.output_options.inspect_options.precision = precision;
}

#[no_mangle]
pub extern fn sass_option_set_output_style(options_ptr: *mut SassOptions, output_style: SassOutputStyle) {
    let options = ptr_to_ref(options_ptr);
    options.output_options.inspect_options.output_style = output_style;
}

#[no_mangle]
pub extern fn sass_option_push_import_extension(options_ptr: *mut SassOptions, ext: *const c_char) {
    let options = ptr_to_ref(options_ptr);
    let pb = c_char_ptr_to_pathbuf(ext);
    options.push_import_extension(pb);
}

#[no_mangle]
pub extern fn sass_option_push_include_path(options_ptr: *mut SassOptions, path: *const c_char) {
    let options = ptr_to_ref(options_ptr);
    let pb = c_char_ptr_to_pathbuf(path);
    options.push_include_path(pb);
}

#[no_mangle]
pub extern fn sass_option_push_plugin_path(options_ptr: *mut SassOptions, path: *const c_char) {
    let options = ptr_to_ref(options_ptr);
    let pb = c_char_ptr_to_pathbuf(path);
    options.push_plugin_path(pb);
}

#[no_mangle]
pub extern fn sass_option_set_source_comments(options_ptr: *mut SassOptions, source_comments: bool) {
    let options = ptr_to_ref(options_ptr);
    options.output_options.source_comments = source_comments;
}

#[no_mangle]
pub extern fn sass_option_set_omit_source_map_url(options_ptr: *mut SassOptions, omit_source_map_url: bool) {
    let options = ptr_to_ref(options_ptr);
    options.omit_source_map_url = omit_source_map_url;
}

#[no_mangle]
pub extern fn sass_option_set_is_indented_syntax_src(options_ptr: *mut SassOptions, is_indented_syntax_src: bool) {
    let options = ptr_to_ref(options_ptr);
    options.is_indented_syntax_src = is_indented_syntax_src;
}

#[no_mangle]
pub extern fn sass_option_set_source_map_embed(options_ptr: *mut SassOptions, is_indented_syntax_src: bool) {
    let options = ptr_to_ref(options_ptr);
    options.source_map_embed = is_indented_syntax_src;
}

#[no_mangle]
pub extern fn sass_option_set_source_map_file(options_ptr: *mut SassOptions, source_map_file: *const c_char) {
    let options = ptr_to_ref(options_ptr);
    let pb = c_char_ptr_to_pathbuf(source_map_file);
    options.source_map_file = pb;
}

#[no_mangle]
pub extern fn sass_option_get_precision(options_ptr: *mut SassOptions) -> u8 {
    let options = ptr_to_ref(options_ptr);
    options.output_options.inspect_options.precision
}

#[no_mangle]
pub extern fn sass_option_get_output_style(options_ptr: *mut SassOptions) -> SassOutputStyle {
    let options = ptr_to_ref(options_ptr);
    options.output_options.inspect_options.output_style
}

#[no_mangle]
pub extern fn sass_option_get_source_comments(options_ptr: *mut SassOptions) -> bool {
    let options = ptr_to_ref(options_ptr);
    options.output_options.source_comments
}

#[no_mangle]
pub extern fn sass_option_get_source_map_embed(options_ptr: *mut SassOptions) -> bool {
    let options = ptr_to_ref(options_ptr);
    options.source_map_embed
}

#[no_mangle]
pub extern fn sass_option_get_source_map_contents(options_ptr: *mut SassOptions) -> bool {
    let options = ptr_to_ref(options_ptr);
    options.source_map_contents
}

#[no_mangle]
pub extern fn sass_option_get_source_map_file_urls(options_ptr: *mut SassOptions) {
}

#[no_mangle]
pub extern fn sass_option_get_omit_source_map_url(options_ptr: *mut SassOptions) -> bool {
    let options = ptr_to_ref(options_ptr);
    options.omit_source_map_url
}

#[no_mangle]
pub extern fn sass_option_get_is_indented_syntax_src(options_ptr: *mut SassOptions) -> bool {
    let options = ptr_to_ref(options_ptr);
    options.is_indented_syntax_src
}

#[no_mangle]
pub extern fn sass_option_get_indent(options_ptr: *mut SassOptions) -> bool {
    //ADDAPI const char* ADDCALL sass_option_get_indent (struct Sass_Options* options);
    let options = ptr_to_ref(options_ptr);
    true
}

#[no_mangle]
pub extern fn sass_option_get_linefeed(options_ptr: *mut SassOptions) -> bool {
    //ADDAPI const char* ADDCALL sass_option_get_linefeed (struct Sass_Options* options);
    let options = ptr_to_ref(options_ptr);
    true
}

#[no_mangle]
pub extern fn sass_option_get_input_path(options_ptr: *mut SassOptions) -> bool {
    //ADDAPI const char* ADDCALL sass_option_get_input_path (struct Sass_Options* options);
    let options = ptr_to_ref(options_ptr);
    true
}

#[no_mangle]
pub extern fn sass_option_get_output_path(options_ptr: *mut SassOptions) -> bool {
    //ADDAPI const char* ADDCALL sass_option_get_output_path (struct Sass_Options* options);
    let options = ptr_to_ref(options_ptr);
    true
}

#[no_mangle]
pub extern fn sass_option_get_source_map_file(options_ptr: *mut SassOptions) -> *const c_char {
    let options = ptr_to_ref(options_ptr);
    //let smf_path_buf = &options.source_map_file;
    //let os_str = smf_path_buf.as_os_str();  // returns &OsStr
    //unsafe { os_str as *const OsStr as *const c_char }
    path_to_c_char_ptr(&options.source_map_file)
}

//ADDAPI const char* ADDCALL sass_option_get_source_map_root (struct Sass_Options* options);
//ADDAPI Sass_Importer_List ADDCALL sass_option_get_c_headers (struct Sass_Options* options);
//ADDAPI Sass_Importer_List ADDCALL sass_option_get_c_importers (struct Sass_Options* options);
//ADDAPI Sass_Function_List ADDCALL sass_option_get_c_functions (struct Sass_Options* options);

#[no_mangle]
pub extern fn sass_option_set_source_map_contents(options_ptr: *mut SassOptions, source_map_contents: bool) {
    let options = ptr_to_ref(options_ptr);
    options.source_map_contents = source_map_contents;
}

#[no_mangle]
pub extern fn sass_option_set_source_map_file_urls(options_ptr: *mut SassOptions, source_map_file_urls: bool) {
    let options = ptr_to_ref(options_ptr);
    options.source_map_file_urls = source_map_file_urls;
}

#[no_mangle]
pub extern fn sass_option_set_indent(options_ptr: *mut SassOptions, indent: *const c_char) {
    let options = ptr_to_ref(options_ptr);
    //options.source_map_file_urls = source_map_file_urls;
}

#[no_mangle]
pub extern fn sass_option_set_linefeed(options_ptr: *mut SassOptions, linefeed: *const c_char) {
    let options = ptr_to_ref(options_ptr);
    //options.source_map_file_urls = source_map_file_urls;
}

#[no_mangle]
pub extern fn sass_option_set_input_path(options_ptr: *mut SassOptions, input_path: *const c_char) {
    let options = ptr_to_ref(options_ptr);
    let pb = c_char_ptr_to_pathbuf(input_path);
    options.input_path = pb;
}

#[no_mangle]
pub extern fn sass_option_set_plugin_path(options_ptr: *mut SassOptions, plugin_path: *const c_char) {
    let options = ptr_to_ref(options_ptr);
    let pb = c_char_ptr_to_pathbuf(plugin_path);
    options.output_path = pb;
}

#[no_mangle]
pub extern fn sass_option_set_include_path(options_ptr: *mut SassOptions, include_path: *const c_char) {
    let options = ptr_to_ref(options_ptr);
    let pb = c_char_ptr_to_pathbuf(include_path);
    options.output_path = pb;
}

#[no_mangle]
pub extern fn sass_option_set_source_map_root(options_ptr: *mut SassOptions, source_map_root: *const c_char) {
    let options = ptr_to_ref(options_ptr);
    let pb = c_char_ptr_to_pathbuf(source_map_root);
    options.output_path = pb;
}

#[no_mangle]
pub extern fn sass_option_set_c_headers(options_ptr: *mut SassOptions, c_headers: *const c_char) {
    // c_headers: SassImporterList (yes this is right)
    let options = ptr_to_ref(options_ptr);
}

#[no_mangle]
pub extern fn sass_option_set_c_importers(options_ptr: *mut SassOptions, c_importers: *const c_char) {
    // c_importers: SassImporterList
    let options = ptr_to_ref(options_ptr);
//    let pb = c_char_ptr_to_pathbuf(output_path);
//    options.output_path = pb;
}

#[no_mangle]
pub extern fn sass_option_set_c_functions(options_ptr: *mut SassOptions, c_functions: *const c_char) {
    let options = ptr_to_ref(options_ptr);
//    let pb = c_char_ptr_to_pathbuf(output_path);
//    options.output_path = pb;
}

#[no_mangle]
pub extern fn sass_option_set_output_path (options_ptr: *mut SassOptions, output_path: *const c_char) {
    let options = ptr_to_ref(options_ptr);
    let pb = c_char_ptr_to_pathbuf(output_path);
    options.output_path = pb;
}

#[no_mangle]
pub extern fn sass_option_get_include_path_size(options_ptr: *mut SassOptions) -> usize {
    let options = ptr_to_ref(options_ptr);
    options.include_paths.len()
}

/// Returns a non-owning pointer to a path.
#[no_mangle]
pub extern fn sass_option_get_include_path(options_ptr: *mut SassOptions, i: usize) -> usize {
    let options = ptr_to_ref(options_ptr);
    let p = &options.include_paths[i];
    i
}

//// Resolve a file via the given include paths in the sass option struct
//// find_file looks for the exact file name while find_include does a regular sass include
//ADDAPI char* ADDCALL sass_find_file (const char* path, struct Sass_Options* opt);
//ADDAPI char* ADDCALL sass_find_include (const char* path, struct Sass_Options* opt);





/*
static void copy_options(struct Sass_Options* to, struct Sass_Options* from) {
    // do not overwrite ourself
    if (to == from) return;
    // free assigned memory
    sass_clear_options(to);
    // move memory
    *to = *from;
    // Reset pointers on source
    sass_reset_options(from);
}

  // helper function, not exported, only accessible locally
  static void sass_reset_options (struct Sass_Options* options)
  {
    // free pointer before
    // or copy/move them
    options->input_path = 0;
    options->output_path = 0;
    options->plugin_path = 0;
    options->include_path = 0;
    options->source_map_file = 0;
    options->source_map_root = 0;
    options->c_functions = 0;
    options->c_importers = 0;
    options->c_headers = 0;
    options->plugin_paths = 0;
    options->include_paths = 0;
    options->extensions = 0;
  }

  // helper function, not exported, only accessible locally
  static void sass_clear_options (struct Sass_Options* options)
  {
    if (options == 0) return;
    // Deallocate custom functions, headers and importes
    sass_delete_function_list(options->c_functions);
    sass_delete_importer_list(options->c_importers);
    sass_delete_importer_list(options->c_headers);
    // Deallocate inc paths
    if (options->plugin_paths) {
      struct string_list* cur;
      struct string_list* next;
      cur = options->plugin_paths;
      while (cur) {
        next = cur->next;
        free(cur->string);
        free(cur);
        cur = next;
      }
    }
    // Deallocate inc paths
    if (options->include_paths) {
      struct string_list* cur;
      struct string_list* next;
      cur = options->include_paths;
      while (cur) {
        next = cur->next;
        free(cur->string);
        free(cur);
        cur = next;
      }
    }
    // Deallocate extension
    if (options->extensions) {
      struct string_list* cur;
      struct string_list* next;
      cur = options->extensions;
      while (cur) {
        next = cur->next;
        free(cur->string);
        free(cur);
        cur = next;
      }
    }
    // Free options strings
    free(options->input_path);
    free(options->output_path);
    free(options->plugin_path);
    free(options->include_path);
    free(options->source_map_file);
    free(options->source_map_root);
    // Reset our pointers
    options->input_path = 0;
    options->output_path = 0;
    options->plugin_path = 0;
    options->include_path = 0;
    options->source_map_file = 0;
    options->source_map_root = 0;
    options->c_functions = 0;
    options->c_importers = 0;
    options->c_headers = 0;
    options->plugin_paths = 0;
    options->include_paths = 0;
    options->extensions = 0;
  }
*/