#[derive(Debug, Copy, Clone)]
struct Point2D {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl From<Point2D> for Point3D {
    fn from(point: Point2D) -> Self {
        Point3D { x: point.x, y: point.y, z: 0 }
    }
}

fn main() {
    let point2d = Point2D { x: 5, y: 10 };

    let point3d = Point3D::from(point2d);
    println!("{:?}", point3d);

    let point3d: Point3D = point2d.into();
    println!("{:?}", point3d);
}
