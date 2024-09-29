fn main() {
    greetings::say_hello();

    // Try to call the private whisper_hello function (this will result in a compile-time error)
    // greetings::whisper_hello();
}

mod greetings {
    pub fn say_hello() {
        println!("Hello, world!");
        whisper_hello();  // This is allowed because it's called from within the module
    }

    fn whisper_hello() {
        println!("(whispers) Hello, world!");
    }
}
