
#![deny(missing_docs,
        trivial_casts)]

//! Bindings to the X-Plane plugin SDK
//!

extern crate xplm_sys;
#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate lazy_static;

use std::ffi::CString;

/// FFI utilities
mod ffi;
/// Plugin macro
mod plugin_macro;

#[doc(hidden)]
/// Utilities that the xplane_plugin macro-generated code uses
///
// These must be public so that code in other crates can access them
pub mod internal;

/// Plugin creation and management
pub mod plugin;
/// Flight loop callbacks
pub mod flight_loop;
/// Commands
pub mod command;
/// Datarefs
pub mod data;
// Error detection
// Not finished
// pub mod error;

/// Writes a message to the X-Plane log.txt file
///
/// No line terminator is added.
pub fn debug(message: &str) {
    use xplm_sys::XPLMDebugString;
    match CString::new(message) {
        Ok(message_c) => unsafe { XPLMDebugString(message_c.as_ptr()) },
        Err(_) => unsafe {
            XPLMDebugString(b"[xplm] Invalid debug message\n\0".as_ptr() as *const i8)
        },
    }
}
