//!
//! The `xplm` crate provides a convenient interface to the X-Plane plugin SDK.
//!

extern crate xplm_sys;
extern crate libc;

/// Common definitions for datarefs
pub mod data;
/// Functionality for reading and writing datarefs
pub mod dataref;
/// Functionality for creating datarefs
pub mod owned_data;
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
// /// Menu bar functionality (currently incomplete)
// pub mod menu;

/// Provides access to the navigation database
pub mod nav;
/// Radio frequency representation
pub mod frequency;

/// Foreign function interface utilities
mod ffi;

/// Writes a message to the X-Plane log.txt file
pub fn debug(message: &str) {
    use std::ffi::CString;
    use xplm_sys::utilities::XPLMDebugString;

    match CString::new(message) {
        Ok(message_c) => unsafe { XPLMDebugString(message_c.as_ptr()) },
        Err(_) => unsafe { XPLMDebugString(b"xplm::debug: Provided string not valid".as_ptr() as *const i8) },
    }
}
