struct Message<'a, 'b> {
    from: &'a String,
    to: &'b String,
}

fn main() {
    let from_unpacked;
    let to_unpacked;
    let message: Message;
    {
        let from = &String::from("word");
        {
            
            let to = &String::from("hello");
            {
                message = Message { from, to };

                from_unpacked = message.from;
                to_unpacked = message.to;
            }
            println!("{to_unpacked}");
        }
        println!("{from_unpacked}");
    }
}