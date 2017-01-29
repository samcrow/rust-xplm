use std::fmt::Debug;
use xplm_sys;
use std::os::raw::*;
use std::mem;
use std::ffi::{CString, NulError};

mod menu;
pub use self::menu::Menu;

mod action;
pub use self::action::ActionItem;

mod check;
pub use self::check::CheckItem;


/// Something that can be added to a menu
pub trait Item: Debug {
    /// Called when this item is added to a parent menu
    #[doc(hidden)]
    fn add_to_menu(&self, parent_id: xplm_sys::XPLMMenuID);
    /// Called when the position of this item in the parent menu changes. The new index
    /// is provided.
    #[doc(hidden)]
    fn update_index(&self, index_in_parent: c_int);
    /// Called when this item is removed from a parent menu
    #[doc(hidden)]
    fn remove_from_menu(&self, parent_id: xplm_sys::XPLMMenuID, index_in_parent: c_int);
    /// Called when the user clicks on this menu item
    ///
    /// The default implementation does nothing.
    #[doc(hidden)]
    fn handle_click(&self) {

    }
}

/// A separator between menu items
#[derive(Debug)]
pub struct Separator;


impl Item for Separator {
    fn add_to_menu(&self, parent_id: xplm_sys::XPLMMenuID) {
        unsafe { xplm_sys::XPLMAppendMenuSeparator(parent_id) }
    }
    fn update_index(&self, _index_in_parent: c_int) {
        // Nothing
    }
    fn remove_from_menu(&self, parent_id: xplm_sys::XPLMMenuID, index_in_parent: c_int) {
        unsafe { xplm_sys::XPLMRemoveMenuItem(parent_id, index_in_parent as c_int) }
    }
}

/// Information stored by a menu item when it has been added to a menu
#[derive(Debug, Copy, Clone)]
struct InMenu {
    /// The menu ID of the parent menu
    pub parent: xplm_sys::XPLMMenuID,
    /// The index of this item in the parent menu
    pub index: c_int,
}

impl InMenu {
    pub fn new(parent: xplm_sys::XPLMMenuID, index: c_int) -> Self {
        InMenu {
            parent: parent,
            index: index,
        }
    }
}

/// Confirms that the provided string can be converted into a CString.
/// Returns an error if it cannot.
fn check_c_string(text: &str) -> Result<(), NulError> {
    CString::new(text)?;
    Ok(())
}
