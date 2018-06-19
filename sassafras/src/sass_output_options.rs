use sass_inspect_options::SassInspectOptions;
use c_api_helpers::ptr_to_ref;

// Different render styles
#[derive(Debug)]
#[repr(C)]
pub enum SassOutputStyle {
    Nested,
    Expanded,
    Compact,
    Compressed,
    // only used internaly
    Inspect,
    ToSass
}

impl Default for SassOutputStyle {
    fn default() -> Self {
        SassOutputStyle::Nested
    }
}

#[derive(Default, Debug)]
#[repr(C)]
pub struct SassOutputOptions {
    pub inspect_options: SassInspectOptions,
    // String to be used for indentation
    pub indent: String,
    // String to be used to for line feeds
    pub linefeed: String,
    // Emit comments in the generated CSS indicating
    // the corresponding source line.
    pub source_comments: bool
}

// sass config options structure
impl SassOutputOptions {
    // Defaults: indent = two spaces, linefeed = '\n', source_comments = false
    pub fn new_from_options<S>(opt: SassInspectOptions, indent: S, linefeed: S, source_comments: bool) -> Self
        where S: Into<String>
    {
        SassOutputOptions {
            inspect_options: opt,
            indent: indent.into(),
            linefeed: linefeed.into(),
            source_comments
        }
    }

    // Defaults: indent = two spaces, linefeed = '\n', source_comments = false
    pub fn new<S>(style: SassOutputStyle, precision: u8, indent: S, linefeed: S, source_comments: bool) -> Self
        where S: Into<String>
    {
        SassOutputOptions {
            inspect_options: SassInspectOptions::new(style, precision),
            indent: indent.into(),
            linefeed: linefeed.into(),
            source_comments
        }
    }
}
