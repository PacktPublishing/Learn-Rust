#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 10, y: 20 };
    let m3 = Message::Write(String::from("Hello"));
    let m4 = Message::ChangeColor(255, 0, 0);

    println!("{:?}", m1);
    println!("{:?}", m2);
    println!("{:?}", m3);
    println!("{:?}", m4);
}
