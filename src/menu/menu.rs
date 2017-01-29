use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::ptr;
use xplm_sys;
use std::os::raw::*;
use std::ffi::{CString, NulError};
use super::{Item, check_c_string};

/// A menu, which contains zero or more items
///
// Invariant: No RefCell is borrowed outside functions of this struct
#[derive(Debug)]
pub struct Menu {
    /// The name of this menu
    ///
    /// If this menu is in the menu bar directly, this name appears in the menu bar.
    /// If this menu is a submenu, this name appears in the menu item that opens this menu.
    ///
    /// Invariant: this can be converted into a CString
    name: RefCell<String>,
    /// The items, separators, and submenus in this menu
    children: RefCell<Vec<Rc<Item>>>,
    /// The status of this menu
    state: Cell<MenuState>
}

impl Menu {
    /// Creates a new menu with the provided name
    ///
    /// Returns an error if the name contains a null byte
    pub fn new<S: Into<String>>(name: S) -> Result<Self, NulError> {
        let name = name.into();
        check_c_string(&name)?;
        Ok(Menu {
            name: RefCell::new(name),
            children: RefCell::new(Vec::new()),
            state: Cell::new(MenuState::Free),
        })
    }

    /// Returns the name of this menu
    pub fn name(&self) -> String {
        let borrow = self.name.borrow();
        borrow.clone()
    }
    /// Sets the name of this menu
    ///
    /// Returns an error if the name contains a null byte
    pub fn set_name<S: AsRef<str>>(&self, name: S) -> Result<(), NulError> {
        let name = name.as_ref();
        check_c_string(name)?;
        let mut borrow = self.name.borrow_mut();
        borrow.clear();
        borrow.push_str(name);
        Ok(())
    }
    /// Adds a child to this menu
    ///
    /// Returns a reference-counted pointer to the child.
    pub fn add_child<C: Item + 'static>(&self, child: C) -> Rc<C> {
        let child_rc = Rc::new(child);
        self.add_child_ref(child_rc.clone());
        child_rc
    }

    /// Adds a child (already in an Rc) to this menu
    pub fn add_child_ref(&self, child: Rc<Item>) {
        let mut borrow = self.children.borrow_mut();
        borrow.push(child);
    }

    /// Adds this menu as a child of the plugins menu
    pub fn add_to_plugins_menu(&self) {
        let plugins_menu = unsafe { xplm_sys::XPLMFindPluginsMenu() };
        self.add_to_menu(plugins_menu);
    }
    /// Removes this menu from the plugins menu
    pub fn remove_from_plugins_menu(&self) {
        let plugins_menu = unsafe { xplm_sys::XPLMFindPluginsMenu() };
        if let MenuState::InMenu { id: _id, parent, index_in_parent } = self.state.get() {
            if parent == plugins_menu {
                self.remove_from_menu(plugins_menu, index_in_parent);
            }
        }
    }
}

/// Status that a menu can have
#[derive(Debug, Copy, Clone)]
enum MenuState {
    /// Not attached to a menu or a menu bar
    Free,
    /// Attached as a submenu
    /// activator is the menu item that causes this menu to appear.
    /// parent is the menu ID of the parent menu.
    /// index_in_parent is the index of the activator in the parent menu
    InMenu { id: xplm_sys::XPLMMenuID, parent: xplm_sys::XPLMMenuID, index_in_parent: c_int },
}


impl Item for Menu {
    fn add_to_menu(&self, parent_id: xplm_sys::XPLMMenuID) {
        if let MenuState::Free = self.state.get() {
            let name_c = CString::new(self.name()).unwrap();
            // A submenu requires a menu item to open it
            let index = unsafe {
                xplm_sys::XPLMAppendMenuItem(parent_id, name_c.as_ptr(), ptr::null_mut(), 0)
            };

            let menu_id = unsafe {
                xplm_sys::XPLMCreateMenu(name_c.as_ptr(),
                                         parent_id,
                                         index,
                                         None,
                                         ptr::null_mut())
            };
            self.state.set(MenuState::InMenu {
                id: menu_id, parent: parent_id, index_in_parent: index
            });
            // Add children
            let borrow = self.children.borrow();
            for child in borrow.iter() {
                child.add_to_menu(menu_id);
            }
        }
    }
    fn update_index(&self, index_in_parent: c_int) {
        let mut state = self.state.get();
        if let MenuState::InMenu { id: _, parent: _, index_in_parent: ref mut index } = state {
            *index = index_in_parent;
        }
        self.state.set(state);
    }
    fn remove_from_menu(&self, _parent_id: xplm_sys::XPLMMenuID, index_in_parent: c_int) {
        if let MenuState::InMenu { id, parent, index_in_parent: _index } = self.state.get() {
            // Remove children
            {
                let borrow = self.children.borrow();
                for child in borrow.iter() {
                    // As each item is removed, the later items move up to index 0.
                    child.update_index(0);
                    child.remove_from_menu(id, 0);
                }
            }
            unsafe {
                xplm_sys::XPLMDestroyMenu(id);
            }
            // Destroy activator item
            unsafe {
                xplm_sys::XPLMRemoveMenuItem(parent, index_in_parent as c_int);
            }
            self.state.set(MenuState::Free);
        }
    }
}
