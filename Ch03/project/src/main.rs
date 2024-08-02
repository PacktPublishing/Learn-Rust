fn main() {
    loop {
        print!("Enter an expression (or 'exit' to quit): ");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim().to_string();
        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            break;
        }

        parse_and_calculate(input);
    }
}

fn parse_and_calculate(expression: String) {
    let tokens: Vec<&str> = expression.split_whitespace().collect();

    if tokens.len() != 3 {
        println!("Invalid input format. Please use the format: number operator number");
        return;
    }

    let left_operand: f64 = match tokens[0].parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Failed to parse left operand");
            return;
        }
    };
    let operator = tokens[1];
    let right_operand: f64 = match tokens[2].parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Failed to parse left operand");
            return;
        }
    };

    let result = match operator {
        "+" => left_operand + right_operand,
        "-" => left_operand - right_operand,
        "*" => left_operand * right_operand,
        "/" => {
            if right_operand == 0.0 {
                println!("Division by zero is undefined.");
                return;
            } else {
                left_operand / right_operand
            }
        }
        _ => {
            println!("Unsupported operator. Please use +, -, *, or /");
            return;
        }
    };
    println!("Result: {}", result);
}
