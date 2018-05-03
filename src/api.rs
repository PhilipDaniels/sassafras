/*
/// This module contains items that are part of the public libsass API
/// (i.e. are from include/sass/...)

pub enum SassOutputStyle {
    Nested,
    Expanded,
    Compact,
    Compressed,
    Inspect,
    ToSass
}

// From src/sass.hpp
pub struct SassInspectOptions {
    output_style: SassOutputStyle,
    precision: i32
}

// From src/sass.hpp
pub struct SassOutputOptions {
    inspect_options: SassInspectOptions
}

// From src/sass_context.hpp
pub struct SassOptions {
    output_options: SassOutputOptions
}

// From src/sass_context.hpp
// base for all contexts
pub struct SassContext {
    options: SassOptions
}

// From src/sass_context.hpp
// struct for file compilation
pub struct SassFileContext {
    context: SassContext
    // no additional fields required
    // input_path is already on options
}

// From src/sass_context.hpp
// struct for data compilation
pub struct SassDataContext {
    context: SassContext,
    // provided source string
    source_string: String,
    srcmap_string: String
}

// From src/sass_context.hpp
pub struct SassCompiler {
    state: SassCompilerState,
}

*/