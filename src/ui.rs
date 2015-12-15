
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::ffi::CString;
use std::ptr;
use ffi::StringBuffer;

use xplm_sys::widgets::widget_defs::*;
use xplm_sys::widgets::widgets::*;

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
struct Base {
    id: XPWidgetID,
    parent: Option<Weak<HasBase>>,
    children: Vec<Rc<HasBase>>,
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
    pub fn new(widget_type: XPWidgetClass, descriptor: &str, geometry: &Rect, root: bool) -> Base {
        let id = unsafe {
            XPCreateWidget(geometry.left, geometry.top, geometry.right, geometry.bottom,
                0, c_string_or_empty(descriptor).as_ptr(), root as i32, 0 as XPWidgetID, widget_type)
        };
        Base {
            id: id,
            parent: None,
            children: Vec::new(),
        }
    }
}

impl Drop for Base {
    /// Destroys this widget
    fn drop(&mut self) {
        unsafe {
            // Destroy widget, do not destroy children
            XPDestroyWidget(self.id, 0);
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

/// Common functions for all types of widgets
pub trait Widget {
    /// Returns the ID of this widget
    fn widget_id(&self) -> XPWidgetID;
    /// Shows or hides this widget
    ///
    /// If this widget is not a root widget and is not a child of a root widget,
    /// showing it may not have any effect.
    fn set_visible(&mut self, visible: bool);
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
    }
}

const WINDOW_WIDGET_CLASS: XPWidgetClass = 1;
/// Represents a window
pub struct Window {
    base: BasePtr,
}

impl Window {
    /// Creates a new Window with the provided title and geometry
    pub fn new(title: &str, geometry: &Rect) -> Window {
        Window {
            base: Rc::new(RefCell::new(Base::new(WINDOW_WIDGET_CLASS, title, geometry, true)))
        }
    }
}

impl HasBase for Window {
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
