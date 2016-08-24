// Copyright (c) 2015 rust-xplm developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

//! Allows control over optional SDK features
//!
//! Available features are documented at
//! http://www.xsquawkbox.net/xpsdk/mediawiki/XPLM_Feature_Keys .
//!

use std::ffi::{CString, CStr};
use xplm_sys::plugin::*;

/// A feature that can be enabled or disabled
#[derive(Debug)]
pub struct Feature {
    /// The name of this feature
    name: CString,
}

impl Feature {
    /// Creates a feature with a name
    ///
    /// Returns None if X-Plane does not support the requested feature, or if the name
    /// contains one or more null bytes.
    pub fn with_name(name: &str) -> Option<Feature> {
        match CString::new(name) {
            Ok(name_c) => {
                match unsafe { XPLMHasFeature(name_c.as_ptr()) } {
                    1 => Some(Feature { name: name_c }),
                    _ => None,
                }
            }
            Err(_) => None,
        }
    }
    /// Returns all the features that X-Plane supports
    pub fn all() -> Vec<Feature> {
        let mut features = Vec::new();
        unsafe {
            XPLMEnumerateFeatures(Some(feature_enumerator),
                                  ::std::mem::transmute(&mut features))
        };
        features
    }

    /// Returns the name of this feature
    ///
    /// Returns None if the feature name is not a valid UTF-8 string.
    pub fn name(&self) -> Option<&str> {
        self.name.to_str().ok()
    }

    /// Enables or disables this feature
    pub fn set_enabled(&mut self, enabled: bool) {
        unsafe { XPLMEnableFeature(self.name.as_ptr(), enabled as i32) };
    }
}

unsafe extern "C" fn feature_enumerator(feature: *const ::std::os::raw::c_char,
                                        refcon: *mut ::std::os::raw::c_void) {
    let features = refcon as *mut Vec<Feature>;
    let feature_c = CStr::from_ptr(feature);
    (*features).push(Feature { name: feature_c.to_owned() });
}
