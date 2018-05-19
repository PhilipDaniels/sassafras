use sass_context::Sass_Context;

// struct for file compilation
#[repr(C)]
pub struct Sass_File_Context {
    context: Sass_Context
    // no additional fields required
    // input_path is already on options
}
