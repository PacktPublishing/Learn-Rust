use std::io::{self, Write};
use rand::Rng;

const MAX_ATTEMPTS: usize = 6;
const WORD_LIST: [&str; 5] = ["rust", "hangman", "developer", "cargo", "crate"];
const STAGES: [&str; 7] = [
    r#"
  +---+
  |   |
      |
      |
      |
      |
========="#,
    r#"
  +---+
  |   |
  O   |
      |
      |
      |
========="#,
    r#"
  +---+
  |   |
  O   |
  |   |
      |
      |
========="#,
    r#"
  +---+
  |   |
  O   |
 /|   |
      |
      |
========="#,
    r#"
  +---+
  |   |
  O   |
 /|\  |
      |
      |
========="#,
    r#"
  +---+
  |   |
  O   |
 /|\  |
 /    |
      |
========="#,
    r#"
  +---+
  |   |
  O   |
 /|\  |
 / \  |
      |
========="#,
];

fn main() {
    loop {
        let secret_word = choose_random_word();
        play_game(secret_word);

        println!("Would you like to play again? (y/n):");
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Failed to read input.");
        if answer.trim().to_lowercase() != "y" {
            break;
        }
    }
}

fn choose_random_word() -> String {
    let mut rng = rand::thread_rng();
    let choice = rng.gen_range(0..WORD_LIST.len());
    WORD_LIST[choice].to_string()
}

fn play_game(secret_word: String) {
    let mut attempts_left = MAX_ATTEMPTS;
    let mut guessed_letters = String::new();
    let mut correct_guesses = "_".repeat(secret_word.len());

    while attempts_left > 0 && correct_guesses.contains("_") {
        display_state(&correct_guesses, &guessed_letters, STAGES[MAX_ATTEMPTS - attempts_left]);

        let guess = get_guess(&guessed_letters);
        guessed_letters.push(guess);

        if check_guess(&secret_word, guess) {
            for (i, c) in secret_word.char_indices() {
                if c == guess {
                    correct_guesses.replace_range(i..=i, &guess.to_string());
                }
            }
        } else {
            attempts_left -= 1;
        }
    }

    if !correct_guesses.contains('_') {
        println!("Congratulations! You guessed the word: {}", secret_word);
    } else {
        println!("{}", STAGES[MAX_ATTEMPTS]);
        println!("Game over! The word was: {}", secret_word);
    }
}

fn display_state(correct_guesses: &str, guessed_letters: &str, stage: &str) {
    println!("{}", stage);
    println!("Word: {}", correct_guesses);
    println!("Guessed letters: {:?}", guessed_letters);
    println!("===============================");
}

fn get_guess(guessed_letters: &str) -> char {
    loop {
        print!("Enter your guess: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input.");

        let input = input.trim();
        if input.len() == 1 {
            let guess = input.chars().next().unwrap();
            if !guessed_letters.contains(guess) && guess.is_alphabetic() {
                return guess;
            } else {
                println!("You already guessed that letter or it's invalid!");
            }
        } else {
            println!("Please enter a single letter.");
        }
    }
}

fn check_guess(secret_word: &str, guess: char) -> bool {
    for c in secret_word.chars() {
        if c == guess {
            return true;
        }
    }
    false
}