//!
//! User interface geometry
//!
//! The 2D coordinate system in X-Plane has its origin in the lower left corner of the window.
//! Its units are pixels. [More information about X-Plane's coordinate systems is available here]
//! (http://www.xsquawkbox.net/xpsdk/mediawiki/ScreenCoordinates)
//!

/// A 2-dimensional rectangle
#[derive(Debug, Copy, Clone)]
pub struct Rect<N> {
    /// The top coordinate
    top: N,
    /// The bottom coordinate
    bottom: N,
    /// The left coordinate
    left: N,
    /// The right coordinate
    right: N,
}

impl<N> Rect<N> {
    /// Creates a rectangle from left, top, right, and bottom coordinates
    pub fn from_left_top_right_bottom(left: N, top: N, right: N, bottom: N) -> Self {
        Rect {
            top: top,
            bottom: bottom,
            left: left,
            right: right,
        }
    }
    /// Creates a rectangle from a top left corner and a bottom right corner
    pub fn from_corners(top_left: Point<N>, bottom_right: Point<N>) -> Self {
        let (left, top) = top_left.into_xy();
        let (bottom, right) = bottom_right.into_xy();
        Rect {
            top: top,
            bottom: bottom,
            left: left,
            right: right,
        }
    }
    /// Consumes this rectangle and returns its left, top, bottom, and right coordinates
    pub fn into_left_top_bottom_right(self) -> (N, N, N, N) {
        (self.left, self.top, self.bottom, self.right)
    }

    pub fn set_top(&mut self, top: N) {
        self.top = top;
    }
    pub fn set_left(&mut self, left: N) {
        self.left = left;
    }
    pub fn set_bottom(&mut self, bottom: N) {
        self.bottom = bottom;
    }
    pub fn set_right(&mut self, right: N) {
        self.right = right;
    }

    /// Determines whether this rectangle contains a point
    ///
    /// For this calculation, the bottom and left edges are inside the rectangle, while the
    /// top and right edges are outside.
    pub fn contains(&self, point: Point<N>) -> bool
    where
        N: PartialOrd,
    {
        let (x, y) = point.into_xy();
        x >= self.left && x < self.right && y >= self.bottom && y < self.top
    }
}

impl<N: Clone> Rect<N> {
    pub fn top(&self) -> N {
        self.top.clone()
    }
    pub fn bottom(&self) -> N {
        self.bottom.clone()
    }
    pub fn left(&self) -> N {
        self.left.clone()
    }
    pub fn right(&self) -> N {
        self.right.clone()
    }
}

/// A 2D point
#[derive(Debug, Copy, Clone)]
pub struct Point<N> {
    /// The X coordinate
    x: N,
    /// The Y coordinate
    y: N,
}

impl<N> Point<N> {
    /// Creates a point from X and Y coordinates
    pub fn from_xy(x: N, y: N) -> Self {
        Point { x: x, y: y }
    }
    pub fn set_x(&mut self, x: N) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: N) {
        self.y = y;
    }
    pub fn into_xy(self) -> (N, N) {
        (self.x, self.y)
    }
}

impl<N: Clone> Point<N> {
    pub fn x(&self) -> N {
        self.x.clone()
    }
    pub fn y(&self) -> N {
        self.y.clone()
    }
}

impl<N> From<(N, N)> for Point<N> {
    /// Converts an (x, y) pair into a point
    fn from((x, y): (N, N)) -> Self {
        Point::from_xy(x, y)
    }
}
