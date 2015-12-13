//!
//! Allows control over optional SDK features
//!
//! Available features are documented at http://www.xsquawkbox.net/xpsdk/mediawiki/XPLM_Feature_Keys .
//!

use std::ffi::CString;
use xplm_sys::plugin::*;

/// Returns true if X-Plane supports the feature with the provided name
pub fn has_feature(name: &str) -> bool {
    match CString::new(name) {
        Ok(name_c) => unsafe { 1 == XPLMHasFeature(name_c.as_ptr()) },
        Err(_) => false,
    }
}

/// Returns true if the specified feature is supported and enabled
pub fn feature_enabled(name: &str) -> bool {
    if !has_feature(name) {
        return false;
    }
    match CString::new(name) {
        Ok(name_c) => unsafe { 1 == XPLMIsFeatureEnabled(name_c.as_ptr()) },
        Err(_) => false,
    }
}
/// Sets a feature as enabled or disabled. Returns an error if the provide feature name
/// is invalid or if the feature is not supported.
pub fn set_feature_enabled(name: &str, enabled: bool) -> Result<(), ()> {
    if !has_feature(name) {
        return Err(());
    }
    match CString::new(name) {
        Ok(name_c) => {
            unsafe { XPLMEnableFeature(name_c.as_ptr(), enabled as i32) };
            Ok(())
        },
        Err(_) => Err(()),
    }
}
