
#![deny(missing_docs,
        trivial_casts)]

//! Bindings to the X-Plane plugin SDK
//!

extern crate xplm_sys;
#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate lazy_static;

#[cfg(all(target_os = "macos", not(feature = "xplm210")))]
extern crate hfs_paths;

use std::ffi::CString;

/// FFI utilities
mod ffi;
/// Plugin macro
mod plugin_macro;
/// Path conversion
mod paths;

#[doc(hidden)]
/// Utilities that the xplane_plugin macro-generated code uses
///
// These must be public so that code in other crates can access them
pub mod internal;

/// Plugin creation and management
pub mod plugin;
/// Flight loop callbacks
#[cfg(feature = "xplm210")]
// TODO: Flight loop implementation that supports SDK 1.0
pub mod flight_loop;
/// Commands
#[cfg(feature = "xplm200")]
pub mod command;
/// Datarefs
pub mod data;
/// Error detection
#[cfg(feature = "xplm200")]
pub mod error;
/// SDK feature management
#[cfg(feature = "xplm200")]
pub mod feature;
/// User interface menus
#[cfg(feature = "xplm200")]
pub mod menu;
/// Low-level drawing
pub mod draw;

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
