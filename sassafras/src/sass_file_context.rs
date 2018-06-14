use sass_context::SassContext;
use std::os::raw::c_char;
use sass_context::SassInputStyle;
use c_api_helpers::*;
use std::path::PathBuf;

// struct for file compilation
#[derive(Default)]
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
    let mut ctx = SassFileContext::new(pb);
    box_to_raw_ptr(ctx)
}

//// Call the compilation step for the specific context
//ADDAPI int ADDCALL sass_compile_file_context (struct Sass_File_Context* ctx);

//// Create a sass compiler instance for more control
//ADDAPI struct Sass_Compiler* ADDCALL sass_make_file_compiler (struct Sass_File_Context* file_ctx);

#[no_mangle]
pub extern fn sass_delete_file_context(ptr: *mut SassFileContext) {
    drop_raw_ptr(ptr);
}

#[no_mangle]
pub extern fn sass_file_context_get_context (file_ctx: *mut SassFileContext) -> *mut SassContext
{
    let mut ctx = ptr_to_ref(file_ctx);
    &mut ctx.context
}

//ADDAPI struct Sass_Options* ADDCALL sass_file_context_get_options (struct Sass_File_Context* file_ctx);
//ADDAPI void ADDCALL sass_file_context_set_options (struct Sass_File_Context* file_ctx, struct Sass_Options* opt);
