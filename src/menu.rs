use std::cell::{Cell, RefCell};
use std::ffi::{CString, NulError};
use std::fmt;
use std::os::raw::*;
use std::ptr;
use std::rc::Rc;
use xplm_sys;

/// Something that can be added to a menu
#[derive(Debug, Clone)]
pub enum Item {
    /// A submenu
    Submenu(Rc<Menu>),
    /// An action item
    Action(Rc<ActionItem>),
    /// A checkable item
    Check(Rc<CheckItem>),
    /// A separator
    Separator,
}

impl Item {
    /// Called when this item is added to a parent menu
    fn add_to_menu(&self, parent_id: xplm_sys::XPLMMenuID) {
        match *self {
            Item::Submenu(ref menu) => menu.add_to_menu(parent_id),
            // Pass the address of this Item as a reference for the callback
            Item::Action(ref action) => action.add_to_menu(parent_id, self),
            Item::Check(ref check) => check.add_to_menu(parent_id, self),
            Item::Separator => Separator.add_to_menu(parent_id),
        }
    }
    /// Called when the position of this item in the parent menu changes. The new index
    /// is provided.
    fn update_index(&self, index_in_parent: c_int) {
        match *self {
            Item::Submenu(ref menu) => menu.update_index(index_in_parent),
            Item::Action(ref action) => action.update_index(index_in_parent),
            Item::Check(ref check) => check.update_index(index_in_parent),
            Item::Separator => Separator.update_index(index_in_parent),
        }
    }
    /// Called when this item is removed from a parent menu
    fn remove_from_menu(&self, parent_id: xplm_sys::XPLMMenuID, index_in_parent: c_int) {
        match *self {
            Item::Submenu(ref menu) => menu.remove_from_menu(parent_id, index_in_parent),
            Item::Action(ref action) => action.remove_from_menu(parent_id, index_in_parent),
            Item::Check(ref check) => check.remove_from_menu(parent_id, index_in_parent),
            Item::Separator => Separator.remove_from_menu(parent_id, index_in_parent),
        }
    }
    /// Called when the user clicks on this menu item
    fn handle_click(&self) {
        match *self {
            Item::Action(ref action) => action.handle_click(),
            Item::Check(ref check) => check.handle_click(),
            _ => {}
        }
    }
}

impl From<Rc<Menu>> for Item {
    fn from(m: Rc<Menu>) -> Self {
        Item::Submenu(m)
    }
}
impl From<Rc<ActionItem>> for Item {
    fn from(a: Rc<ActionItem>) -> Self {
        Item::Action(a)
    }
}
impl From<Rc<CheckItem>> for Item {
    fn from(c: Rc<CheckItem>) -> Self {
        Item::Check(c)
    }
}
impl From<Rc<Separator>> for Item {
    fn from(_: Rc<Separator>) -> Self {
        Item::Separator
    }
}

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
    ///
    /// Each item is in a Box, to allow callbacks to reference it.
    children: RefCell<Vec<Box<Item>>>,
    /// The status of this menu
    state: Cell<MenuState>,
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
    /// The child argument may be a Menu, ActionItem, CheckItem, or Separator,
    /// or an Rc containing one of these types.
    pub fn add_child<R, C>(&self, child: R)
    where
        R: Into<Rc<C>>,
        Rc<C>: Into<Item>,
    {
        let mut borrow = self.children.borrow_mut();
        borrow.push(Box::new(child.into().into()));
    }

    /// Adds this menu as a child of the plugins menu
    pub fn add_to_plugins_menu(&self) {
        let plugins_menu = unsafe { xplm_sys::XPLMFindPluginsMenu() };
        self.add_to_menu(plugins_menu);
    }
    /// Removes this menu from the plugins menu
    pub fn remove_from_plugins_menu(&self) {
        let plugins_menu = unsafe { xplm_sys::XPLMFindPluginsMenu() };
        if let MenuState::InMenu {
            id: _id,
            parent,
            index_in_parent,
        } = self.state.get()
        {
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
    InMenu {
        id: xplm_sys::XPLMMenuID,
        parent: xplm_sys::XPLMMenuID,
        index_in_parent: c_int,
    },
}

impl Menu {
    fn add_to_menu(&self, parent_id: xplm_sys::XPLMMenuID) {
        if let MenuState::Free = self.state.get() {
            let name_c = CString::new(self.name()).unwrap();
            // A submenu requires a menu item to open it
            let index = unsafe {
                xplm_sys::XPLMAppendMenuItem(parent_id, name_c.as_ptr(), ptr::null_mut(), 0)
            };

            let menu_id = unsafe {
                xplm_sys::XPLMCreateMenu(
                    name_c.as_ptr(),
                    parent_id,
                    index,
                    Some(menu_handler),
                    ptr::null_mut(),
                )
            };
            self.state.set(MenuState::InMenu {
                id: menu_id,
                parent: parent_id,
                index_in_parent: index,
            });
            // Add children
            let borrow = self.children.borrow();
            for child in borrow.iter() {
                // Memory safety warning: Child must be allocated in a Box to prevent it from
                // moving
                child.add_to_menu(menu_id);
            }
        }
    }
    fn update_index(&self, index_in_parent: c_int) {
        let mut state = self.state.get();
        if let MenuState::InMenu {
            id: _,
            parent: _,
            index_in_parent: ref mut index,
        } = state
        {
            *index = index_in_parent;
        }
        self.state.set(state);
    }
    fn remove_from_menu(&self, _parent_id: xplm_sys::XPLMMenuID, index_in_parent: c_int) {
        if let MenuState::InMenu {
            id,
            parent,
            index_in_parent: _index,
        } = self.state.get()
        {
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

/// Removes this menu from X-Plane, to prevent the menu handler from running and accessing
/// a dangling pointer
impl Drop for Menu {
    fn drop(&mut self) {
        if let MenuState::InMenu {
            id: _id,
            parent,
            index_in_parent,
        } = self.state.get()
        {
            self.remove_from_menu(parent, index_in_parent);
        }
    }
}

/// A separator between menu items
#[derive(Debug)]
pub struct Separator;

impl Separator {
    fn add_to_menu(&self, parent_id: xplm_sys::XPLMMenuID) {
        // API note: XPLMAppendMenuItem returns the index of the appended item.
        // A menu separator also has an index and takes up a slot, but
        // XPLMAppendMenuSeparator does not return the index of the added separator.
        unsafe { xplm_sys::XPLMAppendMenuSeparator(parent_id) }
    }
    fn update_index(&self, _index_in_parent: c_int) {
        // Nothing
    }
    fn remove_from_menu(&self, parent_id: xplm_sys::XPLMMenuID, index_in_parent: c_int) {
        unsafe { xplm_sys::XPLMRemoveMenuItem(parent_id, index_in_parent as c_int) }
    }
}

/// An item that can be clicked on to perform an action
pub struct ActionItem {
    /// The text displayed for this item
    ///
    /// Invariant: this can be converted into a CString
    name: RefCell<String>,
    /// Information about the menu this item is part of
    in_menu: Cell<Option<InMenu>>,
    /// The item click handler
    handler: Box<RefCell<dyn MenuClickHandler>>,
}

impl ActionItem {
    /// Creates a new item
    ///
    /// Returns an error if the name contains a null byte
    pub fn new<S: Into<String>, H: MenuClickHandler>(
        name: S,
        handler: H,
    ) -> Result<Self, NulError> {
        let name = name.into();
        check_c_string(&name)?;
        Ok(ActionItem {
            name: RefCell::new(name),
            in_menu: Cell::new(None),
            handler: Box::new(RefCell::new(handler)),
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
                xplm_sys::XPLMSetMenuItemName(
                    in_menu.parent,
                    in_menu.index as c_int,
                    name_c.as_ptr(),
                    0,
                );
            }
        }
        Ok(())
    }
}

impl ActionItem {
    fn add_to_menu(&self, parent_id: xplm_sys::XPLMMenuID, enclosing_item: *const Item) {
        let name_c = CString::new(self.name()).unwrap();
        let index = unsafe {
            let index = xplm_sys::XPLMAppendMenuItem(
                parent_id,
                name_c.as_ptr(),
                enclosing_item as *mut _,
                0,
            );
            // Ensure item is not checkable
            xplm_sys::XPLMCheckMenuItem(parent_id, index, xplm_sys::xplm_Menu_NoCheck as c_int);
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

    fn handle_click(&self) {
        let mut borrow = self.handler.borrow_mut();
        borrow.item_clicked(&self);
    }
}

/// Removes this menu from X-Plane, to prevent the menu handler from running and accessing
/// a dangling pointer
impl Drop for ActionItem {
    fn drop(&mut self) {
        if let Some(in_menu) = self.in_menu.get() {
            self.remove_from_menu(in_menu.parent, in_menu.index);
        }
    }
}

impl fmt::Debug for ActionItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ActionItem")
            .field("name", &self.name)
            .field("in_menu", &self.in_menu)
            .finish()
    }
}

/// Trait for things that can respond when the user clicks on a menu item
pub trait MenuClickHandler: 'static {
    /// Called when the user clicks on a menu item. The clicked item is passed.
    fn item_clicked(&mut self, item: &ActionItem);
}

impl<F> MenuClickHandler for F
where
    F: FnMut(&ActionItem) + 'static,
{
    fn item_clicked(&mut self, item: &ActionItem) {
        self(item)
    }
}

/// An item with a checkbox that can be checked or unchecked
pub struct CheckItem {
    /// The text displayed for this item
    ///
    /// Invariant: this can be converted into a CString
    name: RefCell<String>,
    /// If this item is checked
    checked: Cell<bool>,
    /// Information about the menu this item is part of
    in_menu: Cell<Option<InMenu>>,
    /// The check handler
    handler: Box<RefCell<dyn CheckHandler>>,
}

impl CheckItem {
    /// Creates a new item
    ///
    /// Returns an error if the name contains a null byte
    pub fn new<S: Into<String>, H: CheckHandler>(
        name: S,
        checked: bool,
        handler: H,
    ) -> Result<Self, NulError> {
        let name = name.into();
        check_c_string(&name)?;
        Ok(CheckItem {
            name: RefCell::new(name),
            checked: Cell::new(checked),
            in_menu: Cell::new(None),
            handler: Box::new(RefCell::new(handler)),
        })
    }
    /// Returns true if this item is checked
    pub fn checked(&self) -> bool {
        if let Some(in_menu) = self.in_menu.get() {
            // Update from X-Plane
            unsafe {
                let mut check_state = xplm_sys::xplm_Menu_NoCheck as xplm_sys::XPLMMenuCheck;
                xplm_sys::XPLMCheckMenuItemState(
                    in_menu.parent,
                    in_menu.index as c_int,
                    &mut check_state,
                );
                if check_state == xplm_sys::xplm_Menu_NoCheck as xplm_sys::XPLMMenuCheck {
                    self.checked.set(false);
                } else if check_state == xplm_sys::xplm_Menu_Checked as xplm_sys::XPLMMenuCheck {
                    self.checked.set(true);
                } else {
                    // Unexpected state, correct
                    xplm_sys::XPLMCheckMenuItem(
                        in_menu.parent,
                        in_menu.index as c_int,
                        xplm_sys::xplm_Menu_NoCheck as xplm_sys::XPLMMenuCheck,
                    );
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
                xplm_sys::XPLMCheckMenuItem(
                    in_menu.parent,
                    in_menu.index as c_int,
                    check_state(checked),
                );
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
                xplm_sys::XPLMSetMenuItemName(
                    in_menu.parent,
                    in_menu.index as c_int,
                    name_c.as_ptr(),
                    0,
                );
            }
        }
        Ok(())
    }
}

impl CheckItem {
    fn add_to_menu(&self, parent_id: xplm_sys::XPLMMenuID, enclosing_item: *const Item) {
        let name_c = CString::new(self.name()).unwrap();
        let index = unsafe {
            let index = xplm_sys::XPLMAppendMenuItem(
                parent_id,
                name_c.as_ptr(),
                enclosing_item as *mut _,
                0,
            );
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

    fn handle_click(&self) {
        // Invert check
        let checked = !self.checked();
        self.set_checked(checked);
        let mut borrow = self.handler.borrow_mut();
        borrow.item_checked(self, checked);
    }
}
/// Removes this menu from X-Plane, to prevent the menu handler from running and accessing
/// a dangling pointer
impl Drop for CheckItem {
    fn drop(&mut self) {
        if let Some(in_menu) = self.in_menu.get() {
            self.remove_from_menu(in_menu.parent, in_menu.index);
        }
    }
}

impl fmt::Debug for CheckItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CheckItem")
            .field("name", &self.name)
            .field("checked", &self.checked)
            .field("in_menu", &self.in_menu)
            .finish()
    }
}

/// Trait for things that can respond to check state changes
pub trait CheckHandler: 'static {
    /// Called when the user checks or unchecks an item
    fn item_checked(&mut self, item: &CheckItem, checked: bool);
}

impl<F> CheckHandler for F
where
    F: FnMut(&CheckItem, bool) + 'static,
{
    fn item_checked(&mut self, item: &CheckItem, checked: bool) {
        self(item, checked)
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
        InMenu { parent, index }
    }
}

/// Confirms that the provided string can be converted into a CString.
/// Returns an error if it cannot.
fn check_c_string(text: &str) -> Result<(), NulError> {
    CString::new(text).map(|_| ())
}

/// The menu handler callback used for all menu items
///
/// item_ref is a pointer to the relevant Item, allocated in an Rc
unsafe extern "C" fn menu_handler(_menu_ref: *mut c_void, item_ref: *mut c_void) {
    let item = item_ref as *const Item;
    (*item).handle_click();
}
