use std::path::{Path, PathBuf};
use std::os::raw::c_char;
use std::ffi::{CStr, CString, OsStr, OsString};

/// Converts a raw pointer into a Rust reference.
/// ptr must be non-null.
pub fn ptr_to_ref<'a, T>(ptr: *mut T) -> &'a mut T {
    assert!(!ptr.is_null());
    unsafe { &mut *ptr }
}

/// Converts a raw pointer into a Rust value.
/// ptr must be non-null.
pub fn ptr_to<T>(ptr: *mut T) -> T {
    assert!(!ptr.is_null());
    unsafe { *Box::from_raw(ptr) }
}

/// Takes ownership of a Rust value, moves it onto the heap,
/// then returns a raw pointer to the heap value. Does not
/// drop the value at any point. It is caller responsibility
/// to re-take ownership and drop the value at a later point.
/// See also: drop_raw_ptr.
pub fn box_to_raw_ptr<T>(value: T) -> *mut T {
    // Box::new() places the struct onto the heap, then into_raw() ensures it is not cleaned up.
    Box::into_raw(Box::new(value))
}

/// Converts a raw pointer into a boxed value and then drops the box
/// and the value within it. It is safe to call this method on a
/// null pointer. See also: box_to_raw_ptr.
pub fn drop_raw_ptr<T>(ptr: *mut T) {
    if !ptr.is_null() {
        unsafe {
            // from_raw() constructs a box, which is then automatically dropped
            // at the end of the scope, calling drop() on the struct within it.
            Box::from_raw(ptr);
        }
    }
}

/// Converts a C string pointer into a slice. The slice is expected to have the
/// trailing null byte (you will get a runtime error if it doesn't).
pub fn c_char_ptr_to_cstr<'a>(ptr: *const c_char) -> &'a CStr {
    assert!(!ptr.is_null());
    unsafe { CStr::from_ptr(ptr) }
}

/// Converts a C string pointer into an owned CString, including a trailing null byte.
pub fn c_char_ptr_to_cstring(ptr: *const c_char) -> CString {
    let slice = c_char_ptr_to_cstr(ptr);
    CString::from(slice)
}

/// Converts a C string pointer to a byte slice including a trailing null byte.
pub fn c_char_ptr_to_bytes_with_nul<'a>(ptr: *const c_char) -> &'a [u8] {
    let s = c_char_ptr_to_cstr(ptr);
    s.to_bytes_with_nul()
}

/// Converts a C string pointer to a vector, including a trailing null byte.
pub fn c_char_ptr_to_vec(ptr: *const c_char) -> Vec<u8> {
    let s = c_char_ptr_to_bytes_with_nul(ptr);
    s.to_vec()
}

/// Converts a C string pointer to an OsString, including a trailing null byte.
#[cfg(unix)]
fn c_char_ptr_to_osstring(ptr: *const c_char) -> OsString {
    use std::os::unix::ffi::OsStrExt;

    //let slice = c_char_ptr_to_cstr(ptr);
    let bytes = c_char_ptr_to_bytes_with_nul(ptr);
    OsString::from(OsStr::from_bytes(bytes))
}

/// Converts a C string pointer to an OsString, including a trailing null byte.
#[cfg(windows)]
pub fn c_char_ptr_to_osstring(ptr: *const c_char) -> OsString {
    let bytes = c_char_ptr_to_vec(ptr);
    OsString::from_vec(bytes)
}

/// Converts a C string pointer to a PathBuf. The PathBuf should have a trailing null byte.
pub fn c_char_ptr_to_pathbuf(ptr: *const c_char) -> PathBuf {
    let osstr = c_char_ptr_to_osstring(ptr);
    PathBuf::from(osstr)
}

/// Converts a path to a C string pointer by getting a pointer to the inner OsString buffer.
pub fn path_to_c_char_ptr<P: AsRef<Path>>(path: P) -> *const c_char {
    let path = path.as_ref();
    let os_str = path.as_os_str();
    os_str as *const OsStr as *const c_char
}








// Unknown below here.

pub fn c_char_ptr_to_string(ptr: *const c_char) -> String {
    let slice = c_char_ptr_to_cstr(ptr);
    slice.to_string_lossy().into_owned()
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


/* Can't do this unfortunately, both types are not in this crate!
   Could workaround this using the newtype pattern.
impl From<* const c_char> for PathBuf {
    fn from(ptr: *const c_char) -> Self {
        let osstr = c_char_ptr_to_osstring(ptr);
        PathBuf::from(osstr)
    }
}
.*/