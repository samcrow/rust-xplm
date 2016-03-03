// Copyright (c) 2015 rust-xplm developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

//!
//! The `xplm` crate provides a convenient interface to the X-Plane plugin SDK.
//!

extern crate xplm_sys;
extern crate libc;

/// Functionality for reading, writing, and creating datarefs
pub mod data;
/// Functionality for finding, creating, and executing commands
pub mod command;
/// Flight loop callbacks
pub mod flight_loop;
/// SDK feature querying and enabling
pub mod features;
/// Terrain probing
pub mod terrain;
/// Position types
pub mod position;

/// User interface elements
pub mod ui;
/// Provides access to the navigation database
pub mod nav;
/// Radio frequency representation
pub mod frequency;
/// OpenGL-related functionality
pub mod graphics;


/// Foreign function interface utilities
mod ffi;

/// Writes a message to the X-Plane log.txt file
pub fn debug(message: &str) {
    use std::ffi::CString;
    use xplm_sys::utilities::XPLMDebugString;

    match CString::new(message) {
        Ok(message_c) => unsafe { XPLMDebugString(message_c.as_ptr()) },
        Err(_) => unsafe { XPLMDebugString(b"xplm::debug: Provided string not valid".as_ptr() as *const libc::c_char) },
    }
}

/// Enables the logging of plugin-related errors to the log.txt file
pub fn enable_debug_logging() {
    unsafe { xplm_sys::utilities::XPLMSetErrorCallback(Some(log_callback)) };
}

unsafe extern "C" fn log_callback(message: *const ::libc::c_char) {
    use std::ffi::CStr;
    debug(&format!("XPLM error: {}\n", CStr::from_ptr(message).to_string_lossy().into_owned()));
}

/// Finds a symbol in the set of currently loaded libraries
pub fn find_symbol(name: &str) -> *mut ::libc::c_void {
    use std::ffi::CString;
    use std::ptr;
    use xplm_sys::utilities::XPLMFindSymbol;

    match CString::new(name) {
        Ok(name_c) => unsafe { XPLMFindSymbol(name_c.as_ptr()) },
        Err(_) => ptr::null_mut(),
    }
}
