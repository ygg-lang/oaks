#![doc = "Basic geometry types for visualization"]

/// 2D point
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Point {
    /// The x-coordinate.
    pub x: f64,
    /// The y-coordinate.
    pub y: f64,
}

impl Point {
    /// Creates a new `Point`.
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Creates a `Point` at the origin (0, 0).
    pub fn origin() -> Self {
        Self::new(0.0, 0.0)
    }

    /// Calculates the Euclidean distance to another point.
    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Returns a new point translated by the given deltas.
    pub fn translate(&self, dx: f64, dy: f64) -> Point {
        Point::new(self.x + dx, self.y + dy)
    }

    /// Returns a new point scaled by the given factor.
    pub fn scale(&self, factor: f64) -> Point {
        Point::new(self.x * factor, self.y * factor)
    }
}

impl Default for Point {
    fn default() -> Self {
        Self::origin()
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

/// 2D size
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Size {
    /// The width.
    pub width: f64,
    /// The height.
    pub height: f64,
}

impl Size {
    /// Creates a new `Size`.
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    /// Creates a square `Size`.
    pub fn square(size: f64) -> Self {
        Self::new(size, size)
    }

    /// Calculates the area of the size.
    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    /// Calculates the aspect ratio (width / height).
    pub fn aspect_ratio(&self) -> f64 {
        if self.height != 0.0 { self.width / self.height } else { f64::INFINITY }
    }

    /// Returns a new size scaled by the given factor.
    pub fn scale(&self, factor: f64) -> Size {
        Size::new(self.width * factor, self.height * factor)
    }

    /// Returns a new size scaled by the given factors for width and height.
    pub fn scale_xy(&self, x_factor: f64, y_factor: f64) -> Size {
        Size::new(self.width * x_factor, self.height * y_factor)
    }
}

impl Default for Size {
    fn default() -> Self {
        Self::new(100.0, 50.0)
    }
}

/// Rectangle defined by position and size
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rect {
    /// The origin point of the rectangle.
    pub origin: Point,
    /// The size of the rectangle.
    pub size: Size,
}

impl Rect {
    /// Creates a new `Rect`.
    pub fn new(origin: Point, size: Size) -> Self {
        Self { origin, size }
    }

    /// Creates a new `Rect` from x, y, width, and height.
    pub fn from_xywh(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self::new(Point::new(x, y), Size::new(width, height))
    }

    /// Creates a new `Rect` from two points.
    pub fn from_points(p1: Point, p2: Point) -> Self {
        let min_x = p1.x.min(p2.x);
        let min_y = p1.y.min(p2.y);
        let max_x = p1.x.max(p2.x);
        let max_y = p1.y.max(p2.y);

        Self::from_xywh(min_x, min_y, max_x - min_x, max_y - min_y)
    }

    /// Gets the x-coordinate of the origin.
    pub fn x(&self) -> f64 {
        self.origin.x
    }

    /// Gets the y-coordinate of the origin.
    pub fn y(&self) -> f64 {
        self.origin.y
    }

    /// Gets the width of the rectangle.
    pub fn width(&self) -> f64 {
        self.size.width
    }

    /// Gets the height of the rectangle.
    pub fn height(&self) -> f64 {
        self.size.height
    }

    /// Gets the minimum x-coordinate.
    pub fn min_x(&self) -> f64 {
        self.origin.x
    }

    /// Gets the minimum y-coordinate.
    pub fn min_y(&self) -> f64 {
        self.origin.y
    }

    /// Gets the maximum x-coordinate.
    pub fn max_x(&self) -> f64 {
        self.origin.x + self.size.width
    }

    /// Gets the maximum y-coordinate.
    pub fn max_y(&self) -> f64 {
        self.origin.y + self.size.height
    }

    /// Gets the center point of the rectangle.
    pub fn center(&self) -> Point {
        Point::new(self.origin.x + self.size.width / 2.0, self.origin.y + self.size.height / 2.0)
    }

    /// Gets the top-left point.
    pub fn top_left(&self) -> Point {
        self.origin
    }

    /// Gets the top-right point.
    pub fn top_right(&self) -> Point {
        Point::new(self.max_x(), self.min_y())
    }

    /// Gets the bottom-left point.
    pub fn bottom_left(&self) -> Point {
        Point::new(self.min_x(), self.max_y())
    }

    /// Gets the bottom-right point.
    pub fn bottom_right(&self) -> Point {
        Point::new(self.max_x(), self.max_y())
    }

    /// Checks if a point is inside the rectangle.
    pub fn contains_point(&self, point: Point) -> bool {
        point.x >= self.min_x() && point.x <= self.max_x() && point.y >= self.min_y() && point.y <= self.max_y()
    }

    /// Checks if this rectangle intersects with another.
    pub fn intersects(&self, other: &Rect) -> bool {
        !(self.max_x() < other.min_x() || other.max_x() < self.min_x() || self.max_y() < other.min_y() || other.max_y() < self.min_y())
    }

    /// Returns the union of this rectangle and another.
    pub fn union(&self, other: &Rect) -> Rect {
        let min_x = self.min_x().min(other.min_x());
        let min_y = self.min_y().min(other.min_y());
        let max_x = self.max_x().max(other.max_x());
        let max_y = self.max_y().max(other.max_y());

        Rect::from_xywh(min_x, min_y, max_x - min_x, max_y - min_y)
    }

    /// Returns a new rectangle translated by the given deltas.
    pub fn translate(&self, dx: f64, dy: f64) -> Rect {
        Rect::new(self.origin.translate(dx, dy), self.size)
    }

    /// Returns a new rectangle scaled by the given factor.
    pub fn scale(&self, factor: f64) -> Rect {
        Rect::new(self.origin.scale(factor), self.size.scale(factor))
    }

    /// Returns a new rectangle expanded by the given margin on all sides.
    pub fn expand(&self, margin: f64) -> Rect {
        Rect::from_xywh(self.x() - margin, self.y() - margin, self.width() + 2.0 * margin, self.height() + 2.0 * margin)
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self::new(Point::default(), Size::default())
    }
}

/// 2D transformation matrix
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Transform {
    /// Scale factor in x direction.
    pub a: f64,
    /// Skew factor in y direction.
    pub b: f64,
    /// Skew factor in x direction.
    pub c: f64,
    /// Scale factor in y direction.
    pub d: f64,
    /// Translation in x direction.
    pub e: f64,
    /// Translation in y direction.
    pub f: f64,
}

impl Transform {
    /// Returns the identity transformation.
    pub fn identity() -> Self {
        Self { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: 0.0, f: 0.0 }
    }

    /// Creates a translation transformation.
    pub fn translate(x: f64, y: f64) -> Self {
        Self { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: x, f: y }
    }

    /// Creates a scale transformation.
    pub fn scale(x: f64, y: f64) -> Self {
        Self { a: x, b: 0.0, c: 0.0, d: y, e: 0.0, f: 0.0 }
    }

    /// Creates a rotation transformation.
    pub fn rotate(angle: f64) -> Self {
        let cos_a = angle.cos();
        let sin_a = angle.sin();

        Self { a: cos_a, b: sin_a, c: -sin_a, d: cos_a, e: 0.0, f: 0.0 }
    }

    /// Applies the transformation to a point.
    pub fn transform_point(&self, point: Point) -> Point {
        Point::new(self.a * point.x + self.c * point.y + self.e, self.b * point.x + self.d * point.y + self.f)
    }

    /// Composes this transformation with another.
    pub fn compose(&self, other: &Transform) -> Transform {
        Transform {
            a: self.a * other.a + self.b * other.c,
            b: self.a * other.b + self.b * other.d,
            c: self.c * other.a + self.d * other.c,
            d: self.c * other.b + self.d * other.d,
            e: self.e * other.a + self.f * other.c + other.e,
            f: self.e * other.b + self.f * other.d + other.f,
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}
