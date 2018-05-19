use sass_context::Sass_Context;

// Compiler states
#[repr(C)]
pub enum Sass_Compiler_State {
    SASS_COMPILER_CREATED,
    SASS_COMPILER_PARSED,
    SASS_COMPILER_EXECUTED
}

// link c and cpp context
#[repr(C)]
pub struct Sass_Compiler {
    // progress status
    state: Sass_Compiler_State,
    // original c context
    c_ctx: Sass_Context,
    // Sass::Context
    //cpp_ctx: Context,
    // Sass::Block
    //root: Block_Obj
}
