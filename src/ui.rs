
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
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

#[derive(Debug, Clone)]
struct WidgetBaseData {
    id: XPWidgetID,
    parent: Option<Box<WidgetBase>>,
    children: Vec<Box<WidgetBase>>,
}

#[derive(Debug, Clone)]
struct WidgetBase {
    data: Rc<WidgetBaseData>,
}

impl WidgetData for WidgetBase {
    fn widget_id(&self) -> XPWidgetID {
        self.data.id
    }
}

pub struct Window {
    base: WidgetBase,
}

fn test1() {
    let data = WidgetBaseData {
        id: ptr::null_mut(),
        parent: None,
        children: Vec::new(),
    };
    let base = WidgetBase { data: Rc::new(data) };
    Widget::widget_id(&base);

    let window = Window { base: base };
}

/// Common functions for Widgets
///
pub trait Widget : Clone + Sized {
    /// Returns the ID of this widget
    fn widget_id(&self) -> XPWidgetID;
    // /// Sets this widget to be visible or invisbile
    // fn set_visible(&mut self, visible: bool);
    // /// Returns a pointer to the parent of this widget, or none if this widget has no parent
    // fn parent(&self) -> Option<Widget>;
    // /// Sets the parent of this widget. Client code should normally not need to call this method;
    // /// parents are set automatically in other methods.
    // fn set_parent(&mut self, parent: Option<Widget>);
    // /// Returns an array containing pointers to this widget's children
    // ///
    // /// The returned value cannot be used to modify the children of this widget.
    // fn children(&self) -> Vec<Widget>;
    //
    // /// Adds a child to this widget
    // fn add_child(&mut self, child: Widget);
    //
    // /// Sets the descriptor of this widget. The descriptor can have various purposes depending
    // /// on the widget type.
    // fn set_descriptor(&mut self, descriptor: &str);
    // /// Returns the descriptor of this widget
    // fn get_descriptor(&self) -> String;
    // /// Sets a property of this widget
    // fn set_property(&mut self, property: i32, value: isize);
    // /// Returns the value of a property of this widget, or None if this widget does not have
    // /// a value for the requested property
    // fn get_property(&self, property: i32) -> Option<isize>;
    // /// Returns the position of this widget in the X-Plane window
    // fn get_geometry(&self) -> Rect;
    // /// Sets the position of this widget in the X-Plane window
    // fn set_geometry(&mut self, geometry: Rect);
}

/// Provides access to some internal data of a Widget.
/// Used to ease implementation of the Widget trait.
trait WidgetData {
    /// Returns this widget's ID
    fn widget_id(&self) -> XPWidgetID;
    // /// Returns an optional reference to a weak pointer to this widget's parent
    // fn parent(&self) -> &Option<Widget>;
    // /// Returns an optional mutable reference to a weak pointer to this widget's parent
    // fn parent_mut(&mut self) -> &mut Option<Widget>;
    // /// Returns a reference to a vector of pointers to child widgets
    // fn children(&self) -> &Vec<Widget>;
    // /// Returns a mutable reference to a vector of pointers to child widgets
    // fn children_mut(&mut self) -> &mut Vec<Widget>;
}

impl<T> Widget for T where T: WidgetData + Clone + Sized {
    fn widget_id(&self) -> XPWidgetID {
        WidgetData::widget_id(self)
    }
    // fn set_visible(&mut self, visible: bool) {
    //     match visible {
    //         true => unsafe { XPShowWidget(self.widget_id()); },
    //         false => unsafe { XPHideWidget(self.widget_id()); },
    //     }
    // }
    // fn parent(&self) -> Option<Widget> {
    //     WidgetData::parent(self).clone()
    // }
    //
    // fn set_parent(&mut self, parent: Option<Widget>) {
    //     self.parent_mut() = parent;
    // }
    //
    // fn children(&self) -> Vec<Widget> {
    //     WidgetData::children(self).clone()
    // }
    //
    // fn add_child(&mut self, child: Widget) {
    //     unsafe {
    //         XPPlaceWidgetWithin(child.widget_id(), self.widget_id())
    //     };
    //     child.set_parent(Some(self.clone()));
    //     WidgetData::children_mut(self).push(child);
    // }
    //
    // fn set_descriptor(&mut self, descriptor: &str) {
    //     match CString::new(descriptor) {
    //         Ok(descriptor_c) => unsafe { XPSetWidgetDescriptor(self.widget_id(), descriptor_c); },
    //         Err(_) => unsafe { XPSetWidgetDescriptor(self.widget_id(), b"".as_ptr()); },
    //     }
    // }
    // fn get_descriptor(&self) -> String {
    //     unsafe {
    //         let descriptor_length = XPGetWidgetDescriptor(self.widget_id(), ptr::null_mut(), 0) as usize;
    //         let mut buffer = StringBuffer::new(descriptor_length);
    //         XPGetWidgetDescriptor(self.widget_id(), buffer.as_mut_ptr(), descriptor_length as i32);
    //         buffer.as_string()
    //     }
    // }
    //
    // fn set_property(&mut self, property: i32, value: isize) {
    //     unsafe {
    //         XPSetWidgetProperty(self.widget_id(), property, value);
    //     }
    // }
    // fn get_property(&self, property: i32) -> Option<isize> {
    //     let mut exists: i32 = 0;
    //     let result = unsafe {
    //         XPGetWidgetProperty(self.widget_id(), property, &mut exists)
    //     };
    //     if exists == 1 {
    //         Some(result)
    //     }
    //     else {
    //         None
    //     }
    // }
    // fn get_geometry(&self) -> Rect {
    //     let mut rect: Rect = Rect { left: 0, top: 0, right: 0, bottom: 0 };
    //     unsafe {
    //         XPGetWidgetGeometry(self.widget_id(), &mut rect.left, &mut rect.top, &mut rect.right,
    //         &mut rect.bottom)
    //     };
    //     rect
    // }
    // fn set_geometry(&mut self, geometry: Rect) {
    //     unsafe {
    //         XPSetWidgetGeometry(self.widget_id(), geometry.left, geometry.top, geometry.right,
    //         geometry.bottom)
    //     };
    // }
}
