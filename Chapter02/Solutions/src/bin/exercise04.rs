fn main() {
    let mut number = 1;

    loop {
        println!("{}", number);

        if number == 10 {
            break;
        }

        number += 1;
    }
}
