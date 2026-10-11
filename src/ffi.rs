//! C-ABI Foreign Function Interface (FFI) for NVDA and foreign language runtimes.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use crate::to_hindi_phonetic;

/// Converts a null-terminated UTF-8 C string of Romanized Sylheti to Hindi Devanagari phonetics.
///
/// # Safety
/// The caller must ensure `input` is a valid, null-terminated C string pointer.
/// The returned pointer must be freed with `sylheti_free_string`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sylheti_convert_to_hindi(input: *const c_char) -> *mut c_char {
    if input.is_null() {
        return std::ptr::null_mut();
    }
    let c_str = unsafe { CStr::from_ptr(input) };
    let r_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    let converted = to_hindi_phonetic(r_str);
    match CString::new(converted) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Frees a C string allocated by `sylheti_convert_to_hindi`.
///
/// # Safety
/// The pointer must have been returned by this library, and not previously freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sylheti_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}
