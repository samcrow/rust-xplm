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
/// Inter-plugin communication
pub mod ipc;

/// Foreign function interface utilities
mod ffi;

use std::ffi::CString;

/// Writes a message to the X-Plane log.txt file
pub fn debug(message: &str) {
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

/// The error callback provided to X-Plane that receives error messages
unsafe extern "C" fn log_callback(message: *const ::libc::c_char) {
    use std::ffi::CStr;
    debug(&format!("XPLM error: {}\n", CStr::from_ptr(message).to_string_lossy().into_owned()));
}

/// Finds a symbol in the set of currently loaded libraries
pub fn find_symbol(name: &str) -> *mut ::libc::c_void {
    use std::ptr;
    use xplm_sys::utilities::XPLMFindSymbol;

    match CString::new(name) {
        Ok(name_c) => unsafe { XPLMFindSymbol(name_c.as_ptr()) },
        Err(_) => ptr::null_mut(),
    }
}

/// Stores information about a plugin that is provided to X-Plane
pub struct PluginInfo {
    /// The plugin name
    pub name: &'static str,
    /// The plugin's signature, in reverse DNS format
    pub signature: &'static str,
    /// A description of the plugin
    pub description: &'static str,
}


/// The trait that all plugins should implement
pub trait Plugin : Sized {
    /// Called when X-Plane loads this plugin
    ///
    /// On success, returns a plugin object.
    ///
    /// On failure, returns an error.
    fn start() -> Result<Self, Box<std::error::Error>>;
    /// Called when the plugin is enabled
    fn enable(&mut self);
    /// Called when the plugin is disabled
    fn disable(&mut self);

    /// Returns information on this plugin
    fn info() -> PluginInfo;

    /// Called when a plugin sends a message to this plugin
    ///
    /// The message number and the plugin that sent the message are provided. This method is not
    /// called when X-Plane sends a message.
    fn message_from_plugin(&mut self, message: i32, from: ipc::Plugin);

    /// Called when X-Plane sends a message to this plugin
    ///
    /// The message is provided.
    fn message_from_xplane(&mut self, message: ipc::XPlaneMessage);

    // Called when the plugin is stopped
    ///
    /// The plugin will be droped after this function is called.
    fn stop(&mut self);
}

/// Creates an X-Plane plugin
///
/// Provide the type name of your plugin struct. The callbacks that X-Plane uses will be created.
///
#[macro_export]
macro_rules! xplane_plugin {
    ($plugin_type: ty) => (
        use xplm::Plugin;
        type PluginType = $plugin_type;
        type PluginPtr = *mut PluginType;
        // The plugin
        static mut PLUGIN: PluginPtr = 0 as PluginPtr;

        #[no_mangle]
        pub extern fn plugin_info() -> xplm::PluginInfo {
            PluginType::info()
        }
        #[no_mangle]
        pub unsafe extern fn plugin_start() -> Result<(), ()> {
            // Create the plugin, temporarily, on the stack
            let plugin_option = PluginType::start();
            match plugin_option {
                Ok(plugin) => {
                    // Allocate storage
                    PLUGIN = Box::into_raw(Box::new(plugin));
                    Ok(())
                },
                // TODO: Error reporting
                Err(_) => Err(()),
            }
        }
        #[no_mangle]
        pub unsafe extern fn plugin_enable() {
            (*PLUGIN).enable()
        }
        #[no_mangle]
        pub unsafe extern fn plugin_disable() {
            (*PLUGIN).disable()
        }
        #[no_mangle]
        pub unsafe extern fn plugin_stop() {
            (*PLUGIN).stop();
            // Free plugin
            let plugin_box = Box::from_raw(PLUGIN);
            drop(plugin_box);
            PLUGIN = ::std::ptr::null_mut();
        }
        #[no_mangle]
        pub unsafe extern fn plugin_message_from_plugin(message: i32, from: ::xplm::ipc::Plugin) {
            (*PLUGIN).message_from_plugin(message, from)
        }
        #[no_mangle]
        pub unsafe extern fn plugin_message_from_xplane(message: ::xplm::ipc::XPlaneMessage) {
            (*PLUGIN).message_from_xplane(message)
        }
    )
}

// Extern functions that plugin crates must implement
// These are marked as extern "C" because extern "Rust" is not supported. The improper_ctypes
// warning is suppressed because they are only ever called from Rust code.
#[allow(improper_ctypes)]
extern {
    /// Returns information about this plugin
    fn plugin_info() -> PluginInfo;
    /// Starts the plugin. Returns Ok on success or Err on failure
    fn plugin_start() -> Result<(), ()>;
    /// Enables the plugin
    fn plugin_enable();
    /// Disables the plugin
    fn plugin_disable();
    /// Stops and destroys the plugin
    fn plugin_stop();
    /// Called from X-Plane when the plugin receives a message from another plugin
    fn plugin_message_from_plugin(message: i32, from: ipc::Plugin);
    /// Called from X-Plane when the plugin receives a message from X-Plane
    fn plugin_message_from_xplane(message: ipc::XPlaneMessage);
}

// Plugin callbacks, called directly from C

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn XPluginStart(outName: *mut libc::c_char, outSig: *mut libc::c_char,
    outDescription: *mut libc::c_char) -> libc::c_int
{
    match plugin_start() {
        Ok(_) => {
            let info = plugin_info();

            match CString::new(info.name).ok() {
                Some(name) => libc::strcpy(outName, name.as_ptr()),
                None => libc::strcpy(outName, b"<invalid>".as_ptr() as *const libc::c_char),
            };
            match CString::new(info.signature).ok() {
                Some(signature) => libc::strcpy(outSig, signature.as_ptr()),
                None => libc::strcpy(outSig, b"<invalid>".as_ptr() as *const libc::c_char),
            };
            match CString::new(info.description).ok() {
                Some(description) => libc::strcpy(outDescription, description.as_ptr()),
                None => libc::strcpy(outDescription, b"<invalid>".as_ptr() as *const libc::c_char),
            };

            // Success
            1
        },
        Err(_) => {
            // Return failure
            0
        },
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn XPluginStop() {
    plugin_stop()
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn XPluginEnable() {
    plugin_enable()
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn XPluginDisable() {
    plugin_disable()
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn XPluginReceiveMessage(inFrom: libc::c_int, inMessage: libc::c_int,
    _: *mut libc::c_void)
{
    if inFrom == ipc::XPLANE_ID {
        if let Some(message) = ipc::XPlaneMessage::from_i32(inMessage) {
            plugin_message_from_xplane(message);
        }
    } else {
        let sender = ipc::Plugin::with_id(inFrom);
        plugin_message_from_plugin(inMessage, sender);
    }
}
