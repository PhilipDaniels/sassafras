#[allow(non_camel_case_types)]
extern crate built;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate structopt;
extern crate sassafras;
extern crate core;

use structopt::StructOpt;
use std::path::PathBuf;
use sassafras::sass_options::*;
use sassafras::sass_output_options::SassOutputStyle;
use std::path::Path;
use std::os::raw::c_char;
use std::ffi::OsString;
use std::ffi::CString;
use std::panic;
use sassafras::c_api_helpers::path_to_cstring;

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
    include_path: Option<PathBuf>,

    /// Set path to autoload plugins.
    #[structopt(short = "P", long = "plugin-path", parse(from_os_str))]
    plugin_path: Option<PathBuf>,

    /// Set additional extensions to use when resolving imports.
    #[structopt(short = "E", long = "import-extension", parse(from_os_str))]
    import_extension: Option<PathBuf>,

    /// Emit source map.
    #[structopt(short = "m", long = "sourcemap", default_value = "no", raw(possible_values = "&SourceMapEmission::variants()", case_insensitive = "true"))]
    emit_sourcemap: SourceMapEmission,

    /// Omits the source map url comment.
    #[structopt(short = "M", long = "omit-map-comment")]
    omit_sourcemap_comment: bool,

    /// Sets the precision for numbers.
    #[structopt(short = "p", long = "precision", default_value = "5")]
    precision: u8,  // TODO: What is the permissible range here.

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
    let options = sass_make_options();
    sass_option_set_output_style(options, SassOutputStyle::Nested);
    sass_option_set_precision(options, 5);
    sass_option_set_output_style(options, translate_output_style(args.output_style));
    sass_option_set_source_comments(options, args.line_numbers);
    sass_option_set_omit_source_map_url(options, args.omit_sourcemap_comment);
    sass_option_set_precision(options, args.precision);
    sass_option_set_is_indented_syntax_src(options, args.input_is_indented);

    if let Some(ext) = args.import_extension {
        let cstring = path_to_cstring(&ext);
        sass_option_push_import_extension(options, cstring.as_ptr());
    }

    if let Some(path) = args.include_path {
        let cstring = path_to_cstring(&path);
        sass_option_push_include_path(options, cstring.as_ptr());
    }

    if let Some(path) = args.plugin_path {
        let cstring = path_to_cstring(&path);
        sass_option_push_plugin_path(options, cstring.as_ptr());
    }

    let mut auto_source_map = false;
    let mut generate_source_map = false;

    match args.emit_sourcemap {
        SourceMapEmission::Auto => {
            auto_source_map = true;
            generate_source_map = true
        }
        SourceMapEmission::Inline => {
            sass_option_set_source_map_embed(options, true);
            generate_source_map = true
        }
        SourceMapEmission::No => {}
    }

    let mut result = 0;
    if !args.from_stdin {
        if generate_source_map {
            if args.output_file.is_some() {
                let mut src_map_name = args.output_file.clone().unwrap();
                src_map_name.push(".map");
                let cstring = path_to_cstring(&src_map_name);
                sass_option_set_source_map_file(options, cstring.as_ptr());
            } else {
                sass_option_set_source_map_embed(options, true);
            }
        } else {
            sass_option_set_source_map_embed(options, true);
        }

        // If output_file is None, we write to stdout.
        result = compile_file(options, args.input_file, args.output_file);
    } else {
        // If output_file is None, we write to stdout.
        result = compile_stdin(options, args.output_file);
    }

    sass_option_print(options);
    sass_delete_options(options);
}

fn translate_output_style(arg_style: OutputStyles) -> SassOutputStyle {
    match arg_style {
        OutputStyles::Compressed => SassOutputStyle::Compressed,
        OutputStyles::Compact => SassOutputStyle::Compact,
        OutputStyles::Expanded => SassOutputStyle::Expanded,
        OutputStyles::Nested => SassOutputStyle::Nested
    }
}


pub fn compile_file(options_ptr: *mut SassOptions, input_file: Option<PathBuf>, output_file: Option<PathBuf>) -> i32 {
//    int ret;
//    struct Sass_File_Context* ctx = sass_make_file_context(input_path);
//    struct Sass_Context* ctx_out = sass_file_context_get_context(ctx);
//    if (outfile) sass_option_set_output_path(options, outfile);
//    const char* srcmap_file = sass_option_get_source_map_file(options);
//    sass_option_set_input_path(options, input_path);
//    sass_file_context_set_options(ctx, options);
//
//    sass_compile_file_context(ctx);
//
//    ret = output(
//        sass_context_get_error_status(ctx_out),
//        sass_context_get_error_message(ctx_out),
//        sass_context_get_output_string(ctx_out),
//        outfile
//    );
//
//    if (ret == 0 && srcmap_file) {
//        ret = output(
//            sass_context_get_error_status(ctx_out),
//            sass_context_get_error_message(ctx_out),
//            sass_context_get_source_map_string(ctx_out),
//            srcmap_file
//        );
//    }
//
//    sass_delete_file_context(ctx);
//    return ret;

    0
}

pub fn compile_stdin(options_ptr: *mut SassOptions, output_file: Option<PathBuf>) -> i32 {
//    int ret;
//    struct Sass_Data_Context* ctx;
//    char buffer[BUFSIZE];
//    size_t size = 1;
//    char *source_string = malloc(sizeof(char) * BUFSIZE);
//
//    if(source_string == NULL) {
//        perror("Allocation failed");
//        #ifdef _WIN32
//        exit(ERROR_OUTOFMEMORY);
//        #else
//        exit(EX_OSERR); // system error (e.g., can't fork)
//        #endif
//    }
//
//    source_string[0] = '\0';
//
//    while(fgets(buffer, BUFSIZE, stdin)) {
//        char *old = source_string;
//        size += strlen(buffer);
//        source_string = realloc(source_string, size);
//        if(source_string == NULL) {
//            perror("Reallocation failed");
//            free(old);
//            #ifdef _WIN32
//            exit(ERROR_OUTOFMEMORY);
//            #else
//            exit(EX_OSERR); // system error (e.g., can't fork)
//            #endif
//        }
//        strcat(source_string, buffer);
//    }
//
//    if(ferror(stdin)) {
//        free(source_string);
//        perror("Error reading standard input");
//        #ifdef _WIN32
//        exit(ERROR_READ_FAULT); //
//        #else
//        exit(EX_IOERR); // input/output error
//        #endif
//    }
//
//    ctx = sass_make_data_context(source_string);
//    struct Sass_Context* ctx_out = sass_data_context_get_context(ctx);
//    sass_data_context_set_options(ctx, options);
//    sass_compile_data_context(ctx);
//    ret = output(
//        sass_context_get_error_status(ctx_out),
//        sass_context_get_error_message(ctx_out),
//        sass_context_get_output_string(ctx_out),
//        outfile
//    );
//    sass_delete_data_context(ctx);
//    return ret;

    0
}

fn output() -> i32 {
//    if (error_status) {
//        if (error_message) {
//            fprintf(stderr, "%s", error_message);
//        } else {
//            fprintf(stderr, "An error occured; no error message available.\n");
//        }
//        return 1;
//    } else if (output_string) {
//        if(outfile) {
//            FILE* fp = fopen(outfile, "wb");
//            if(!fp) {
//                perror("Error opening output file");
//                return 1;
//            }
//            if(fprintf(fp, "%s", output_string) < 0) {
//                perror("Error writing to output file");
//                fclose(fp);
//                return 1;
//            }
//            fclose(fp);
//        }
//            else {
//                #ifdef _WIN32
//                setmode(fileno(stdout), O_BINARY);
//                #endif
//                printf("%s", output_string);
//            }
//        return 0;
//    } else {
//        fprintf(stderr, "Unknown internal error.\n");
//        return 2;
//    }

    0
}