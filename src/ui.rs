
use std::rc::Rc;
use std::cell::RefCell;
use std::ffi::CString;
use std::ptr;
use std::mem;

use ffi::StringBuffer;

use xplm_sys::widgets::widget_defs::*;
use xplm_sys::widgets::widgets::*;
use xplm_sys::widgets::standard_widgets;

/// A rectangle on the screen in X-Plane
///
/// Coordinates are in pixels. The origin is at the bottom left corner of the window. Positive
/// X values are right, positive Y values are up.
///
pub struct Rect {
    /// Left X coordinate
    pub left: i32,
    /// Top Y coordinate
    pub top: i32,
    /// Right X coordinate
    pub right: i32,
    /// Bottom Y coordinate
    pub bottom: i32,
}

/// The basic information that each widget needs
///
/// Each widget stores a reference-counted pointer to a Base. Multiple Widgets may refer to
/// the same Base.
///
/// Each Base corresponds to one X-Plane widget resource.
struct Base {
    /// The ID of this widget
    id: XPWidgetID,
    /// The children of this widget
    children: Vec<Box<Widget>>,
    /// Pointer to the delegate of this widget, allocated in a Box
    delegate: *mut WidgetDelegate,
}

impl Base {
    /// Creates a new Widget
    ///
    /// geometry: The initial geometry of the widget
    ///
    /// type: The type of widget to create
    ///
    /// descriptor: The initial descriptor of the widget
    ///
    /// root: True if this widget should be a root widget. If this is false, the widget must be
    /// added as a child of a root widget to be displayed.
    ///
    /// delegate: A delegate that will handle message sent to this widget
    pub fn new<D>(widget_type: XPWidgetClass, descriptor: &str, geometry: &Rect, root: bool,
        delegate: D) -> Base where D: 'static + WidgetDelegate {

        let id = unsafe {
            XPCreateWidget(geometry.left, geometry.top, geometry.right, geometry.bottom,
                0, c_string_or_empty(descriptor).as_ptr(), root as i32, 0 as XPWidgetID, widget_type)
        };

        let delegate_ptr = Box::into_raw(Box::new(delegate));
        // Set the delegate as the widget's refcon
        unsafe { XPSetWidgetProperty(id, xpProperty_Refcon as i32, delegate_ptr as isize); }
        // Install the callback
        unsafe { XPAddWidgetCallback(id, Some(message_handler::<D>)); }

        Base {
            id: id,
            children: Vec::new(),
            delegate: delegate_ptr,
        }
    }
}

impl Drop for Base {
    /// Destroys this widget
    fn drop(&mut self) {
        unsafe {
            // Destroy widget, do not destroy children
            XPDestroyWidget(self.id, 0);
            // Destroy delegate
            let delegate_box = Box::from_raw(self.delegate);
            drop(delegate_box);
        }
    }
}

/// A refernece-counted, runtime-mutability-checked, pointer to a Base
type BasePtr = Rc<RefCell<Base>>;

/// Trait for all widgets that contain a Base. All widges should implement this trait.
/// Implementing this trait also implements Widget (see below).
trait HasBase {
    fn base(&self) -> BasePtr;
}

pub trait WidgetDelegate {
    /// Handles a message received by a widget
    fn handle_message(&mut self, message: XPWidgetMessage, param1: isize, param2: isize) -> i32;
}

impl<T> WidgetDelegate for T where T: Fn(XPWidgetMessage, isize, isize) -> i32 {
    fn handle_message(&mut self, message: XPWidgetMessage, param1: isize, param2: isize) -> i32 {
        self(message, param1, param2)
    }
}

/// Common functions for all types of widgets
pub trait Widget {
    /// Returns the ID of this widget
    fn widget_id(&self) -> XPWidgetID;
    /// Shows or hides this widget
    ///
    /// If this widget is not a root widget and is not a child of a root widget,
    /// showing it may not have any effect.
    fn set_visible(&mut self, visible: bool);
    /// Returns the value of a property, or None if this widget does not have the requested
    /// property
    fn get_property(&self, property: i32) -> Option<isize>;
    /// Sets the value of a property of this widget
    fn set_property(&mut self, property: i32, value: isize);
    /// Returns the descriptor of this widget
    fn get_descriptor(&self) -> String;
    /// Sets the descriptor of this widget
    ///
    /// If the provided string is not valid as a C string, the descriptor will not be changed.
    fn set_descriptor(&mut self, descriptor: &str);
    /// Returns the geometry of this widget
    fn get_geometry(&self) -> Rect;
    /// Sets the geometry of this widget
    fn set_geometry(&mut self, geometry: &Rect);
    /// Removes the children of this widget
    fn clear_children(&mut self);
    /// Adds a child to this widget
    fn add_child(&mut self, child: Box<Widget>);
}


/// Implements Widget for all widgets that have bases
impl<T> Widget for T where T: HasBase {
    fn widget_id(&self) -> XPWidgetID {
        let base = self.base();
        let borrow = base.borrow();
        borrow.id
    }
    fn set_visible(&mut self, visible: bool) {
        match visible {
            true => unsafe { XPShowWidget(self.widget_id()) },
            false => unsafe { XPHideWidget(self.widget_id()) },
        }
        let base = self.base();
        let mut borrow = base.borrow_mut();
        for mut child in borrow.children.iter_mut() {
            child.set_visible(visible);
        }
    }
    fn get_property(&self, property: i32) -> Option<isize> {
        let mut exists: i32 = 0;
        let result = unsafe {
            XPGetWidgetProperty(self.widget_id(), property, &mut exists)
        };
        if exists == 1 {
            Some(result)
        }
        else {
            None
        }
    }
    fn set_property(&mut self, property: i32, value: isize) {
        unsafe {
            XPSetWidgetProperty(self.widget_id(), property, value);
        }
    }
    fn get_descriptor(&self) -> String {
        let id = self.widget_id();
        let length = unsafe {
            XPGetWidgetDescriptor(id, ptr::null_mut(), 0) as usize
        };
        let mut buffer = StringBuffer::new(length);
        unsafe {
            XPGetWidgetDescriptor(id, buffer.as_mut_ptr(), length as i32);
        }
        buffer.as_string()
    }
    fn set_descriptor(&mut self, descriptor: &str) {
        match CString::new(descriptor) {
            Ok(descriptor_c) => unsafe {
                XPSetWidgetDescriptor(self.widget_id(), descriptor_c.as_ptr());
            },
            Err(_) => {},
        }
    }
    fn get_geometry(&self) -> Rect {
        let mut rect: Rect = Rect { left: 0, top: 0, right: 0, bottom: 0 };
        unsafe {
            XPGetWidgetGeometry(self.widget_id(), &mut rect.left, &mut rect.top, &mut rect.right,
            &mut rect.bottom);
        }
        rect
    }
    fn set_geometry(&mut self, geometry: &Rect) {
        unsafe {
            XPSetWidgetGeometry(self.widget_id(), geometry.left, geometry.top, geometry.right,
            geometry.bottom);
        }
    }
    fn clear_children(&mut self) {
        let base = self.base();
        let mut borrow = base.borrow_mut();
        borrow.children.clear();
    }
    fn add_child(&mut self, child: Box<Widget>) {
        unsafe {
            XPPlaceWidgetWithin(child.widget_id(), self.widget_id());
        }
        let base = self.base();
        let mut borrow = base.borrow_mut();
        borrow.children.push(child);
    }
}



const WINDOW_WIDGET_CLASS: XPWidgetClass = 1;

/// Represents a window
///
/// By default, a window has close buttons.
///
/// The descriptor of a window is its title.
pub struct Window {
    base: BasePtr,
}

impl Window {
    /// Creates a new Window with the provided title and geometry
    pub fn new(title: &str, geometry: &Rect) -> Window {
        let mut window = Window {
            base: Rc::new(RefCell::new(Base::new(WINDOW_WIDGET_CLASS, title, geometry, true, |_,_,_| 0)))
        };
        window.set_close_buttons(true);
        window.set_translucent(false);
        window
    }
    /// Sets whether this window should have close buttons
    pub fn set_close_buttons(&mut self, close_buttons: bool) {
        self.set_property(standard_widgets::xpProperty_MainWindowHasCloseBoxes as i32,
            close_buttons as isize);
    }
    /// Sets whether this window should be translucent or should be a standard opaque window
    pub fn set_translucent(&mut self, translucent: bool) {
        let property_value = match translucent {
            true => standard_widgets::xpMainWindowStyle_Translucent,
            false => standard_widgets::xpMainWindowStyle_MainWindow,
        };
        self.set_property(standard_widgets::xpProperty_MainWindowType as i32,
            property_value as isize);
    }
}

impl HasBase for Window {
    fn base(&self) -> BasePtr {
        self.base.clone()
    }
}

/// Appearance styles for panes
pub enum PaneType {
    /// A standard pane
    Pane,
    /// A dark-colored screen that can display test
    Screen,
    /// A list of items
    List,
}

const PANE_WIDGET_CLASS: XPWidgetClass = 2;

/// A pane that can subdivide a Window
///
/// While a Window automatically moves its children when the user drags it around the screen,
/// a Pane does not. Therefore, it may be better to have UI elements be children of a window than
/// children of a Pane.
pub struct Pane {
    base: BasePtr,
}

impl Pane {
    /// Creates a pane with the provided title and geometry
    pub fn new(title: &str, geometry: &Rect) -> Pane {
        let mut pane = Pane {
            base: Rc::new(RefCell::new(Base::new(PANE_WIDGET_CLASS, title, geometry, false, |_,_,_| 0)))
        };
        pane.set_pane_type(PaneType::Pane);
        pane
    }
    /// Sets the type of this pane
    pub fn set_pane_type(&mut self, pane_type: PaneType) {
        let value = match pane_type {
            PaneType::Pane => standard_widgets::xpSubWindowStyle_SubWindow,
            PaneType::Screen => standard_widgets::xpSubWindowStyle_Screen,
            PaneType::List => standard_widgets::xpSubWindowStyle_ListView,
        };
        self.set_property(standard_widgets::xpProperty_SubWindowType as i32,
            value as isize);
    }
}

impl HasBase for Pane {
    fn base(&self) -> BasePtr {
        self.base.clone()
    }
}

/// The widget class used for buttons, check boxes, and radio buttons
const BUTTON_WIDGET_CLASS: XPWidgetClass = 3;

/// A push button
pub struct Button {
    base: BasePtr,
}

impl Button {
    /// Creates a button with the provided text and geometry
    pub fn new(text: &str, geometry: &Rect) -> Button {
        let mut button = Button {
            base: Rc::new(RefCell::new(Base::new(BUTTON_WIDGET_CLASS, text, geometry, false, |_,_,_| 0)))
        };
        button.set_property(standard_widgets::xpProperty_ButtonType as i32,
            standard_widgets::xpPushButton as isize);
        button.set_property(standard_widgets::xpProperty_ButtonBehavior as i32,
            standard_widgets::xpButtonBehaviorPushButton as isize);

        button
    }

}

impl HasBase for Button {
    fn base(&self) -> BasePtr {
        self.base.clone()
    }
}
/// A check box
///
/// A check box does not include a label.
pub struct CheckBox {
    base: BasePtr,
}

impl CheckBox {
    /// Creates a check box with the provided geometry
    pub fn new(geometry: &Rect) -> Button {
        let mut button = Button {
            base: Rc::new(RefCell::new(Base::new(BUTTON_WIDGET_CLASS, "", geometry, false, |_,_,_| 0)))
        };
        button.set_property(standard_widgets::xpProperty_ButtonType as i32,
            standard_widgets::xpRadioButton as isize);
        button.set_property(standard_widgets::xpProperty_ButtonBehavior as i32,
            standard_widgets::xpButtonBehaviorCheckBox as isize);

        button
    }
    pub fn is_checked(&self) -> bool {
        Some(1) == self.get_property(standard_widgets::xpProperty_ButtonState as i32)
    }
    pub fn set_checked(&mut self, checked: bool) {
        self.set_property(standard_widgets::xpProperty_ButtonState as i32, checked as isize);
    }
}

impl HasBase for CheckBox {
    fn base(&self) -> BasePtr {
        self.base.clone()
    }
}

/// Tries to convert a string into a CString. If the conversion fails,
/// returns a valid but empty CString.
fn c_string_or_empty(value: &str) -> CString {
    match CString::new(value) {
        Ok(value_c) => value_c,
        Err(_) => CString::new("").unwrap(),
    }
}

// Callback section

/// Callback used for all Windows
///
/// D is tye type of widget delegate that is used for this widget
extern "C" fn message_handler<D>(message: XPWidgetMessage, widget: XPWidgetID,
                                    param1: ::libc::intptr_t, param2: ::libc::intptr_t)
                                    -> ::libc::c_int where D: WidgetDelegate {
    unsafe {
        // The refcon is a pointer to the widget delegate
        let mut exists: i32 = 0;
        let delegate: *mut D =
            mem::transmute(XPGetWidgetProperty(widget, xpProperty_Refcon as i32, &mut exists));
        if exists == 1 {
            (*delegate).handle_message(message, param1, param2)
        }
        else {
            // Not processed
            0
        }
    }
}
