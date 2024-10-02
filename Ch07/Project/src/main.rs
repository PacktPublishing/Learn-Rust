use rand::Rng;
use std::io;
use std::num::ParseFloatError;

fn main() {
    println!("Welcome to the Math Quiz Game!");
    println!("Solve the problems or type 'exit' to quit.\n");

    let operators = vec!["+", "-", "*", "/"];
    let mut total_score = 0;

    for (round, &operator) in operators.iter().enumerate() {
        println!("\n--- Round {}: {} ---", round + 1, operator);
        let mut score = 0;

        for question in 1..=10 {
            let max_value = score + 1; // Ensure difficulty increases with score.
            let (problem, correct_answer, round_score) = generate_problem(operator, score + 1, question, max_value);

            println!("Problem {}: {}", question, problem);

            // User input handling
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            let input = input.trim();

            if input.eq_ignore_ascii_case("exit") {
                println!("You chose to exit. Your final score: {}", total_score + score);
                return;
            }

            match validate_input(input) {
                Ok(answer) => {
                    if (answer - correct_answer).abs() < 1e-5 {
                        println!("Correct!");
                        score += round_score;
                        println!("Your current round score: {}\n", score);
                    } else {
                        println!("Incorrect! The correct answer was: {}\n", correct_answer);
                        break; // Move to the next round if incorrect
                    }
                }
                Err(e) => {
                    println!("Error: {}. Please enter a valid number.\n", e);
                }
            }
        }

        total_score += score;
        println!("Round {} finished! Your total score is: {}\n", round + 1, total_score);
    }

    println!("Game Over! Your final score: {}", total_score);
}

fn generate_problem(operator: &str, score: i32, question: usize, max_value: i32) -> (String, f64, i32) {
    let mut rng = rand::thread_rng();

    if question == 1 {
        return match operator {
            "+" => (format!("1 + 1"), 2.0, 2),
            "-" => (format!("1 - 1"), 0.0, 1),
            "*" => (format!("1 * 1"), 1.0, 1),
            "/" => (format!("1 / 1"), 1.0, 2), // Score is 2 as we are multiplying by divisor and quotient
            _ => unreachable!(),
        };
    }

    let (a, b) = (rng.gen_range(1..=max_value), rng.gen_range(1..=max_value));

    match operator {
        "+" => (format!("{} + {}", a, b), (a + b) as f64, a + b),
        "-" => (format!("{} - {}", a, b), (a - b) as f64, a + b),
        "*" => (format!("{} * {}", a, b), (a * b) as f64, a + b),
        "/" => {
            // For division, generate b and c, and calculate a.
            let c = rng.gen_range(1..=max_value);
            let a = b * c;
            (format!("{} / {}", a, b), c as f64, b + c)
        }
        _ => unreachable!(),
    }
}

fn validate_input(input: &str) -> Result<f64, String> {
    input.parse::<f64>().map_err(|e: ParseFloatError| format!("Invalid input: {}", e))
}
