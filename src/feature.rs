use std::ffi::{CStr, CString};
use std::fmt;
use std::os::raw::*;
use xplm_sys;

/// A feature provided by the SDK that this plugin is running in
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Feature {
    /// The name of this feature
    /// Invariant: this can be successfully converted into a CString
    name: String,
}

impl Feature {
    /// Returns the name of this feature
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns true if this feature is currently enabled
    pub fn enabled(&self) -> bool {
        let name_c = CString::new(&*self.name).unwrap();
        let enabled = unsafe { xplm_sys::XPLMIsFeatureEnabled(name_c.as_ptr()) };
        enabled == 1
    }

    /// Enables or disables this feature
    pub fn set_enabled(&self, enable: bool) {
        // Because this name was either copied from C with XPLMEnumerateFeatures or
        // checked with XPLMHasFeature, it must be valid as a C string.
        let name_c = CString::new(&*self.name).unwrap();
        unsafe { xplm_sys::XPLMEnableFeature(name_c.as_ptr(), enable as c_int) }
    }
}

impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Looks for a feature with the provided name and returns it if it exists
pub fn find_feature<S: Into<String>>(name: S) -> Option<Feature> {
    match CString::new(name.into()) {
        Ok(name) => {
            let has_feature = unsafe { xplm_sys::XPLMHasFeature(name.as_ptr()) };
            if has_feature == 1 {
                // Convert name back into a String
                // Because the string was not modified, conversion will always work.
                Some(Feature { name: name.into_string().unwrap() })
            } else {
                None
            }
        }
        Err(_) => None
    }
}

/// Returns all features supported by the X-Plane plugin SDK
pub fn all_features() -> Vec<Feature> {
    let mut features = Vec::new();
    let features_ptr: *mut _ = &mut features;
    unsafe {
        xplm_sys::XPLMEnumerateFeatures(Some(feature_callback), features_ptr as *mut c_void);
    }
    features
}

/// Interprets refcon as a pointer to a Vec<Feature>.
/// Allocates a new Feature and adds it to the vector
unsafe extern "C" fn feature_callback(feature: *const c_char, refcon: *mut c_void) {
    let features = refcon as *mut Vec<Feature>;

    let name = CStr::from_ptr(feature);
    if let Ok(name) = name.to_str() {
        let new_feature = Feature { name: name.to_owned() };
        (*features).push(new_feature);
    }
}
