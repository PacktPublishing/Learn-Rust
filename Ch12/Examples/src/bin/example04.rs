enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(10, Box::new(Cons(20, Box::new(Cons(30, Box::new(Nil))))));

    let second_payload = match list {
        Cons(_, next) => match *next {
            Cons(payload, _) => payload,
            _ => 0,
        },
        _ => 0,
    };

    println!("the second payload is: {}", second_payload);
}
