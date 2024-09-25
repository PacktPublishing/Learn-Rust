trait Identifiable  {
    fn id(&self) -> i32;
}

struct User {
    id: i32,
}

impl Identifiable for User {
    fn id(&self) -> i32 {
        self.id
    }
}

fn main() {
    let user = User {
        id: 12,
    };

    println!("{}", user.id());
}
