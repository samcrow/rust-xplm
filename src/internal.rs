use std::os::raw::c_char;
use std::ffi::CString;
use std::ptr;
use std::panic::{self, UnwindSafe};
use std::process;
use std::fmt::Display;

use xplm_sys;

/// Copies bytes from a str into a C memory location
pub unsafe fn rstrcpy(dest: *mut ::std::os::raw::c_char, src: &str) {
    match CString::new(src) {
        Ok(src_c) => {
            ptr::copy_nonoverlapping(src_c.as_ptr(), dest, src_c.to_bytes_with_nul().len());
        }
        Err(_) => {
            let message = b"invalid\0";
            ptr::copy_nonoverlapping(message.as_ptr() as *const c_char, dest, message.len());
        }
    }
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
