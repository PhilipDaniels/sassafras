// FROM: src/sass.hpp       // TODO This is not internal, not really part of the external API.

use super::Sass_Output_Style;

// input behaviours
#[derive(Debug)]
pub enum Sass_Input_Style {
    SASS_CONTEXT_NULL,
    SASS_CONTEXT_FILE,
    SASS_CONTEXT_DATA,
    SASS_CONTEXT_FOLDER
}

// sass config options structure
#[derive(Default, Debug)]
pub struct Sass_Inspect_Options {
    // Output style for the generated css code
    pub output_style: Sass_Output_Style,

    // Precision for fractional numbers
    pub precision: i32
}

impl Sass_Inspect_Options {
    // Defaults = Nested, 5.
    pub fn new(style: Sass_Output_Style, precision: i32) -> Self {
        Sass_Inspect_Options { output_style: style, precision }
    }
}

#[derive(Default, Debug)]
pub struct Sass_Output_Options {
    pub inspect_options: Sass_Inspect_Options,
    // String to be used for indentation
    pub indent: String,
    // String to be used to for line feeds
    pub linefeed: String,
    // Emit comments in the generated CSS indicating
    // the corresponding source line.
    pub source_comments: bool
}

// sass config options structure
impl Sass_Output_Options {
    // Defaults: indent = two spaces, linefeed = '\n', source_comments = false
    pub fn new_from_options<S>(opt: Sass_Inspect_Options, indent: S, linefeed: S, source_comments: bool) -> Self
        where S: Into<String>
    {
        Sass_Output_Options {
            inspect_options: opt,
            indent: indent.into(),
            linefeed: linefeed.into(),
            source_comments
        }
    }

    // Defaults: indent = two spaces, linefeed = '\n', source_comments = false
    pub fn new<S>(style: Sass_Output_Style, precision: i32, indent: S, linefeed: S, source_comments: bool) -> Self
        where S: Into<String>
    {
        Sass_Output_Options {
            inspect_options: Sass_Inspect_Options::new(style, precision),
            indent: indent.into(),
            linefeed: linefeed.into(),
            source_comments
        }
    }
}
