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


//
//// Resolve a file relative to last import or include paths in the sass option struct
//// find_file looks for the exact file name while find_include does a regular sass include
//ADDAPI char* ADDCALL sass_compiler_find_file (const char* path, struct Sass_Compiler* compiler);
//ADDAPI char* ADDCALL sass_compiler_find_include (const char* path, struct Sass_Compiler* compiler);

//// Getters for Sass_Compiler options
//ADDAPI enum Sass_Compiler_State ADDCALL sass_compiler_get_state(struct Sass_Compiler* compiler);
//ADDAPI struct Sass_Context* ADDCALL sass_compiler_get_context(struct Sass_Compiler* compiler);
//ADDAPI struct Sass_Options* ADDCALL sass_compiler_get_options(struct Sass_Compiler* compiler);
//ADDAPI size_t ADDCALL sass_compiler_get_import_stack_size(struct Sass_Compiler* compiler);
//ADDAPI Sass_Import_Entry ADDCALL sass_compiler_get_last_import(struct Sass_Compiler* compiler);
//ADDAPI Sass_Import_Entry ADDCALL sass_compiler_get_import_entry(struct Sass_Compiler* compiler, size_t idx);
//ADDAPI size_t ADDCALL sass_compiler_get_callee_stack_size(struct Sass_Compiler* compiler);
//ADDAPI Sass_Callee_Entry ADDCALL sass_compiler_get_last_callee(struct Sass_Compiler* compiler);
//ADDAPI Sass_Callee_Entry ADDCALL sass_compiler_get_callee_entry(struct Sass_Compiler* compiler, size_t idx);

//// Execute the different compilation steps individually
//// Usefull if you only want to query the included files
//ADDAPI int ADDCALL sass_compiler_parse(struct Sass_Compiler* compiler);
//ADDAPI int ADDCALL sass_compiler_execute(struct Sass_Compiler* compiler);
//
//// Release all memory allocated with the compiler
//// This does _not_ include any contexts or options
//ADDAPI void ADDCALL sass_delete_compiler(struct Sass_Compiler* compiler);

