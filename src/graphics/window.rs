// Copyright (c) 2015 rust-xplm developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.


use xplm_sys::display::*;
use xplm_sys::defs::{XPLMKeyFlags, xplm_ShiftFlag, xplm_OptionAltFlag, xplm_ControlFlag,
    xplm_DownFlag, xplm_UpFlag};
use ui::{Rect, Point, Cursor, MouseEvent, KeyEvent, ModifierKeys, Key};

use std::ptr;
use std::mem;
use std::boxed::Box;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::ops::DerefMut;

/// Trait for callbacks that can draw windows
pub trait DrawCallback {
    /// Called to draw a window
    /// The window is provided.
    fn draw(&mut self, window: &mut Window);
}

impl<F> DrawCallback for F where F: Fn(&mut Window) {
    fn draw(&mut self, window: &mut Window) {
        self(window)
    }
}

/// Trait for things that can specifiy a cursor
pub trait CursorCallback {
    /// Called to determine the cursor to show
    ///
    /// cursor_pos is the position of the cursor, in global coordinates.
    fn cursor(&mut self, cursor_pos: &Point, window: &mut Window) -> Cursor;
}

impl<F> CursorCallback for F where F: Fn(&Point, &mut Window) -> Cursor {
    fn cursor(&mut self, cursor_pos: &Point, window: &mut Window) -> Cursor {
        self(cursor_pos, window)
    }
}

/// Trait for things that can respond to mouse events
pub trait MouseCallback {
    /// Called when the mouse is pressed, dragged, or released
    ///
    /// cursor_pos is the position of the cursor in global coordinates
    ///
    /// event is the type of mouse event
    ///
    /// This function should return true if it has processed the mouse event and other code should
    /// not receive it.
    fn mouse_event(&mut self, cursor_pos: &Point, event: MouseEvent, window: &mut Window) -> bool;
}

impl<T> MouseCallback for T where T: Fn(&Point, MouseEvent, &mut Window) -> bool {
    fn mouse_event(&mut self, cursor_pos: &Point, event: MouseEvent, window: &mut Window) -> bool {
        self(cursor_pos, event, window)
    }
}

/// Trait for things that can respond to mouse wheel events
pub trait MouseWheelCallback {
    /// Called when the mouse wheel is moved
    ///
    /// cursor_pos is the position of the cursor in global coordinates
    ///
    /// delta_x and delta_y are the distance of wheel movement, in unspecified units, on the
    /// X and Y axes.
    ///
    /// This function should return true if it has processed the event and other code should not
    /// receive it.
    fn mouse_wheel_event(&mut self, cursor_pos: &Point, delta_x: i32, delta_y: i32,
        window: &mut Window) -> bool;
}

impl<T> MouseWheelCallback for T where T: Fn(&Point, i32, i32, &mut Window) -> bool {
    fn mouse_wheel_event(&mut self, cursor_pos: &Point, delta_x: i32, delta_y: i32,
        window: &mut Window) -> bool {
        self(cursor_pos, delta_x, delta_y, window)
    }
}

/// Trait for things that can respond to keyboard events
pub trait KeyboardCallback {
    /// Called when a key is pressed or released
    ///
    /// event_type stores information on whether the key was pressed or released
    ///
    /// key stores the key that was pressed or released
    ///
    /// modifiers stores the modifier keys that were held down during this event
    ///
    /// ascii stores a character representation of the key pressed, if one exists
    fn keyboard_event(&mut self, event_type: KeyEvent, key: Key, modifiers: ModifierKeys,
        ascii: Option<char>, window: &mut Window);
}

impl<T> KeyboardCallback for T where T: Fn(KeyEvent, Key, ModifierKeys, Option<char>, &mut Window) {
    fn keyboard_event(&mut self, event_type: KeyEvent, key: Key, modifiers: ModifierKeys,
        ascii: Option<char>, window: &mut Window) {
        self(event_type, key, modifiers, ascii, window)
    }
}

/// A low-level window
///
/// A Window provides a designated area on the screen and callbacks for drawing and handling user
/// input. By default, it is invisible and does not respond to input.
///
/// Windows should always be kept in `Rc<RefCell>`s to ensure proper functionality.
#[allow(missing_debug_implementations)]
pub struct Window {
    /// Pointer to the window data block, allocated in a Box
    data: *mut WindowData,
}

impl Window {
    /// Creates a new window the the specified geometry
    pub fn new(geometry: &Rect) -> Rc<RefCell<Window>> {
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
            refcon: ptr::null_mut(),
        };
        let window_id = unsafe { XPLMCreateWindowEx(&mut params) };

        // Create a window
        let window = Rc::new(RefCell::new(Window {
            data: ptr::null_mut(),
        }));

        // Allocate a data block with a weak pointer to the window
        let data_ptr = Box::into_raw(Box::new(WindowData {
            id: window_id,
            draw_callback: None,
            keyboard_callback: None,
            mouse_callback: None,
            mouse_wheel_callback: None,
            cursor_callback: None,
            window: Rc::downgrade(&window),
        }));
        // Set window ID as refcon
        unsafe {
            XPLMSetWindowRefCon(window_id, data_ptr as *mut ::libc::c_void);
        }
        // Set pointer to data block in window
        window.borrow_mut().data = data_ptr;

        window
    }

    /// Sets the callback used to draw this window
    pub fn set_draw_callback<C>(&mut self, callback: C) where C: 'static + DrawCallback {
        unsafe {
            (*self.data).draw_callback = Some(Box::new(callback));
        }
    }
    /// Sets the callback to handle keyboard events for this window
    pub fn set_keyboard_callback<C>(&mut self, callback: C) where C: 'static + KeyboardCallback {
        unsafe {
            (*self.data).keyboard_callback = Some(Box::new(callback));
        }
    }
    /// Sets the callback to handle mouse events for this window
    pub fn set_mouse_callback<C>(&mut self, callback: C) where C: 'static + MouseCallback {
        unsafe {
            (*self.data).mouse_callback = Some(Box::new(callback));
        }
    }
    /// Sets the callback to handle mouse wheel events for this window
    pub fn set_mouse_wheel_callback<C>(&mut self, callback: C) where C: 'static + MouseWheelCallback {
        unsafe {
            (*self.data).mouse_wheel_callback = Some(Box::new(callback));
        }
    }
    /// Sets the callback to specify cursors for this window
    pub fn set_cursor_callback<C>(&mut self, callback: C) where C: 'static + CursorCallback {
        unsafe {
            (*self.data).cursor_callback = Some(Box::new(callback));
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
    /// Gives this window keyboard focus
    pub fn request_focus(&self) {
        unsafe { XPLMTakeKeyboardFocus((*self.data).id); }
    }

    /// Returns the geometry of this window
    pub fn get_geometry(&self) -> Rect {
        let mut rect = Rect { left: 0, top: 0, right: 0, bottom: 0 };
        unsafe {
            XPLMGetWindowGeometry((*self.data).id, &mut rect.left, &mut rect.top, &mut rect.right,
            &mut rect.bottom);
        }
        rect
    }

    /// Sets the geometry of this window
    pub fn set_geometry(&mut self, geometry: &Rect) {
        unsafe {
            XPLMSetWindowGeometry((*self.data).id, geometry.left, geometry.top, geometry.right,
            geometry.bottom);
        }
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
    /// The draw callback
    draw_callback: Option<Box<DrawCallback>>,
    /// The key callback
    keyboard_callback: Option<Box<KeyboardCallback>>,
    /// The mouse callback
    mouse_callback: Option<Box<MouseCallback>>,
    /// The mouse wheel callback
    mouse_wheel_callback: Option<Box<MouseWheelCallback>>,
    /// The cursor callback
    cursor_callback: Option<Box<CursorCallback>>,
    /// Pointer to the Window that holds this data
    window: Weak<RefCell<Window>>,
}

impl WindowData {
    /// Calls the draw callback, if it is available
    pub fn draw_callback(&mut self) {
        match self.draw_callback {
            Some(ref mut callback) => {
                match self.window.upgrade() {
                    Some(window_strong) => {
                        callback.draw(window_strong.borrow_mut().deref_mut());
                    },
                    None => {},
                }
            },
            None => {},
        }
    }
    /// Calls the keyboard callback, if it is available
    pub fn keyboard_callback(&mut self, event_type: KeyEvent, key: Key, modifiers: ModifierKeys,
        ascii: Option<char>) {
        match self.keyboard_callback {
            Some(ref mut callback) => {
                match self.window.upgrade() {
                    Some(window_strong) => {
                        callback.keyboard_event(event_type, key, modifiers, ascii,
                            window_strong.borrow_mut().deref_mut());
                    },
                    None => {},
                }
            },
            None => {},
        }
    }
    /// Calls the mouse callback, if it is available
    pub fn mouse_callback(&mut self, cursor_pos: &Point, event: MouseEvent) -> bool {
        match self.mouse_callback {
            Some(ref mut callback) => {
                match self.window.upgrade() {
                    Some(window_strong) => {
                        callback.mouse_event(cursor_pos, event,
                            window_strong.borrow_mut().deref_mut())
                    },
                    None => true,
                }
            },
            None => true,
        }
    }
    /// Calls the mouse wheel callback, if it is available
    pub fn mouse_wheel_callback(&mut self, cursor_pos: &Point, delta_x: i32, delta_y: i32) -> bool {
        match self.mouse_wheel_callback {
            Some(ref mut callback) => {
                match self.window.upgrade() {
                    Some(window_strong) => {
                        callback.mouse_wheel_event(cursor_pos, delta_x, delta_y,
                            window_strong.borrow_mut().deref_mut())
                    },
                    None => true,
                }
            },
            None => true,
        }
    }
    /// Calls the cursor callback, if it is available
    pub fn cursor_callback(&mut self, cursor_pos: &Point) -> Cursor {
        match self.cursor_callback {
            Some(ref mut callback) => {
                match self.window.upgrade() {
                    Some(window_strong) => {
                        callback.cursor(cursor_pos, window_strong.borrow_mut().deref_mut())
                    },
                    None => Cursor::Default,
                }
            },
            None => Cursor::Default,
        }
    }
}


// Global callbacks

unsafe extern "C" fn draw_callback(_: XPLMWindowID, refcon: *mut ::libc::c_void) {
    let data = refcon as *mut WindowData;
    (*data).draw_callback();
}

unsafe extern "C" fn key_callback(_: XPLMWindowID, key: ::libc::c_char, flags: XPLMKeyFlags,
                                virtual_key: ::libc::c_char, refcon: *mut ::libc::c_void,
                                _: ::libc::c_int) {
    let data = refcon as *mut WindowData;
    let ascii = match key {
        0 => None,
        _ => Some((key as u8) as char),
    };
    let modifiers = flags_to_modifiers(flags);
    let event = flags_to_event(flags);
    let key = vk_to_key(virtual_key);
    if let (Some(event), Some(key)) = (event, key) {
        (*data).keyboard_callback(event, key, modifiers, ascii);
    }
}

#[allow(non_upper_case_globals)]
unsafe extern "C" fn mouse_click_callback(_: XPLMWindowID, x: ::libc::c_int, y: ::libc::c_int,
                                    status: XPLMMouseStatus, refcon: *mut ::libc::c_void)
                                    -> ::libc::c_int {
    let data = refcon as *mut WindowData;
    let event = match status as u32 {
        xplm_MouseDown => Some(MouseEvent::Pressed),
        xplm_MouseDrag => Some(MouseEvent::Dragged),
        xplm_MouseUp => Some(MouseEvent::Released),
        _ => None,
    };
    if let Some(event) = event {
        (*data).mouse_callback(&Point { x: x, y: y }, event) as i32
    }
    else {
        // Unrecognized event not processed
        0
    }
}

unsafe extern "C" fn cursor_callback(_: XPLMWindowID, x: ::libc::c_int, y: ::libc::c_int,
                                           refcon: *mut ::libc::c_void) -> XPLMCursorStatus {
    let data = refcon as *mut WindowData;
    let cursor = (*data).cursor_callback(&Point { x: x, y: y });
    cursor_to_xplm_cursor(cursor)
}

unsafe extern "C" fn mouse_wheel_callback(_: XPLMWindowID, x: ::libc::c_int, y: ::libc::c_int,
                                           wheel: ::libc::c_int, clicks: ::libc::c_int,
                                           refcon: *mut ::libc::c_void) -> ::libc::c_int {
    let data = refcon as *mut WindowData;
    let (dx, dy) = match wheel {
        0 => (0, clicks), // vertical
        1 => (clicks, 0), // horizontal
        _ => return 0, // unrecognized request not processed
    };
    (*data).mouse_wheel_callback(&Point { x: x, y: y }, dx, dy) as i32
}

fn cursor_to_xplm_cursor(cursor: Cursor) -> XPLMCursorStatus {
    match cursor {
        Cursor::Default => xplm_CursorDefault as i32,
        Cursor::Hidden => xplm_CursorHidden as i32,
        Cursor::Arrow => xplm_CursorArrow as i32,
    }
}

// Data conversion

fn flags_to_modifiers(flags: XPLMKeyFlags) -> ModifierKeys {
    let flags = flags as u32;
    ModifierKeys {
        shift: (flags & xplm_ShiftFlag) != 0,
        option: (flags & xplm_OptionAltFlag) != 0,
        control: (flags & xplm_ControlFlag) != 0,
    }
}

/// Converts an XPLMKeyFlags object into a key event
fn flags_to_event(flags: XPLMKeyFlags) -> Option<KeyEvent> {
    let flags = flags as u32;
    if (flags & xplm_DownFlag) != 0 {
        Some(KeyEvent::KeyDown)
    }
    else if (flags & xplm_UpFlag) != 0 {
        Some(KeyEvent::KeyUp)
    }
    else {
        None
    }
}

/// Converts an XPLM virtual key into a Key
fn vk_to_key(vk: ::libc::c_char) -> Option<Key> {
    match vk as u8 {
        0x08u8 => Some(Key::Back),
        0x09u8 => Some(Key::Tab),
        0x0Cu8 => Some(Key::Clear),
        0x0Du8 => Some(Key::Return),
        0x1Bu8 => Some(Key::Escape),
        0x20u8 => Some(Key::Space),
        0x21u8 => Some(Key::Prior),
        0x22u8 => Some(Key::Next),
        0x23u8 => Some(Key::End),
        0x24u8 => Some(Key::Home),
        0x25u8 => Some(Key::Left),
        0x26u8 => Some(Key::Up),
        0x27u8 => Some(Key::Right),
        0x28u8 => Some(Key::Down),
        0x29u8 => Some(Key::Select),
        0x2Au8 => Some(Key::Print),
        0x2Bu8 => Some(Key::Execute),
        0x2Cu8 => Some(Key::Snapshot),
        0x2Du8 => Some(Key::Insert),
        0x2Eu8 => Some(Key::Delete),
        0x2Fu8 => Some(Key::Help),
        0x30u8 => Some(Key::Key0),
        0x31u8 => Some(Key::Key1),
        0x32u8 => Some(Key::Key2),
        0x33u8 => Some(Key::Key3),
        0x34u8 => Some(Key::Key4),
        0x35u8 => Some(Key::Key5),
        0x36u8 => Some(Key::Key6),
        0x37u8 => Some(Key::Key7),
        0x38u8 => Some(Key::Key8),
        0x39u8 => Some(Key::Key9),
        0x41u8 => Some(Key::A),
        0x42u8 => Some(Key::B),
        0x43u8 => Some(Key::C),
        0x44u8 => Some(Key::D),
        0x45u8 => Some(Key::E),
        0x46u8 => Some(Key::F),
        0x47u8 => Some(Key::G),
        0x48u8 => Some(Key::H),
        0x49u8 => Some(Key::I),
        0x4Au8 => Some(Key::J),
        0x4Bu8 => Some(Key::K),
        0x4Cu8 => Some(Key::L),
        0x4Du8 => Some(Key::M),
        0x4Eu8 => Some(Key::N),
        0x4Fu8 => Some(Key::O),
        0x50u8 => Some(Key::P),
        0x51u8 => Some(Key::Q),
        0x52u8 => Some(Key::R),
        0x53u8 => Some(Key::S),
        0x54u8 => Some(Key::T),
        0x55u8 => Some(Key::U),
        0x56u8 => Some(Key::V),
        0x57u8 => Some(Key::W),
        0x58u8 => Some(Key::X),
        0x59u8 => Some(Key::Y),
        0x5Au8 => Some(Key::Z),
        0x60u8 => Some(Key::Pad0),
        0x61u8 => Some(Key::Pad1),
        0x62u8 => Some(Key::Pad2),
        0x63u8 => Some(Key::Pad3),
        0x64u8 => Some(Key::Pad4),
        0x65u8 => Some(Key::Pad5),
        0x66u8 => Some(Key::Pad6),
        0x67u8 => Some(Key::Pad7),
        0x68u8 => Some(Key::Pad8),
        0x69u8 => Some(Key::Pad9),
        0x6Au8 => Some(Key::Multiply),
        0x6Bu8 => Some(Key::Add),
        0x6Cu8 => Some(Key::Separator),
        0x6Du8 => Some(Key::Subtract),
        0x6Eu8 => Some(Key::Decimal),
        0x6Fu8 => Some(Key::Divide),
        0x70u8 => Some(Key::F1),
        0x71u8 => Some(Key::F2),
        0x72u8 => Some(Key::F3),
        0x73u8 => Some(Key::F4),
        0x74u8 => Some(Key::F5),
        0x75u8 => Some(Key::F6),
        0x76u8 => Some(Key::F7),
        0x77u8 => Some(Key::F8),
        0x78u8 => Some(Key::F9),
        0x79u8 => Some(Key::F10),
        0x7Au8 => Some(Key::F11),
        0x7Bu8 => Some(Key::F12),
        0x7Cu8 => Some(Key::F13),
        0x7Du8 => Some(Key::F14),
        0x7Eu8 => Some(Key::F15),
        0x7Fu8 => Some(Key::F16),
        0x80u8 => Some(Key::F17),
        0x81u8 => Some(Key::F18),
        0x82u8 => Some(Key::F19),
        0x83u8 => Some(Key::F20),
        0x84u8 => Some(Key::F21),
        0x85u8 => Some(Key::F22),
        0x86u8 => Some(Key::F23),
        0x87u8 => Some(Key::F24),
        0xB0u8 => Some(Key::Equal),
        0xB1u8 => Some(Key::Minus),
        0xB2u8 => Some(Key::RightBrace),
        0xB3u8 => Some(Key::LeftBrace),
        0xB4u8 => Some(Key::Quote),
        0xB5u8 => Some(Key::Semicolon),
        0xB6u8 => Some(Key::Backslash),
        0xB7u8 => Some(Key::Comma),
        0xB8u8 => Some(Key::Slash),
        0xB9u8 => Some(Key::Period),
        0xBAu8 => Some(Key::BackQuote),
        0xBBu8 => Some(Key::Enter),
        0xBCu8 => Some(Key::PadEnter),
        0xBDu8 => Some(Key::PadEqual),
        _ => None,
    }
}
