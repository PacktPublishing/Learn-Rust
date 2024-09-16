struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3D {
    fn new(x: f64, y: f64, z: f64) -> Point3D {
        Point3D { x, y, z }
    }
}

fn main() {
    let point = Point3D::new(1.0, 2.0, 3.0);
    println!("Point3D({}, {}, {})", point.x, point.y, point.z);
}
