enum Message {
    Text(String),
    Move { x: i32, y: i32 },
    Quit,
}

impl Message {
    fn print_message(&self) {
        match self {
            Message::Text(text) => println!("Text message: {}", text),
            Message::Move { x, y } => println!("Move to coordinates: x = {}, y = {}", x, y),
            Message::Quit => println!("Quit message received."),
        }
    }
}
fn main() {
    let msg1 = Message::Text(String::from("Hello, World!"));
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Quit;

    msg1.print_message();
    msg2.print_message();
    msg3.print_message();
}
