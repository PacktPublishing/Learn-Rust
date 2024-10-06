use std::env;

fn main() {
    let mut args = env::args();
    for arg in args.skip(1) {
        let parsed_arg: i32 = arg.parse().unwrap();
        println!("Parsed argument: {}", parsed_arg);
    }
}