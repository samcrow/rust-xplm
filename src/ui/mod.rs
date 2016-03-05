// Copyright (c) 2015 rust-xplm developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

//! User-interface-related types
//!
//! The 2D coordinate system used in X-Plane uses units of screen pixels. The origin is at the
//! bottom left corner of the X-Plane window. Positive X values are right, positive Y values are
//! up.
//!

/// Defines widget-related types
pub mod widget;

/// A point on the screen in X-Plane
#[derive(Debug,Clone,PartialEq)]
pub struct Point {
    /// X coordinate
    pub x: i32,
    /// Y coordinate
    pub y: i32,
}

/// A rectangle on the screen in X-Plane
///
#[derive(Debug,Clone,PartialEq)]
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

impl Rect {
    /// Returns a new rectangle dilated by a specified amount
    ///
    /// To dilate by amount x, the following changes are applied:
    ///
    /// * top increases by x
    /// * bottom decreases by x
    /// * left decreases by x
    /// * right increases by x
    ///
    /// Positive values make the rectangle larger, negative values make it smaller.
    /// If a dilation operation would result in a rectangle with left > right or bottom > top,
    /// the returned rectangle will have zero width and/or zero height.
    pub fn dilate(&self, amount: i32) -> Rect {
        let mut new_left = self.left - amount;
        let mut new_right = self.right + amount;
        let mut new_top = self.top + amount;
        let mut new_bottom = self.bottom - amount;
        let center = self.center();
        if new_left > new_right {
            new_left = center.x;
            new_right = center.x;
        }
        if new_bottom > new_top {
            new_bottom = center.y;
            new_top = center.y;
        }
        Rect {
            left: new_left,
            top: new_top,
            right: new_right,
            bottom: new_bottom,
        }
    }

    /// Returns the lower left corner of this rectangle
    pub fn lower_left(&self) -> Point {
        Point {
            x: self.left,
            y: self.bottom,
        }
    }
    /// Returns the width of this rectangle
    pub fn width(&self) -> i32 {
        self.right - self.left
    }
    /// Returns the height of this rectangle
    pub fn height(&self) -> i32 {
        self.top - self.bottom
    }

    /// Returns the center point of this rectangle
    pub fn center(&self) -> Point {
        Point {
            x: (self.right + self.left) / 2,
            y: (self.top + self.bottom) / 2,
        }
    }
}


/// Possible cursor states to set
#[derive(Debug,Clone,PartialEq)]
pub enum Cursor {
    /// Allows X-Plane to set the cursor
    Default,
    /// Hides the cursor
    Hidden,
    /// Makes the cursor an arrow
    Arrow,
}

/// Events that a mouse action can create
#[derive(Debug,Clone,PartialEq)]
pub enum MouseEvent {
    /// The mouse button was pressed down
    Pressed,
    /// The mouse was moved while held down
    Dragged,
    /// The mouse button was released
    Released,
}

/// Events that a key action can create
#[derive(Debug,Clone,PartialEq)]
pub enum KeyEvent {
    /// A key was pressed
    KeyDown,
    /// A key was released
    KeyUp,
}

/// Defines modifier keys that may be pressed
#[derive(Debug,Clone,PartialEq)]
pub struct ModifierKeys {
    /// If the shift key is pressed
    pub shift: bool,
    /// If the option key is pressed
    pub option: bool,
    /// If the control key is pressed
    pub control: bool,
}

/// Defines keys
#[derive(Debug,Clone,PartialEq)]
pub enum Key {
    Back,
    Tab,
    Clear,
    Return,
    Escape,
    Space,
    Prior,
    Next,
    End,
    Home,
    Left,
    Up,
    Right,
    Down,
    Select,
    Print,
    Execute,
    Snapshot,
    Insert,
    Delete,
    Help,
    /// Key0-Key9 correspond to the 0-9 keys at the top of a keyboard
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    /// Pad0-Pad9 correspond to the 0-9 keys on the number pad
    Pad0,
    Pad1,
    Pad2,
    Pad3,
    Pad4,
    Pad5,
    Pad6,
    Pad7,
    Pad8,
    Pad9,
    Multiply,
    Add,
    Separator,
    Subtract,
    Decimal,
    Divide,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    Equal,
    Minus,
    RightBrace,
    LeftBrace,
    Quote,
    Semicolon,
    Backslash,
    Comma,
    Slash,
    Period,
    BackQuote,
    /// The enter or return key on the keyboard
    Enter,
    /// The enter or return key on the numerical keypad
    PadEnter,
    /// The equal key on the numerical keypad
    PadEqual,
}
