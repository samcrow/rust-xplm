use std::cell::{Cell, RefCell};
use std::ptr;
use xplm_sys;
use std::os::raw::*;
use std::ffi::{CString, NulError};
use super::{Item, InMenu, check_c_string};


/// An item that can be clicked on to perform an action
#[derive(Debug)]
pub struct ActionItem {
    /// The text displayed for this item
    ///
    /// Invariant: this can be converted into a CString
    name: RefCell<String>,
    /// Information about the menu this item is part of
    in_menu: Cell<Option<InMenu>>,
}

impl ActionItem {
    /// Creates a new item
    ///
    /// Returns an error if the name contains a null byte
    pub fn new<S: Into<String>>(name: S) -> Result<Self, NulError> {
        let name = name.into();
        check_c_string(&name)?;
        Ok(ActionItem {
            name: RefCell::new(name),
            in_menu: Cell::new(None),
        })
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


impl Item for ActionItem {
    fn add_to_menu(&self, parent_id: xplm_sys::XPLMMenuID) {
        let name_c = CString::new(self.name()).unwrap();
        let index = unsafe {
            // API note: XPLMAppendMenuItem returns the index of the appended item.
            // A menu separator also has an index and takes up a slot, but
            // XPLMAppendMenuSeparator does not return the index of the added separator.
            let index = xplm_sys::XPLMAppendMenuItem(parent_id, name_c.as_ptr(), ptr::null_mut(), 0);
            // Ensure item is not checkable
            xplm_sys::XPLMCheckMenuItem(parent_id,
                                        index,
                                        xplm_sys::xplm_Menu_NoCheck as c_int);
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

/// Trait for things that can respond when the user clicks on a menu item
pub trait MenuAction {
    /// Called when the user clicks on a menu item. The clicked item is passed.
    fn item_clicked(&mut self, item: &ActionItem);
}

impl<F> MenuAction for F where F: FnMut(&ActionItem) {
    fn item_clicked(&mut self, item: &ActionItem) {
        self(item)
    }
}
