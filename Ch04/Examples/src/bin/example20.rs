#[derive(Debug)]
#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::Write(String::from("Hello, world!"));
    if let Message::Write(text) = msg {
        println!("Text message: {}", text);
    } else {
        println!("Not a Write message.");
    }
}