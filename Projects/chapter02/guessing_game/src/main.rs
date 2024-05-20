use rand::Rng;
use std::io;
use std::io::Write;

const MAX_ATTEMPTS:i32 = 8;

fn main() {
    println!("Guess the number!");
    println!("=================");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;

    loop {
        println!("");
        print!("What is your guess? ");
        io::stdout().flush().ok();

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        attempts += 1;

        if attempts > MAX_ATTEMPTS {
            println!("You've used all {} attempts. You lose! The correct number was {}", MAX_ATTEMPTS, secret_number);
            break;
        }

        if guess < secret_number {
            println!("Too low!");
        } else if guess > secret_number {
            println!("Too high!");
        } else {
            println!("Correct! You guessed the number in {} attempts!", attempts);
            break;
        }
    }
}
