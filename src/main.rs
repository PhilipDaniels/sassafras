extern crate built;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate structopt;
extern crate sassafras;

use structopt::StructOpt;
use std::path::PathBuf;
use sassafras::c_api;

// TODO: Both of these enums cause a warning to be emitted.

arg_enum! {
    #[derive(Debug)]
    enum SourceMapEmission
    {
        No,
        Auto,
        Inline
    }
}

arg_enum! {
    #[derive(Debug)]
    enum OutputStyles
    {
        Compressed,
        Compact,
        Expanded,
        Nested
    }
}

#[derive(StructOpt, Debug)]
struct Arguments {
    /// Read input from standard input instead of an input file.
    #[structopt(short = "s", long = "stdin")]
    from_stdin: bool,

    /// Output style.
    #[structopt(short = "t", long = "style", default_value = "Nested", raw(possible_values = "&OutputStyles::variants()", case_insensitive = "true"))]
    output_style: OutputStyles,

    /// Emit comments showing original line numbers.
    #[structopt(short = "l", long = "line-numbers", raw(aliases = r#"&["line-comments"]"#))]
    line_numbers: bool,

    /// Set Sass import path.
    #[structopt(short = "I", long = "load-path", parse(from_os_str))]
    load_path: Option<PathBuf>,

    /// Set path to autoload plugins.
    #[structopt(short = "P", long = "plugin-path", parse(from_os_str))]
    plugin_path: Option<PathBuf>,

    /// Set additional extensions to use when resolving imports.
    #[structopt(short = "E", long = "import-extension")]
    import_extension: Option<String>,

    /// Emit source map.
    #[structopt(short = "m", long = "sourcemap", default_value = "no", raw(possible_values = "&SourceMapEmission::variants()", case_insensitive = "true"))]
    emit_sourcemap: SourceMapEmission,

    /// Omits the source map url comment.
    #[structopt(short = "M", long = "omit-map-comment")]
    omit_sourcemap_comment: bool,

    /// Sets the precision for numbers.
    #[structopt(short = "p", long = "precision", default_value = "5")]
    precision: usize,  // TODO: What is the permissible range here.

    /// Treat input as indented syntax.
    #[structopt(short = "a", long = "sass")]
    input_is_indented: bool,

    /// Input file.
    #[structopt(name = "INPUT", parse(from_os_str))]
    input_file: Option<PathBuf>,

    /// Output file.
    #[structopt(name = "OUTPUT", parse(from_os_str))]
    output_file: Option<PathBuf>,
}

fn main() {
    let args = Arguments::from_args();
    // The program terminates in the line above if arguments are invalid.

    // sassc uses the C API to drive libsass.
    // For the sake of testing and porting, we will do the same for now,
    // so this is not idiomatic Rust.
    let mut options = c_api::sass_make_options();
    c_api::sass_option_set_output_style(&mut options, c_api::Sass_Output_Style::SASS_STYLE_NESTED);
    c_api::sass_option_set_precision(&mut options, 5);

    if let Some(ext) = args.import_extension {
        c_api::sass_option_push_import_extension(&mut options, ext);
    }

    println!("{:#?}", options);
}
