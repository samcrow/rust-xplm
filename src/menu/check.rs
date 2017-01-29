use std::cell::{Cell, RefCell};
use std::ptr;
use xplm_sys;
use std::os::raw::*;
use std::ffi::{CString, NulError};
use super::{Item, InMenu, check_c_string};


/// An item with a checkbox that can be checked or unchecked
#[derive(Debug)]
pub struct CheckItem {
    /// The text displayed for this item
    ///
    /// Invariant: this can be converted into a CString
    name: RefCell<String>,
    /// If this item is checked
    checked: Cell<bool>,
    /// Information about the menu this item is part of
    in_menu: Cell<Option<InMenu>>,
}

impl CheckItem {
    /// Creates a new item
    ///
    /// Returns an error if the name contains a null byte
    pub fn new<S: Into<String>>(name: S, checked: bool) -> Result<Self, NulError> {
        let name = name.into();
        check_c_string(&name)?;
        Ok(CheckItem {
            name: RefCell::new(name),
            checked: Cell::new(checked),
            in_menu: Cell::new(None),
        })
    }
    /// Returns true if this item is checked
    pub fn checked(&self) -> bool {
        if let Some(in_menu) = self.in_menu.get() {
            // Update from X-Plane
            unsafe {
                let mut check_state = xplm_sys::xplm_Menu_NoCheck as xplm_sys::XPLMMenuCheck;
                xplm_sys::XPLMCheckMenuItemState(in_menu.parent,
                                                 in_menu.index as c_int,
                                                 &mut check_state);
                if check_state == xplm_sys::xplm_Menu_NoCheck as xplm_sys::XPLMMenuCheck {
                    self.checked.set(false);
                } else if check_state == xplm_sys::xplm_Menu_Checked as xplm_sys::XPLMMenuCheck {
                    self.checked.set(true);
                } else {
                    // Unexpected state, correct
                    xplm_sys::XPLMCheckMenuItem(in_menu.parent,
                                                in_menu.index as c_int,
                                                xplm_sys::xplm_Menu_NoCheck as
                                                xplm_sys::XPLMMenuCheck);
                    self.checked.set(false);
                }
            }
        }
        self.checked.get()
    }
    /// Sets this item as checked or unchecked
    pub fn set_checked(&self, checked: bool) {
        self.checked.set(checked);
        if let Some(in_menu) = self.in_menu.get() {
            unsafe {
                xplm_sys::XPLMCheckMenuItem(in_menu.parent,
                                            in_menu.index as c_int,
                                            check_state(checked));
            }
        }
    }
    /// Returns the name of this item
    pub fn name(&self) -> String {
        let borrow = self.name.borrow();
        borrow.clone()
    }
    /// Sets the name of this item
    ///
    /// Returns an error if the name contains a null byte
    pub fn set_name(&self, name: &str) -> Result<(), NulError> {
        let name_c = CString::new(name)?;
        let mut borrow = self.name.borrow_mut();
        borrow.clear();
        borrow.push_str(name);
        if let Some(in_menu) = self.in_menu.get() {
            unsafe {
                xplm_sys::XPLMSetMenuItemName(in_menu.parent,
                                              in_menu.index as c_int,
                                              name_c.as_ptr(),
                                              0);
            }
        }
        Ok(())
    }
}


impl Item for CheckItem {
    fn add_to_menu(&self, parent_id: xplm_sys::XPLMMenuID) {
        let name_c = CString::new(self.name()).unwrap();
        let index = unsafe {
            let index = xplm_sys::XPLMAppendMenuItem(parent_id, name_c.as_ptr(), ptr::null_mut(), 0);
            // Configure check
            let check_state = check_state(self.checked.get());
            xplm_sys::XPLMCheckMenuItem(parent_id, index, check_state);
            index
        };
        self.in_menu.set(Some(InMenu::new(parent_id, index)));
    }
    fn update_index(&self, index_in_parent: c_int) {
        let mut in_menu = self.in_menu.get();
        if let Some(ref mut in_menu) = in_menu {
            in_menu.index = index_in_parent;
        }
        self.in_menu.set(in_menu);
    }
    fn remove_from_menu(&self, parent_id: xplm_sys::XPLMMenuID, index_in_parent: c_int) {
        unsafe { xplm_sys::XPLMRemoveMenuItem(parent_id, index_in_parent as c_int) }
    }
}


/// Maps true->checked and false->unchecked
fn check_state(checked: bool) -> xplm_sys::XPLMMenuCheck {
    if checked {
        xplm_sys::xplm_Menu_Checked as xplm_sys::XPLMMenuCheck
    } else {
        xplm_sys::xplm_Menu_Unchecked as xplm_sys::XPLMMenuCheck
    }
}
