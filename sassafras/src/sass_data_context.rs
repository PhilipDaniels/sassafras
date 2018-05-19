use sass_context::Sass_Context;

// struct for data compilation
#[repr(C)]
pub struct Sass_Data_Context {
    context: Sass_Context,
    source_string: String,
    srcmap_string: String
}
