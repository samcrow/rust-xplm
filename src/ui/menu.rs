// Copyright (c) 2015 rust-xplm developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

//!
//! Allows the creation of menus and menu items
//!
//! This module is incomplete and should not be used.
//!

use std::boxed::Box;
use std::ptr;
use std::ffi::CString;
use xplm_sys::menus::*;

/// Trait for an item that can be placed in the menu
pub trait MenuItem {
    /// Called when this menu item is selected
    fn selected(&mut self);
}

pub struct Menu {
    /// X-plane menu ID
    id: XPLMMenuID,
}

impl Menu {

    /// Creates and returns a root menu in the X-Plane menu bar
    pub fn root_menu(name: &str) -> Menu {
        unimplemented!();
    }

    /// Creates and returns a menu in the plugins menu
    pub fn plugin_menu(name: &str) -> Menu {
        unimplemented!();
    }

    pub fn append_item<I>(&mut self, item: I) where I: MenuItem {
        unimplemented!();
    }
    pub fn append_separator(&mut self) {
        unimplemented!();
    }
}

impl Drop for Menu {
    fn drop(&mut self) {
        unimplemented!();
    }
}


/// The global menu callback
unsafe extern "C" fn global_callback(inMenuRef: *mut ::libc::c_void,
                                           inItemRef: *mut ::libc::c_void) {

    unimplemented!();
}


/// Converts a string slice into a CString. If the provided string slice
/// could not be converted, returns an unspecified non-empty string value.
fn make_c(name: &str) -> CString {
    match CString::new(name) {
        Ok(cstr) => cstr,
        Err(_) => CString::new("<invalid>").unwrap(),
    }
}
