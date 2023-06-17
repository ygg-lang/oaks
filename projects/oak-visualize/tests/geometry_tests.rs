use oak_visualize::geometry::{Point, Rect, Size, Transform};

#[test]
fn test_point_operations() {
    let p1 = Point::new(1.0, 2.0);
    let p2 = Point::new(3.0, 4.0);

    assert_eq!(p1 + p2, Point::new(4.0, 6.0));
    assert_eq!(p2 - p1, Point::new(2.0, 2.0));
    assert_eq!(p1.distance_to(&p2), (8.0_f64).sqrt());
}

#[test]
fn test_point_creation() {
    let point = Point::new(5.0, 10.0);
    assert_eq!(point.x, 5.0);
    assert_eq!(point.y, 10.0);

    let origin = Point::origin();
    assert_eq!(origin.x, 0.0);
    assert_eq!(origin.y, 0.0);
}

#[test]
fn test_point_transformations() {
    let point = Point::new(2.0, 3.0);

    let translated = point.translate(1.0, 2.0);
    assert_eq!(translated, Point::new(3.0, 5.0));

    let scaled = point.scale(2.0);
    assert_eq!(scaled, Point::new(4.0, 6.0));
}

#[test]
fn test_size_operations() {
    let size = Size::new(10.0, 20.0);
    assert_eq!(size.width, 10.0);
    assert_eq!(size.height, 20.0);
    assert_eq!(size.area(), 200.0);
    assert_eq!(size.aspect_ratio(), 0.5);

    let square = Size::square(5.0);
    assert_eq!(square.width, 5.0);
    assert_eq!(square.height, 5.0);
    assert_eq!(square.aspect_ratio(), 1.0);
}

#[test]
fn test_size_scaling() {
    let size = Size::new(4.0, 6.0);

    let scaled = size.scale(2.0);
    assert_eq!(scaled, Size::new(8.0, 12.0));

    let scaled_xy = size.scale_xy(2.0, 3.0);
    assert_eq!(scaled_xy, Size::new(8.0, 18.0));
}

#[test]
fn test_rect_operations() {
    let rect1 = Rect::from_xywh(0.0, 0.0, 10.0, 10.0);
    let rect2 = Rect::from_xywh(5.0, 5.0, 10.0, 10.0);

    assert!(rect1.intersects(&rect2));
    assert!(rect1.contains_point(Point::new(5.0, 5.0)));
    assert!(!rect1.contains_point(Point::new(15.0, 15.0)));

    let union = rect1.union(&rect2);
    assert_eq!(union, Rect::from_xywh(0.0, 0.0, 15.0, 15.0));
}

#[test]
fn test_rect_creation() {
    let origin = Point::new(1.0, 2.0);
    let size = Size::new(10.0, 20.0);
    let rect = Rect::new(origin, size);

    assert_eq!(rect.x(), 1.0);
    assert_eq!(rect.y(), 2.0);
    assert_eq!(rect.width(), 10.0);
    assert_eq!(rect.height(), 20.0);

    let rect_xywh = Rect::from_xywh(1.0, 2.0, 10.0, 20.0);
    assert_eq!(rect, rect_xywh);
}

#[test]
fn test_rect_corners() {
    let rect = Rect::from_xywh(10.0, 20.0, 30.0, 40.0);

    assert_eq!(rect.top_left(), Point::new(10.0, 20.0));
    assert_eq!(rect.top_right(), Point::new(40.0, 20.0));
    assert_eq!(rect.bottom_left(), Point::new(10.0, 60.0));
    assert_eq!(rect.bottom_right(), Point::new(40.0, 60.0));
    assert_eq!(rect.center(), Point::new(25.0, 40.0));
}

#[test]
fn test_rect_transformations() {
    let rect = Rect::from_xywh(0.0, 0.0, 10.0, 10.0);

    let translated = rect.translate(5.0, 5.0);
    assert_eq!(translated, Rect::from_xywh(5.0, 5.0, 10.0, 10.0));

    let scaled = rect.scale(2.0);
    assert_eq!(scaled, Rect::from_xywh(0.0, 0.0, 20.0, 20.0));

    let expanded = rect.expand(2.0);
    assert_eq!(expanded, Rect::from_xywh(-2.0, -2.0, 14.0, 14.0));
}

#[test]
fn test_transform() {
    let transform = Transform::translate(10.0, 20.0);
    let point = Point::new(5.0, 5.0);
    let transformed = transform.transform_point(point);

    assert_eq!(transformed, Point::new(15.0, 25.0));
}

#[test]
fn test_transform_identity() {
    let identity = Transform::identity();
    let point = Point::new(5.0, 10.0);
    let transformed = identity.transform_point(point);

    assert_eq!(transformed, point);
}

#[test]
fn test_transform_scale() {
    let scale = Transform::scale(2.0, 3.0);
    let point = Point::new(4.0, 5.0);
    let transformed = scale.transform_point(point);

    assert_eq!(transformed, Point::new(8.0, 15.0));
}

#[test]
fn test_transform_compose() {
    let translate = Transform::translate(5.0, 10.0);
    let scale = Transform::scale(2.0, 2.0);
    let composed = translate.compose(&scale);

    let point = Point::new(1.0, 1.0);
    let transformed = composed.transform_point(point);

    // First scale (1,1) -> (2,2), then translate (2,2) -> (7,12)
    assert_eq!(transformed, Point::new(7.0, 12.0));
}
