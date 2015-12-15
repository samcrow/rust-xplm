
/// Defines widget-related types
pub mod widget;


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
