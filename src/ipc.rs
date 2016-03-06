//! Functionality for inter-plugin communication
//!
extern crate libc;

use xplm_sys::plugin::*;
use xplm_sys::defs::XPLMPluginID;

use std::ffi::CString;
use std::error::Error;
use std::fmt;
use ffi::StringBuffer;

///
/// A plugin ID that indicates no plugin
///
const NO_ID: XPLMPluginID = -1;

///
/// A plugin ID that indicates a message sent by X-Plane
///
pub const XPLANE_ID: XPLMPluginID = 0;

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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
        Plugin { id: plugin_id }
    }

    ///
    /// Searches for a plugin based on its signature
    ///
    pub fn with_signature(signature: &str) -> Option<Plugin> {
        match CString::new(signature) {
            Ok(c_sig) => {
                let plugin_id = unsafe { XPLMFindPluginBySignature(c_sig.as_ptr()) };
                match plugin_id {
                    NO_ID => None,
                    id => Some(Plugin { id: id }),
                }
            }
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
    /// Creates a Plugin from an XPLMPluginID.
    ///
    /// This function is unsafe because it may create an invalid Plugin.
    pub unsafe fn with_id(id: XPLMPluginID) -> Plugin {
        Plugin { id: id }
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
            }
            (false, true) => {
                // Enable
                let result = unsafe { XPLMEnablePlugin(self.id) };
                match result {
                    1 => Ok(()),
                    _ => Err(()),
                }
            }
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
            XPLMGetPluginInfo(self.id,
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
    pub fn send_message(&self, message: i32, argument: usize) -> Result<(), SendError> {
        if message >= MIN_USER_MESSAGE {
            unsafe {
                XPLMSendMessageToPlugin(self.id,
                                        message as libc::c_int,
                                        argument as *mut libc::c_void);
            }
            Ok(())
        } else {
            // Reserved message number
            Err(SendError)
        }
    }
}

/// An error that indicates that a message could not be sent because its message number
/// was invald
#[derive(Debug)]
pub struct SendError;

impl fmt::Display for SendError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Message number less than {}", MIN_USER_MESSAGE)
    }
}

impl Error for SendError {
    fn description(&self) -> &str {
        "Message number less than minimum"
    }
}

/// Messages that X-Plane can send to a plugin
#[derive(Debug, Clone)]
pub enum XPlaneMessage {
    /// Indicates that the plane has crashed
    PlaneCrashed,
    /// Indicates that the user has loaded a new aircraft
    PlaneLoaded,
    /// Indicates a new livery for the current aircraft has been loaded
    LiveryLoaded,
    /// Indicates that the number of active aircraft has changed
    PlaneCountChanged,
    /// Indicates that the user has unloaded the aircraft
    PlaneUnloaded,
    /// Indicates that the user has positioned the aircraft at an airport
    AirportLoaded,
    /// Indicates that some new scenery has been loaded
    SceneryLoaded,
    /// Indicates that X-Plane is about to write preferences
    WillWritePreferences,
}

impl XPlaneMessage {
    /// Converts an integer value (as provided by X-Plane) into an XPlaneMessage
    pub fn from_i32(value: i32) -> Option<XPlaneMessage> {
        match value {
            101 => Some(XPlaneMessage::PlaneCrashed),
            102 => Some(XPlaneMessage::PlaneLoaded),
            103 => Some(XPlaneMessage::AirportLoaded),
            104 => Some(XPlaneMessage::SceneryLoaded),
            105 => Some(XPlaneMessage::PlaneCountChanged),
            106 => Some(XPlaneMessage::PlaneUnloaded),
            107 => Some(XPlaneMessage::WillWritePreferences),
            108 => Some(XPlaneMessage::LiveryLoaded),
            _ => None,
        }
    }
}
