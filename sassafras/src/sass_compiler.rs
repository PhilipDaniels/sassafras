use sass_context::SassContext;
use c_api_helpers::ptr_to_ref;
use sass_options::SassOptions;

// Compiler states
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub enum SassCompilerState {
    Created,
    Parsed,
    Executed
}

// link c and cpp context
#[derive(Debug)]
#[repr(C)]
pub struct SassCompiler {
    // progress status
    state: SassCompilerState,
    // original c context
    c_ctx: SassContext,
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

#[no_mangle]
pub extern fn sass_compiler_get_state(compiler: *mut SassCompiler) -> SassCompilerState {
    let compiler = ptr_to_ref(compiler);
    compiler.state
}

#[no_mangle]
pub extern fn sass_compiler_get_context(compiler: *mut SassCompiler) -> *mut SassContext {
    let compiler = ptr_to_ref(compiler);
    &mut compiler.c_ctx
}

#[no_mangle]
pub extern fn sass_compiler_get_options(compiler: *mut SassCompiler) -> *mut SassOptions {
    let compiler = ptr_to_ref(compiler);
    &mut compiler.c_ctx.options
}

#[no_mangle]
pub extern fn sass_compiler_get_stack_size(compiler: *mut SassCompiler) -> usize {
    let compiler = ptr_to_ref(compiler);
    //compiler.cpp_ctx..import_stack.size()
    0
}

#[no_mangle]
pub extern fn sass_compiler_get_last_import(compiler: *mut SassCompiler) -> usize {
    let compiler = ptr_to_ref(compiler);
    //compiler.cpp_ctx.import_stack.back() // the last element, should be a SassImportEntry,
    // which is not defined yet. It is a function pointer.
    // SassImportEntry
    0
}

#[no_mangle]
pub extern fn sass_compiler_get_import_entry(compiler: *mut SassCompiler) -> usize {
    let compiler = ptr_to_ref(compiler);
    // return compiler->cpp_ctx->import_stack[idx]; // SassImportEntry
    0
}

#[no_mangle]
pub extern fn sass_compiler_get_callee_stack_size(compiler: *mut SassCompiler) -> usize {
    let compiler = ptr_to_ref(compiler);
    // return compiler->cpp_ctx->callee_stack.size();
    0
}

#[no_mangle]
pub extern fn sass_compiler_get_last_callee(compiler: *mut SassCompiler) -> usize {
    // should return a SassCalleeEntry
    let compiler = ptr_to_ref(compiler);
    // return &compiler->cpp_ctx->callee_stack.back();
    0
}

#[no_mangle]
pub extern fn sass_compiler_get_callee_entry(compiler: *mut SassCompiler, idx: usize) -> usize {
    // should return a SassCalleeEntry
    let compiler = ptr_to_ref(compiler);
    // return &compiler->cpp_ctx->callee_stack[idx];
    0
}

/*
//// Execute the different compilation steps individually
//// Usefull if you only want to query the included files
  int ADDCALL sass_compiler_parse(struct Sass_Compiler* compiler)
  {
    if (compiler == 0) return 1;
    if (compiler->state == SASS_COMPILER_PARSED) return 0;
    if (compiler->state != SASS_COMPILER_CREATED) return -1;
    if (compiler->c_ctx == NULL) return 1;
    if (compiler->cpp_ctx == NULL) return 1;
    if (compiler->c_ctx->error_status)
      return compiler->c_ctx->error_status;
    // parse the context we have set up (file or data)
    compiler->root = sass_parse_block(compiler);
    // success
    return 0;
  }
*/

/*
  int ADDCALL sass_compiler_execute(struct Sass_Compiler* compiler)
  {
    if (compiler == 0) return 1;
    if (compiler->state == SASS_COMPILER_EXECUTED) return 0;
    if (compiler->state != SASS_COMPILER_PARSED) return -1;
    if (compiler->c_ctx == NULL) return 1;
    if (compiler->cpp_ctx == NULL) return 1;
    if (compiler->root.isNull()) return 1;
    if (compiler->c_ctx->error_status)
      return compiler->c_ctx->error_status;
    compiler->state = SASS_COMPILER_EXECUTED;
    Context* cpp_ctx = compiler->cpp_ctx;
    Block_Obj root = compiler->root;
    // compile the parsed root block
    try { compiler->c_ctx->output_string = cpp_ctx->render(root); }
    // pass catched errors to generic error handler
    catch (...) { return handle_errors(compiler->c_ctx) | 1; }
    // generate source map json and store on context
    compiler->c_ctx->source_map_string = cpp_ctx->render_srcmap();
    // success
    return 0;
  }
*/

/*
//// Release all memory allocated with the compiler
//// This does _not_ include any contexts or options
void ADDCALL sass_delete_compiler (struct Sass_Compiler* compiler)
  {
    if (compiler == 0) {
      return;
    }
    Context* cpp_ctx = compiler->cpp_ctx;
    if (cpp_ctx) delete(cpp_ctx);
    compiler->cpp_ctx = NULL;
    compiler->c_ctx = NULL;
    compiler->root = NULL;
    free(compiler);
  }
*/