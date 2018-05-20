use sass_output_options::SassOutputStyle;

// sass config options structure
#[derive(Default, Debug)]
#[repr(C)]
pub struct SassInspectOptions {
    // Output style for the generated css code
    pub output_style: SassOutputStyle,

    // Precision for fractional numbers
    pub precision: u8
}

impl SassInspectOptions {
    // Defaults = Nested, 5.
    pub fn new(style: SassOutputStyle, precision: u8) -> Self {
        SassInspectOptions { output_style: style, precision }
    }
}
