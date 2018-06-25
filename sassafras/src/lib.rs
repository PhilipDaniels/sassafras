#[allow(unused_variables)]
#[allow(dead_code)]

extern crate libc;

pub mod base;
pub mod c_api_helpers;
pub mod functions;
pub mod sass2scss;
pub mod sass_compiler;
pub mod sass_context;
pub mod sass_data_context;
pub mod sass_file_context;
pub mod sass_inspect_options;
pub mod sass_options;
pub mod sass_output_options;

/*
  #define IMPLEMENT_SASS_OPTION_ACCESSOR(type, option) \
    type ADDCALL sass_option_get_##option (struct Sass_Options* options) { return options->option; } \
    void ADDCALL sass_option_set_##option (struct Sass_Options* options, type option) { options->option = option; }
  #define IMPLEMENT_SASS_OPTION_STRING_GETTER(type, option, def) \
    type ADDCALL sass_option_get_##option (struct Sass_Options* options) { return safe_str(options->option, def); }
  #define IMPLEMENT_SASS_OPTION_STRING_SETTER(type, option, def) \
    void ADDCALL sass_option_set_##option (struct Sass_Options* options, type option) \
    { free(options->option); options->option = option || def ? sass_copy_c_string(option ? option : def) : 0; }
  #define IMPLEMENT_SASS_OPTION_STRING_ACCESSOR(type, option, def) \
    IMPLEMENT_SASS_OPTION_STRING_GETTER(type, option, def) \
    IMPLEMENT_SASS_OPTION_STRING_SETTER(type, option, def)

  #define IMPLEMENT_SASS_CONTEXT_GETTER(type, option) \
    type ADDCALL sass_context_get_##option (struct Sass_Context* ctx) { return ctx->option; }
  #define IMPLEMENT_SASS_CONTEXT_TAKER(type, option) \
    type sass_context_take_##option (struct Sass_Context* ctx) \
    { type foo = ctx->option; ctx->option = 0; return foo; }
*/