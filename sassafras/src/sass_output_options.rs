use sass_inspect_options::Sass_Inspect_Options;

// Different render styles
#[derive(Debug)]
#[repr(C)]
pub enum Sass_Output_Style {
    SASS_STYLE_NESTED,
    SASS_STYLE_EXPANDED,
    SASS_STYLE_COMPACT,
    SASS_STYLE_COMPRESSED,
    // only used internaly
    SASS_STYLE_INSPECT,
    SASS_STYLE_TO_SASS
}

impl Default for Sass_Output_Style {
    fn default() -> Self {
        Sass_Output_Style::SASS_STYLE_NESTED
    }
}

#[derive(Default, Debug)]
#[repr(C)]
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
    pub fn new<S>(style: Sass_Output_Style, precision: u8, indent: S, linefeed: S, source_comments: bool) -> Self
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
