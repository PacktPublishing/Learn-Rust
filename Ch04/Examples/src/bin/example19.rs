#[derive(Debug)]
#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(255, 0, 0);

    match msg {
        Message::Quit => println!("The Quit variant has no data."),
        Message::Move { x, y } => println!("Move to: {}, y: {}", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change color to r: {}, g: {}, b: {}", r, g, b)
        }
    }
}
