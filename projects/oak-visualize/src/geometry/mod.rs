#![doc = "Basic geometry types for visualization"]

use serde::{Deserialize, Serialize};

/// 2D point
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn origin() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn translate(&self, dx: f64, dy: f64) -> Point {
        Point::new(self.x + dx, self.y + dy)
    }

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
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl Size {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    pub fn square(size: f64) -> Self {
        Self::new(size, size)
    }

    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    pub fn aspect_ratio(&self) -> f64 {
        if self.height != 0.0 { self.width / self.height } else { f64::INFINITY }
    }

    pub fn scale(&self, factor: f64) -> Size {
        Size::new(self.width * factor, self.height * factor)
    }

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
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

impl Rect {
    pub fn new(origin: Point, size: Size) -> Self {
        Self { origin, size }
    }

    pub fn from_xywh(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self::new(Point::new(x, y), Size::new(width, height))
    }

    pub fn from_points(p1: Point, p2: Point) -> Self {
        let min_x = p1.x.min(p2.x);
        let min_y = p1.y.min(p2.y);
        let max_x = p1.x.max(p2.x);
        let max_y = p1.y.max(p2.y);

        Self::from_xywh(min_x, min_y, max_x - min_x, max_y - min_y)
    }

    pub fn x(&self) -> f64 {
        self.origin.x
    }

    pub fn y(&self) -> f64 {
        self.origin.y
    }

    pub fn width(&self) -> f64 {
        self.size.width
    }

    pub fn height(&self) -> f64 {
        self.size.height
    }

    pub fn min_x(&self) -> f64 {
        self.origin.x
    }

    pub fn min_y(&self) -> f64 {
        self.origin.y
    }

    pub fn max_x(&self) -> f64 {
        self.origin.x + self.size.width
    }

    pub fn max_y(&self) -> f64 {
        self.origin.y + self.size.height
    }

    pub fn center(&self) -> Point {
        Point::new(self.origin.x + self.size.width / 2.0, self.origin.y + self.size.height / 2.0)
    }

    pub fn top_left(&self) -> Point {
        self.origin
    }

    pub fn top_right(&self) -> Point {
        Point::new(self.max_x(), self.min_y())
    }

    pub fn bottom_left(&self) -> Point {
        Point::new(self.min_x(), self.max_y())
    }

    pub fn bottom_right(&self) -> Point {
        Point::new(self.max_x(), self.max_y())
    }

    pub fn contains_point(&self, point: Point) -> bool {
        point.x >= self.min_x() && point.x <= self.max_x() && point.y >= self.min_y() && point.y <= self.max_y()
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        !(self.max_x() < other.min_x()
            || other.max_x() < self.min_x()
            || self.max_y() < other.min_y()
            || other.max_y() < self.min_y())
    }

    pub fn union(&self, other: &Rect) -> Rect {
        let min_x = self.min_x().min(other.min_x());
        let min_y = self.min_y().min(other.min_y());
        let max_x = self.max_x().max(other.max_x());
        let max_y = self.max_y().max(other.max_y());

        Rect::from_xywh(min_x, min_y, max_x - min_x, max_y - min_y)
    }

    pub fn translate(&self, dx: f64, dy: f64) -> Rect {
        Rect::new(self.origin.translate(dx, dy), self.size)
    }

    pub fn scale(&self, factor: f64) -> Rect {
        Rect::new(self.origin.scale(factor), self.size.scale(factor))
    }

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
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    pub a: f64, // scale_x
    pub b: f64, // skew_y
    pub c: f64, // skew_x
    pub d: f64, // scale_y
    pub e: f64, // translate_x
    pub f: f64, // translate_y
}

impl Transform {
    pub fn identity() -> Self {
        Self { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: 0.0, f: 0.0 }
    }

    pub fn translate(x: f64, y: f64) -> Self {
        Self { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: x, f: y }
    }

    pub fn scale(x: f64, y: f64) -> Self {
        Self { a: x, b: 0.0, c: 0.0, d: y, e: 0.0, f: 0.0 }
    }

    pub fn rotate(angle: f64) -> Self {
        let cos_a = angle.cos();
        let sin_a = angle.sin();

        Self { a: cos_a, b: sin_a, c: -sin_a, d: cos_a, e: 0.0, f: 0.0 }
    }

    pub fn transform_point(&self, point: Point) -> Point {
        Point::new(self.a * point.x + self.c * point.y + self.e, self.b * point.x + self.d * point.y + self.f)
    }

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
