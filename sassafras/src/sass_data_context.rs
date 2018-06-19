use sass_context::SassContext;
use c_api_helpers::ptr_to_ref;

// struct for data compilation
#[derive(Debug, Default)]
#[repr(C)]
pub struct SassDataContext {
    context: SassContext,
    source_string: String,
    srcmap_string: String,
}

#[no_mangle]
pub fn sass_data_context_print(msg: &str, ctx: *mut SassDataContext) {
    let context = ptr_to_ref(ctx);
    println!("{}{:#?}", msg, context);
}

//ADDAPI struct Sass_Options* ADDCALL sass_data_context_get_options (struct Sass_Data_Context* data_ctx);
//ADDAPI void ADDCALL sass_data_context_set_options (struct Sass_Data_Context* data_ctx, struct Sass_Options* opt);

//ADDAPI struct Sass_Context* ADDCALL sass_data_context_get_context (struct Sass_Data_Context* data_ctx);
//ADDAPI void ADDCALL sass_delete_data_context (struct Sass_Data_Context* ctx);
//ADDAPI struct Sass_Compiler* ADDCALL sass_make_data_compiler (struct Sass_Data_Context* data_ctx);
//ADDAPI int ADDCALL sass_compile_data_context (struct Sass_Data_Context* ctx);
//ADDAPI struct Sass_Data_Context* ADDCALL sass_make_data_context (char* source_string);
