
use xplm_sys::display::*;
use xplm_sys::defs::XPLMKeyFlags;
use ui::Rect;

use std::ptr;
use std::mem;
use std::boxed::Box;

/// Trait for callbacks that can draw windows
pub trait DrawCallback {
    /// Called to draw a window
    fn draw(&mut self);
}

impl<F> DrawCallback for F where F: Fn() {
    fn draw(&mut self) {
        self()
    }
}

pub struct Window {
    /// Pointer to the window data block, allocated in a Box
    data: *mut WindowData,
}

impl Window {
    /// Creates a new window the the specified geometry
    pub fn new(geometry: &Rect) -> Window {
        // Allocate a data block
        let data_ptr = Box::into_raw(Box::new(WindowData {
            id: ptr::null_mut(), draw_callback: None
        }));

        let mut params = XPLMCreateWindow_t {
            structSize: mem::size_of::<XPLMCreateWindow_t>() as i32,
            left: geometry.left,
            top: geometry.top,
            right: geometry.right,
            bottom: geometry.bottom,
            visible: false as i32,
            drawWindowFunc: Some(draw_callback),
            handleMouseClickFunc: Some(mouse_click_callback),
            handleKeyFunc: Some(key_callback),
            handleCursorFunc: Some(cursor_callback),
            handleMouseWheelFunc: Some(mouse_wheel_callback),
            refcon: data_ptr as *mut ::libc::c_void,
        };
        let window_id = unsafe { XPLMCreateWindowEx(&mut params) };
        // Set window ID in data block
        unsafe {
            (*data_ptr).id = window_id;
        }

        Window {
            data: data_ptr,
        }
    }

    /// Sets the callback used to draw this window
    pub fn set_draw_callback<C>(&mut self, callback: C) where C: 'static + DrawCallback {
        unsafe {
            (*self.data).draw_callback = Some(Box::new(callback));
        }
    }

    /// Sets a window to be visible or hidden
    pub fn set_visible(&mut self, visible: bool) {
        unsafe { XPLMSetWindowIsVisible((*self.data).id, visible as i32); }
    }

    /// Moves this window on top of other windows
    pub fn bring_to_front(&self) {
        unsafe { XPLMBringWindowToFront((*self.data).id); }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            let data_box = Box::from_raw(self.data);
            // Destroy window
            XPLMDestroyWindow(data_box.id);
            drop(data_box);
        }
    }
}

struct WindowData {
    /// The ID of this window
    pub id: XPLMWindowID,
    /// The callback used for drawing
    pub draw_callback: Option<Box<DrawCallback>>,
}


// Global callbacks

unsafe extern "C" fn draw_callback(_: XPLMWindowID, refcon: *mut ::libc::c_void) {
    let data = refcon as *mut WindowData;
    match (*data).draw_callback {
        Some(ref mut callback) => callback.draw(),
        None => {},
    }
}

unsafe extern "C" fn key_callback(_: XPLMWindowID, _: ::libc::c_char, _: XPLMKeyFlags,
                                _: ::libc::c_char, _: *mut ::libc::c_void, _: ::libc::c_int) {
    // Do nothing
}

unsafe extern "C" fn mouse_click_callback(_: XPLMWindowID, _: ::libc::c_int, _: ::libc::c_int,
                                    _: XPLMMouseStatus, _: *mut ::libc::c_void) -> ::libc::c_int {
    // Consume click
    1
}

unsafe extern "C" fn cursor_callback(_: XPLMWindowID, _: ::libc::c_int, _: ::libc::c_int,
                                           _: *mut ::libc::c_void) -> XPLMCursorStatus {
   // Standard cursor determined by X-Plane
   xplm_CursorDefault as i32
}

unsafe extern "C" fn mouse_wheel_callback(_: XPLMWindowID, _: ::libc::c_int, _: ::libc::c_int,
                                           _: ::libc::c_int, _: ::libc::c_int,
                                           _: *mut ::libc::c_void) -> ::libc::c_int {
   // Consume scroll event
   1
}
