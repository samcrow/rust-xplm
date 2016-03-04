//!
//! Functionality for inter-plugin communication
//!
extern crate libc;

use xplm_sys::plugin::*;
use xplm_sys::defs::XPLMPluginID;

use std::ffi::CString;
use std::ptr;
use ffi::StringBuffer;

///
/// A plugin ID that indicates no plugin
///
const NO_ID: XPLMPluginID = -1;

///
/// Size of string buffers to allocate when getting plugin information
///
const BUFFER_SIZE: usize = 512;

///
/// Smallest message number allowed to be sent by a plugin (others are reserved
/// for X-Plane)
///
pub const MIN_USER_MESSAGE: u32 = 0x00FFFFFF;

///
/// Information about a plugin
///
pub struct PluginInfo {
    /// The name of the plugin
    pub name: String,
    /// The absolute path of the plugin binary file
    pub path: String,
    /// The signature of the plugin
    pub signature: String,
    /// The description of the plugin
    pub description: String,
}

///
/// Represents a plugin and allows access to it
///
pub struct Plugin {
    /// The ID of this plugin
    id: XPLMPluginID,
}

impl Plugin {
    ///
    /// Returns the current running plugin
    ///
    pub fn this_plugin() -> Plugin {
        let plugin_id = unsafe { XPLMGetMyID() };
        assert!(plugin_id != NO_ID);
        Plugin {
            id: plugin_id
        }
    }

    ///
    /// Searches for a plugin based on its signature
    ///
    pub fn with_signature(signature: &str) -> Option<Plugin> {
        match CString::new(signature) {
            Ok(c_sig) => {
                let plugin_id = unsafe {
                    XPLMFindPluginBySignature(c_sig.as_ptr())
                };
                match plugin_id {
                    NO_ID => None,
                    id => Some(Plugin { id: id }),
                }
            },
            Err(_) => None,
        }
    }
    ///
    /// Returns all available plugins, including disabled ones
    ///
    pub fn all_plugins() -> Vec<Plugin> {
        let plugin_count = unsafe { XPLMCountPlugins() };
        let mut plugins = Vec::with_capacity(plugin_count as usize);
        for i in 0..plugin_count {
            let id = unsafe { XPLMGetNthPlugin(i) };
            assert!(id != NO_ID);
            plugins.push(Plugin { id: id });
        }
        plugins
    }

    ///
    /// Returns true if this plugin is enabled
    ///
    pub fn is_enabled(&self) -> bool {
        let enabled = unsafe { XPLMIsPluginEnabled(self.id) };
        enabled != 0
    }

    ///
    /// Enables or disables this plugin
    ///
    /// Returns Err if the plugin was disabled and failed to enable.
    ///
    /// Returns Ok in all other cases, including if the plugin was already
    /// in the requested state.
    ///
    pub fn set_enabled(&self, enabled: bool) -> Result<(), ()> {
        match (self.is_enabled(), enabled) {
            (true, false) => {
                // Disable
                unsafe { XPLMDisablePlugin(self.id) };
                Ok(())
            },
            (false, true) => {
                // Enable
                let result = unsafe { XPLMEnablePlugin(self.id) };
                match result {
                    1 => Ok(()),
                    _ => Err(()),
                }
            },
            _ => {
                // Already in requested state
                Ok(())
            }
        }
    }

    ///
    /// Returns information about this plugin
    ///
    pub fn info(&self) -> PluginInfo {
        let mut name = StringBuffer::new(BUFFER_SIZE);
        let mut path = StringBuffer::new(BUFFER_SIZE);
        let mut signature = StringBuffer::new(BUFFER_SIZE);
        let mut description = StringBuffer::new(BUFFER_SIZE);

        unsafe {
            XPLMGetPluginInfo(
                self.id,
                name.as_mut_ptr(),
                path.as_mut_ptr(),
                signature.as_mut_ptr(),
                description.as_mut_ptr());
        }
        PluginInfo {
            name: name.as_string(),
            path: path.as_string(),
            signature: path.as_string(),
            description: description.as_string(),
        }
    }

    ///
    /// Sends a message to this plugin
    ///
    /// Returns Err if the message is less than the minimum user message (`0x00FFFFFF`).
    ///
    pub fn send_message(&self, message: u32) -> Result<(), ()> {
        if message >= MIN_USER_MESSAGE {
            unsafe {
                XPLMSendMessageToPlugin(self.id, message as libc::c_int, ptr::null_mut());
            }
            Ok(())
        } else {
            // Reserved message number
            Err(())
        }
    }
}
