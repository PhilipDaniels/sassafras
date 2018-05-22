use std::path::{Path, PathBuf};
use std::os::raw::c_char;
use std::ffi::{CStr, CString, OsStr, OsString};

pub fn c_char_ptr_to_cstr<'a>(ptr: *const c_char) -> &'a CStr {
    assert!(!ptr.is_null());
    unsafe { CStr::from_ptr(ptr) }
}

pub fn c_char_ptr_to_cstring(ptr: *const c_char) -> CString {
    let slice = c_char_ptr_to_cstr(ptr);
    CString::from(slice)
}

pub fn c_char_ptr_to_string(ptr: *const c_char) -> String {
    let slice = c_char_ptr_to_cstr(ptr);
    slice.to_string_lossy().into_owned()
}

pub fn c_char_ptr_to_vec(ptr: *const c_char) -> Vec<u8> {
    let s = c_char_ptr_to_cstring(ptr);
    s.to_bytes().to_vec()
}

#[cfg(unix)]
fn c_char_ptr_to_osstring(ptr: *const c_char) -> OsString {
    use std::os::unix::ffi::OsStrExt;

    let slice = c_char_ptr_to_cstr(ptr);
    let bytes = slice.to_bytes();
    OsString::from(OsStr::from_bytes(bytes))
}

#[cfg(windows)]
pub fn c_char_ptr_to_osstring(ptr: *const c_char) -> OsString {
    let bytes = c_char_ptr_to_vec(ptr);
    OsString::from_vec(bytes)
}

pub fn c_char_ptr_to_pathbuf(ptr: *const c_char) -> PathBuf {
    let osstr = c_char_ptr_to_osstring(ptr);
    PathBuf::from(osstr)
}

#[cfg(unix)]
pub fn path_to_cstring(path: &Path) -> CString {
    use std::os::unix::ffi::OsStrExt;

    CString::new(path.as_os_str().as_bytes()).expect("Conversion to work")
}

#[cfg(windows)]
pub fn path_to_cstring(path: &Path) -> CString {
    match path.to_str() {
        Some(s) => CString::new(s),
        None => panic!("Could not convert path to cstring")
    }
}
