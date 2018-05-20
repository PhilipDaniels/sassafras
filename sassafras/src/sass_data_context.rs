use sass_context::Sass_Context;

// struct for data compilation
#[repr(C)]
pub struct Sass_Data_Context {
    context: Sass_Context,
    source_string: String,
    srcmap_string: String
}


//ADDAPI struct Sass_Options* ADDCALL sass_data_context_get_options (struct Sass_Data_Context* data_ctx);
//ADDAPI void ADDCALL sass_data_context_set_options (struct Sass_Data_Context* data_ctx, struct Sass_Options* opt);

//ADDAPI struct Sass_Context* ADDCALL sass_data_context_get_context (struct Sass_Data_Context* data_ctx);
//ADDAPI void ADDCALL sass_delete_data_context (struct Sass_Data_Context* ctx);
//ADDAPI struct Sass_Compiler* ADDCALL sass_make_data_compiler (struct Sass_Data_Context* data_ctx);
//ADDAPI int ADDCALL sass_compile_data_context (struct Sass_Data_Context* ctx);
//ADDAPI struct Sass_Data_Context* ADDCALL sass_make_data_context (char* source_string);
