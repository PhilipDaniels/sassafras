use sass_context::SassContext;
use std::os::raw::c_char;
use sass_context::SassInputStyle;
use c_api_helpers::*;
use std::path::PathBuf;
use sass_options::SassOptions;
use std::os::raw::c_int;

// struct for file compilation
#[derive(Default, Debug)]
#[repr(C)]
pub struct SassFileContext {
    pub context: SassContext
    // no additional fields required
    // input_path is already on options
}

impl SassFileContext {
    pub fn new<P: Into<PathBuf>>(input_path: P) -> Self {
        let mut ctx = SassFileContext::default();
        // ctx->type = SASS_CONTEXT_FILE;
        ctx.context.context_type = SassInputStyle::File;
        ctx.context.options.init();

        ctx.context.options.input_path = input_path.into();
        // try {
        //     if (input_path == 0) { throw(std::runtime_error("File context created without an input path")); }
        //     if (*input_path == 0) { throw(std::runtime_error("File context created with empty input path")); }
        //     sass_option_set_input_path(ctx, input_path);
        // } catch (...) {
        //    handle_errors(ctx);
        // }
        ctx
    }
}

#[no_mangle]
pub extern fn sass_make_file_context(input_path: *const c_char) -> *mut SassFileContext {
    // SharedObj::setTaint(true); // needed for static colors
    let pb = c_char_ptr_to_pathbuf(input_path);
    let ctx = SassFileContext::new(pb);
    box_to_raw_ptr(ctx)
}

// Call the compilation step for the specific context
#[no_mangle]
pub extern fn sass_compile_file_context(file_ctx: *mut SassFileContext) -> c_int {
    let ctx = ptr_to_ref(file_ctx);

//    if (file_ctx == 0) return 1;
//    if (file_ctx->error_status)
//    return file_ctx->error_status;
//    try {
//        if (file_ctx->input_path == 0) { throw(std::runtime_error("File context has no input path")); }
//    if (*file_ctx->input_path == 0) { throw(std::runtime_error("File context has empty input path")); }
//    }
//    catch (...) { return handle_errors(file_ctx) | 1; }
//    Context* cpp_ctx = new File_Context(*file_ctx);
//    return sass_compile_context(file_ctx, cpp_ctx);
    1
}

/*
//// Create a sass compiler instance for more control
  struct Sass_Compiler* ADDCALL sass_make_file_compiler (struct Sass_File_Context* file_ctx)
  {
    if (file_ctx == 0) return 0;
    Context* cpp_ctx = new File_Context(*file_ctx);
    return sass_prepare_context(file_ctx, cpp_ctx);
  }

*/

#[no_mangle]
pub extern fn sass_delete_file_context(ptr: *mut SassFileContext) {
    drop_raw_ptr(ptr);
}

#[no_mangle]
pub extern fn sass_file_context_get_context(file_ctx: *mut SassFileContext) -> *mut SassContext {
    let ctx = ptr_to_ref(file_ctx);
    &mut ctx.context
}

#[no_mangle]
pub extern fn sass_file_context_get_options(file_ctx: *mut SassFileContext) -> *mut SassOptions {
    let ctx = ptr_to_ref(file_ctx);
    &mut ctx.context.options
}

#[no_mangle]
pub extern fn sass_file_context_set_options(file_ctx: *mut SassFileContext, options: *mut SassOptions) {
    // { copy_options(ctx, opt); }
    let ctx = ptr_to_ref(file_ctx);
    ctx.context.options = ptr_to(options);
}
