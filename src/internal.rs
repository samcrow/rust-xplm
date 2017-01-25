use std::os::raw::c_char;
use std::ffi::CString;
use std::ptr;
use std::panic::{self, UnwindSafe};
use std::process;
use std::fmt::Display;

use xplm_sys;

/// Copies up to 256 bytes (including null termination) to
/// the provided destination. If the provided source string is too long, it will be
/// truncated.
pub unsafe fn copy_to_c_buffer(mut src: String, dest: *mut c_char) {
    // Truncate to 255 bytes (256 including the null terminator)
    src.truncate(255);
    let src_c = CString::new(src)
        .unwrap_or(CString::new("<invalid>").unwrap());
    let src_c_length = src_c.to_bytes_with_nul().len();
    debug_assert!(src_c_length <= 256);
    ptr::copy_nonoverlapping(src_c.as_ptr(), dest, src_c_length);
}


/// Executes the provided function, and aborts the process if it panics
///
/// context: A string that describes the surrounding code, used in error messages
pub fn run_or_abort<F: FnOnce() -> R + UnwindSafe, R>(context: &str, function: F) -> R {
    panic::catch_unwind(function).unwrap_or_else(|e| {
        // Try to output error information
        let error_info = match e.downcast_ref::<&Display>() {
            Some(displayable) => format!("{}", displayable),
            None => "[Unknown error]".into(),
        };
        let debug_text = format!("Rust XPLM panic: {} in {}\nAborting\n", error_info, context);
        let debug_text = CString::new(debug_text)
            .unwrap_or(CString::new("Rust XPLM panic: Invalid message\nAborting\n").unwrap());
        unsafe { xplm_sys::XPLMDebugString(debug_text.as_ptr()) };
        process::exit(-1);
    })
}
