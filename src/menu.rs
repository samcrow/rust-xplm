use std::cell::{Cell, RefCell};
use std::fmt::Debug;
use std::ptr;
use xplm_sys;
use std::os::raw::*;
use std::ffi::{CString, NulError};

/// Adds a menu item to the plugins menu
pub fn add_to_plugins_menu(menu: &Menu) {
    let plugin_menu_id = unsafe { xplm_sys::XPLMFindPluginsMenu() };
    // TODO: figure out index
    menu.add_to_menu(plugin_menu_id, 0);
}

/// Removes a menu item from the plugins menu
pub fn remove_from_plugins_menu(menu: &Menu) {
    let plugin_menu_id = unsafe { xplm_sys::XPLMFindPluginsMenu() };
    // TODO: figure out index
    menu.remove_from_menu(plugin_menu_id, 0);
}


/// A menu, which contains zero or more items
///
#[derive(Debug)]
pub struct Menu<'c, 'n> {
    /// The name of this menu
    ///
    /// If this menu is in the menu bar directly, this name appears in the menu bar.
    /// If this menu is a submenu, this name appears in the menu item that opens this menu.
    ///
    /// Invariant: this can be converted into a CString
    name: Cell<&'n str>,
    /// The items, separators, and submenus in this menu
    children: RefCell<Vec<&'c Item>>,
    /// Information about the menu this menu is in (if this is a submenu)
    in_menu: Cell<Option<InMenu>>,
    /// The ID of this menu, if it has been provided to X-Plane
    id: Cell<Option<xplm_sys::XPLMMenuID>>,
}

impl<'c, 'n> Menu<'c, 'n> {
    /// Creates a new menu with the provided name
    ///
    /// Returns an error if the name contains a null byte
    pub fn new(name: &'n str) -> Result<Self, NulError> {
        check_c_string(name)?;
        Ok(Menu {
            name: Cell::new(name),
            children: RefCell::new(Vec::new()),
            in_menu: Cell::new(None),
            id: Cell::new(None),
        })
    }

    /// Returns the name of this menu
    pub fn name(&self) -> &'n str {
        self.name.get()
    }
    /// Sets the name of this menu
    ///
    /// Returns an error if the name contains a null byte
    pub fn set_name(&self, name: &'n str) -> Result<(), NulError> {
        check_c_string(name)?;
        self.name.set(name);
        Ok(())
    }
    /// Adds a child to this menu
    pub fn add_child<C: Item>(&self, child: &'c C) {
        let mut borrow = self.children.borrow_mut();
        borrow.push(child);
    }
}

/// An item that can be clicked on to perform an action
#[derive(Debug)]
pub struct ActionItem<'n> {
    /// The text displayed for this item
    ///
    /// Invariant: this can be converted into a CString
    name: Cell<&'n str>,
    /// Information about the menu this item is part of
    in_menu: Cell<Option<InMenu>>,
}

impl<'n> ActionItem<'n> {
    /// Creates a new item
    ///
    /// Returns an error if the name contains a null byte
    pub fn new(name: &'n str) -> Result<Self, NulError> {
        check_c_string(name)?;
        Ok(ActionItem {
            name: Cell::new(name),
            in_menu: Cell::new(None),
        })
    }
    /// Returns the name of this item
    pub fn name(&self) -> &'n str {
        self.name.get()
    }
    /// Sets the name of this item
    ///
    /// Returns an error if the name contains a null byte
    pub fn set_name(&self, name: &'n str) -> Result<(), NulError> {
        check_c_string(name)?;
        self.name.set(name);
        Ok(())
    }
}

/// An item with a checkbox that can be checked or unchecked
#[derive(Debug)]
pub struct CheckItem<'n> {
    /// The text displayed for this item
    ///
    /// Invariant: this can be converted into a CString
    name: Cell<&'n str>,
    /// If this item is checked
    checked: Cell<bool>,
    /// Information about the menu this item is part of
    in_menu: Cell<Option<InMenu>>,
}

impl<'n> CheckItem<'n> {
    /// Creates a new item
    ///
    /// Returns an error if the name contains a null byte
    pub fn new(name: &'n str, checked: bool) -> Result<Self, NulError> {
        check_c_string(name)?;
        Ok(CheckItem {
            name: Cell::new(name),
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
    pub fn name(&self) -> &'n str {
        self.name.get()
    }
    /// Sets the name of this item
    ///
    /// Returns an error if the name contains a null byte
    pub fn set_name(&self, name: &'n str) -> Result<(), NulError> {
        let name_c = CString::new(name)?;
        self.name.set(name);
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

/// A separator between menu items
#[derive(Debug)]
pub struct Separator;

/// The one separator instance
static SEPARATOR: Separator = Separator;

/// Returns a menu separator
pub fn separator() -> &'static Separator {
    &SEPARATOR
}

/// Something that can be added to a menu
pub trait Item: Debug {
    /// Called when this item is added to a parent menu
    /// index_in_parent is the 0-based index where this item is being added
    #[doc(hidden)]
    fn add_to_menu(&self, parent_id: xplm_sys::XPLMMenuID, index_in_parent: usize);
    /// Called when the position of this item in the parent menu changes. The new index
    /// is provided.
    #[doc(hidden)]
    fn update_index(&self, index_in_parent: usize);
    /// Called when this item is removed from a parent menu
    #[doc(hidden)]
    fn remove_from_menu(&self, parent_id: xplm_sys::XPLMMenuID, index_in_parent: usize);
}

impl<'n> Item for ActionItem<'n> {
    fn add_to_menu(&self, parent_id: xplm_sys::XPLMMenuID, index_in_parent: usize) {
        self.in_menu.set(Some(InMenu::new(parent_id, index_in_parent)));
        let name_c = CString::new(self.name.get()).unwrap();
        unsafe {
            // API note: XPLMAppendMenuItem returns the index of the appended item.
            // A menu separator also has an index and takes up a slot, but
            // XPLMAppendMenuSeparator does not return the index of the added separator.
            xplm_sys::XPLMAppendMenuItem(parent_id, name_c.as_ptr(), ptr::null_mut(), 0);
            // Ensure item is not checkable
            xplm_sys::XPLMCheckMenuItem(parent_id,
                                        index_in_parent as c_int,
                                        xplm_sys::xplm_Menu_NoCheck as c_int);
        }
    }
    fn update_index(&self, index_in_parent: usize) {
        let mut in_menu = self.in_menu.get();
        if let Some(ref mut in_menu) = in_menu {
            in_menu.index = index_in_parent;
        }
        self.in_menu.set(in_menu);
    }
    fn remove_from_menu(&self, parent_id: xplm_sys::XPLMMenuID, index_in_parent: usize) {
        unsafe { xplm_sys::XPLMRemoveMenuItem(parent_id, index_in_parent as c_int) }
    }
}

impl<'n> Item for CheckItem<'n> {
    fn add_to_menu(&self, parent_id: xplm_sys::XPLMMenuID, index_in_parent: usize) {
        let name_c = CString::new(self.name.get()).unwrap();
        unsafe {
            xplm_sys::XPLMAppendMenuItem(parent_id, name_c.as_ptr(), ptr::null_mut(), 0);
            // Configure check
            let check_state = check_state(self.checked.get());
            xplm_sys::XPLMCheckMenuItem(parent_id, index_in_parent as c_int, check_state);
        }
    }
    fn update_index(&self, index_in_parent: usize) {
        let mut in_menu = self.in_menu.get();
        if let Some(ref mut in_menu) = in_menu {
            in_menu.index = index_in_parent;
        }
        self.in_menu.set(in_menu);
    }
    fn remove_from_menu(&self, parent_id: xplm_sys::XPLMMenuID, index_in_parent: usize) {
        unsafe { xplm_sys::XPLMRemoveMenuItem(parent_id, index_in_parent as c_int) }
    }
}

impl<'c, 'n> Item for Menu<'c, 'n> {
    fn add_to_menu(&self, parent_id: xplm_sys::XPLMMenuID, index_in_parent: usize) {
        let name_c = CString::new(self.name.get()).unwrap();
        let menu_id = unsafe {
            xplm_sys::XPLMCreateMenu(name_c.as_ptr(),
                                     parent_id,
                                     index_in_parent as c_int,
                                     None,
                                     ptr::null_mut())
        };
        self.id.set(Some(menu_id));
        self.in_menu.set(Some(InMenu::new(parent_id, index_in_parent)));
        // Add children
        let borrow = self.children.borrow();
        for (i, child) in borrow.iter().enumerate() {
            child.add_to_menu(menu_id, i);
        }
    }
    fn update_index(&self, index_in_parent: usize) {
        let mut in_menu = self.in_menu.get();
        if let Some(ref mut in_menu) = in_menu {
            in_menu.index = index_in_parent;
        }
        self.in_menu.set(in_menu);
    }
    fn remove_from_menu(&self, _parent_id: xplm_sys::XPLMMenuID, _index_in_parent: usize) {
        if let Some(menu_id) = self.id.get() {
            // Remove children
            {
                let borrow = self.children.borrow();
                for child in borrow.iter() {
                    // As each item is removed, the later items move up to index 0.
                    child.update_index(0);
                    child.remove_from_menu(menu_id, 0);
                }
            }
            unsafe {
                xplm_sys::XPLMDestroyMenu(menu_id);
            }
            self.id.set(None);
            self.in_menu.set(None);
        }
    }
}

impl Item for Separator {
    fn add_to_menu(&self, parent_id: xplm_sys::XPLMMenuID, _index_in_parent: usize) {
        unsafe { xplm_sys::XPLMAppendMenuSeparator(parent_id) }
    }
    fn update_index(&self, _index_in_parent: usize) {
        // Nothing
    }
    fn remove_from_menu(&self, parent_id: xplm_sys::XPLMMenuID, index_in_parent: usize) {
        unsafe { xplm_sys::XPLMRemoveMenuItem(parent_id, index_in_parent as c_int) }
    }
}

/// Information stored by a menu item when it has been added to a menu
#[derive(Debug, Copy, Clone)]
struct InMenu {
    /// The menu ID of the parent menu
    pub parent: xplm_sys::XPLMMenuID,
    /// The index of this item in the parent menu
    pub index: usize,
}

impl InMenu {
    pub fn new(parent: xplm_sys::XPLMMenuID, index: usize) -> Self {
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

/// Maps true->checked and false->unchecked
fn check_state(checked: bool) -> xplm_sys::XPLMMenuCheck {
    if checked {
        xplm_sys::xplm_Menu_Checked as xplm_sys::XPLMMenuCheck
    } else {
        xplm_sys::xplm_Menu_Unchecked as xplm_sys::XPLMMenuCheck
    }
}
