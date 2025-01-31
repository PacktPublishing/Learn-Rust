struct Message<'a, 'b> {
    sender: &'a str,
    receiver: &'b str,
}

fn main() {
    let message = Message {
        sender: "Alice",
        receiver: "Bob",
    };

    println!("{}", message.sender);
    println!("{}", message.receiver);
}