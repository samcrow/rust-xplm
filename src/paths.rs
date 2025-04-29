use std::{path::PathBuf, ptr::null_mut, str::Utf8Error};

use xplm_sys::{
    XPLMGetMyID, XPLMGetNthAircraftModel, XPLMGetPluginInfo, XPLMGetPrefsPath, XPLMGetSystemPath,
};

use super::ffi::StringBuffer;

/// Get path to this plugin
pub fn plugin_path() -> Result<PathBuf, Utf8Error> {
    let mut path = StringBuffer::new(512);
    unsafe {
        XPLMGetPluginInfo(
            XPLMGetMyID(),
            null_mut(),
            path.as_mut_ptr(),
            null_mut(),
            null_mut(),
        );
    }
    Ok(PathBuf::from(path.as_str()?))
}

/// Get path to X-Plane's preferences file
/// (usually "Output/preferences/Set X-Plane.prf")
pub fn prefs_path() -> Result<PathBuf, Utf8Error> {
    let mut path = StringBuffer::new(512);
    unsafe {
        XPLMGetPrefsPath(path.as_mut_ptr());
    }
    Ok(PathBuf::from(path.as_str()?))
}

/// Get path to X-Plane root folder
pub fn system_path() -> Result<PathBuf, Utf8Error> {
    let mut path = StringBuffer::new(512);
    unsafe {
        XPLMGetSystemPath(path.as_mut_ptr());
    }
    Ok(PathBuf::from(path.as_str()?))
}

/// Get path to loaded aircraft's .acf file
pub fn aircraft_path() -> Result<PathBuf, Utf8Error> {
    let mut path = StringBuffer::new(512);
    unsafe {
        XPLMGetNthAircraftModel(0, StringBuffer::new(256).as_mut_ptr(), path.as_mut_ptr());
    }
    Ok(PathBuf::from(path.as_str()?))
}
