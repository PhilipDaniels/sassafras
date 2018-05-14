#[allow(non_camel_case_types)]
#[allow(unused_variables)]
#[allow(dead_code)]

extern crate libc;

pub mod c_api;

#[no_mangle]
pub extern fn sass_make_options() -> *mut c_api::Sass_Options {
    let mut options = c_api::Sass_Options::default();
    options.init();
    Box::into_raw(Box::new(options))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
