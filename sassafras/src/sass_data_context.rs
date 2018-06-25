use sass_context::SassContext;
use c_api_helpers::ptr_to_ref;
use sass_options::SassOptions;
use c_api_helpers::drop_raw_ptr;

// struct for data compilation
#[derive(Debug, Default)]
#[repr(C)]
pub struct SassDataContext {
    context: SassContext,
    source_string: String,
    srcmap_string: String,
}

#[no_mangle]
pub extern fn sass_data_context_get_options(data_ctx: *mut SassDataContext) -> *mut SassOptions {
    let ctx = ptr_to_ref(data_ctx);
    &mut ctx.context.options
}

//ADDAPI void ADDCALL sass_data_context_set_options (struct Sass_Data_Context* data_ctx, struct Sass_Options* opt)
// { copy_options(ctx, opt); }

#[no_mangle]
pub extern fn sass_data_context_get_context(data_ctx: *mut SassDataContext) -> *mut SassContext {
    let ctx = ptr_to_ref(data_ctx);
    &mut ctx.context
}

#[no_mangle]
pub extern fn sass_delete_data_context(data_ctx: *mut SassDataContext) {
    drop_raw_ptr(data_ctx);
}

/*
struct Sass_Compiler* ADDCALL sass_make_data_compiler (struct Sass_Data_Context* data_ctx)
{
if (data_ctx == 0) return 0;
Context* cpp_ctx = new Data_Context(*data_ctx);
return sass_prepare_context(data_ctx, cpp_ctx);
}
*/

/*
//ADDAPI struct Sass_Data_Context* ADDCALL sass_make_data_context (char* source_string);
  Sass_Data_Context* ADDCALL sass_make_data_context(char* source_string)
  {
    struct Sass_Data_Context* ctx = (struct Sass_Data_Context*) calloc(1, sizeof(struct Sass_Data_Context));
    if (ctx == 0) { std::cerr << "Error allocating memory for data context" << std::endl; return 0; }
    ctx->type = SASS_CONTEXT_DATA;
    init_options(ctx);
    try {
      if (source_string == 0) { throw(std::runtime_error("Data context created without a source string")); }
      if (*source_string == 0) { throw(std::runtime_error("Data context created with empty source string")); }
      ctx->source_string = source_string;
    } catch (...) {
      handle_errors(ctx);
    }
    return ctx;
  }
*/

/*//ADDAPI int ADDCALL sass_compile_data_context (struct Sass_Data_Context* ctx);
  int ADDCALL sass_compile_data_context(Sass_Data_Context* data_ctx)
  {
    if (data_ctx == 0) return 1;
    if (data_ctx->error_status)
      return data_ctx->error_status;
    try {
      if (data_ctx->source_string == 0) { throw(std::runtime_error("Data context has no source string")); }
      // empty source string is a valid case, even if not really usefull (different than with file context)
      // if (*data_ctx->source_string == 0) { throw(std::runtime_error("Data context has empty source string")); }
    }
    catch (...) { return handle_errors(data_ctx) | 1; }
    Context* cpp_ctx = new Data_Context(*data_ctx);
    return sass_compile_context(data_ctx, cpp_ctx);
  }
*/
