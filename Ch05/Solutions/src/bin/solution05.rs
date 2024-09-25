use std::ops::Add;

#[derive(Debug)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Add for Point3D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

fn main() {
    let v1 = Point3D { x: 3.0, y: 4.0, z: 5.0 };
    let v2 = Point3D { x: 1.0, y: 2.0, z: 3.0 };

    let v3 = v1 + v2;

    println!("{:#?}", v3);
}
