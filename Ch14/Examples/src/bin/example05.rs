fn print_static(data: &'static str) {
    println!("{}", data);
}

fn main() {
    let long_lived_data = "I live forever!";
    print_static(long_lived_data);

    let temporary_string = String::from("Temporary data");
    // print_static(&temporary_string); // This will not compile.
}
