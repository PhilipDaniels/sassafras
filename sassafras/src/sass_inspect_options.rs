use sass_output_options::Sass_Output_Style;

// sass config options structure
#[derive(Default, Debug)]
#[repr(C)]
pub struct Sass_Inspect_Options {
    // Output style for the generated css code
    pub output_style: Sass_Output_Style,

    // Precision for fractional numbers
    pub precision: u8
}

impl Sass_Inspect_Options {
    // Defaults = Nested, 5.
    pub fn new(style: Sass_Output_Style, precision: u8) -> Self {
        Sass_Inspect_Options { output_style: style, precision }
    }
}
