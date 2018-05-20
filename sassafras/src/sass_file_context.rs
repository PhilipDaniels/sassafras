use sass_context::Sass_Context;

// struct for file compilation
#[repr(C)]
pub struct Sass_File_Context {
    context: Sass_Context
    // no additional fields required
    // input_path is already on options
}

//// Create and initialize a specific context
//ADDAPI struct Sass_File_Context* ADDCALL sass_make_file_context (const char* input_path);

//// Call the compilation step for the specific context
//ADDAPI int ADDCALL sass_compile_file_context (struct Sass_File_Context* ctx);
//// Create a sass compiler instance for more control
//ADDAPI struct Sass_Compiler* ADDCALL sass_make_file_compiler (struct Sass_File_Context* file_ctx);
//

//// Release all memory allocated and also ourself
//ADDAPI void ADDCALL sass_delete_file_context (struct Sass_File_Context* ctx);
//
//// Getters for context from specific implementation
//ADDAPI struct Sass_Context* ADDCALL sass_file_context_get_context (struct Sass_File_Context* file_ctx);
//ADDAPI struct Sass_Options* ADDCALL sass_file_context_get_options (struct Sass_File_Context* file_ctx);
//ADDAPI void ADDCALL sass_file_context_set_options (struct Sass_File_Context* file_ctx, struct Sass_Options* opt);
