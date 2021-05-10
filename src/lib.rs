#![deny(trivial_casts)]

//! Bindings to the X-Plane plugin SDK

extern crate xplm_sys;

use std::ffi::CString;

/// FFI utilities
mod ffi;
/// Path conversion
mod paths;
/// Plugin macro
mod plugin_macro;

/// Utilities that the xplane_plugin macro-generated code uses
///
mod internal;

/// Commands
pub mod command;
/// Datarefs
pub mod data;
/// Low-level drawing callbacks
pub mod draw;
/// Error detection
pub mod error;
/// SDK feature management
pub mod feature;
/// Flight loop callbacks
// TODO: Flight loop implementation that supports SDK 1.0
pub mod flight_loop;
/// 2D user interface geometry
pub mod geometry;
/// User interface menus
pub mod menu;
/// Plugin creation and management
pub mod plugin;
/// Relatively low-level windows
pub mod window;

/// Writes a message to the developer console and Log.txt file
///
/// No line terminator is added.
#[deprecated(note = "Please use the debug! or debugln! macro instead")]
pub fn debug<S: Into<String>>(message: S) {
    match CString::new(message.into()) {
        Ok(message_c) => unsafe { XPLMDebugString(message_c.as_ptr()) },
        Err(_) => unsafe {
            XPLMDebugString("[xplm] Invalid debug message\n\0".as_ptr() as *const _)
        },
    }
}

/// Re-export the signature of XPLMDebugString as it is needed in the debug macros.
/// By re-exporting we can avoid that users have to import xplm_sys into their plugin.
#[doc(hidden)]
pub use xplm_sys::XPLMDebugString;

/// Writes a message to the developer console and Log.txt file
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ({
        let formatted_string: String = std::fmt::format(std::format_args!($($arg)*));
        #[allow(unused_unsafe)] // Disable unnecessary unsafe block warning when embedded in unsafe function
        match std::ffi::CString::new(formatted_string) {
            Ok(c_str) => unsafe { $crate::XPLMDebugString(c_str.as_ptr()) },
            Err(_) => unsafe { $crate::XPLMDebugString("[xplm] Invalid debug message\n\0".as_ptr() as *const _) }
        }
    });
}

/// Writes a message to the developer console and Log.txt file, with a newline
#[macro_export]
#[allow(unused_unsafe)]
macro_rules! debugln {
    () => ($crate::debug!("\n"));
    ($($arg:tt)*) => ({
        let mut formatted_string: String = std::fmt::format(std::format_args!($($arg)*));
        formatted_string.push_str("\n");
        #[allow(unused_unsafe)] // Disable unnecessary unsafe block warning when embedded in unsafe function
        match std::ffi::CString::new(formatted_string) {
            Ok(c_str) => unsafe { $crate::XPLMDebugString(c_str.as_ptr()) },
            Err(_) => unsafe { $crate::XPLMDebugString("[xplm] Invalid debug message\n\0".as_ptr() as *const _) }
        }
    });
}

/// Attempts to locate a symbol. If it exists, returns a pointer to it
pub fn find_symbol<S: Into<String>>(name: S) -> *mut std::os::raw::c_void {
    use std::ptr;
    match std::ffi::CString::new(name.into()) {
        Ok(name_c) => unsafe { xplm_sys::XPLMFindSymbol(name_c.as_ptr()) },
        Err(_) => ptr::null_mut(),
    }
}
