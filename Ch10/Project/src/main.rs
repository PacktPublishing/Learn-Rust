use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

fn main() {
    loop {
        println!("Welcome to the User Registration System!");
        println!("1. Register");
        println!("2. Login");
        println!("3. Exit");

        let choice = get_user_input("Choose an option (1, 2, 3):");

        match choice.as_str() {
            "1" => register_user(),
            "2" => login_user(),
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option, please try again."),
        }
    }
}

fn register_user() {
    let username = get_user_input("Enter a username:");
    let password = get_user_input("Enter a password:");

    // Validate username and password
    if !validate_username(&username) {
        println!("Error: Username must be at least 3 characters long.");
        return;
    }

    if !validate_password(&password) {
        println!("Error: Password must be at least 8 characters long.");
        return;
    }

    // Check if username already exists
    if let Ok(file) = File::open("credentials.txt") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(entry) = line {
                if entry.starts_with(&format!("{}:", username)) {
                    println!("Error: Username already exists.");
                    return;
                }
            }
        }
    }

    // Encrypt password
    let encrypted_password = encrypt_password(&password);

    // Append username and encrypted password to the file
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("credentials.txt")
    {
        writeln!(file, "{}:{}", username, encrypted_password).unwrap();
        println!("Registration successful!");
    } else {
        println!("Error: Unable to write to credentials file.");
    }
}

fn login_user() {
    let username = get_user_input("Enter your username:");
    let password = get_user_input("Enter your password:");

    // Check if the user exists and verify the password
    if let Ok(file) = File::open("credentials.txt") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(entry) = line {
                if entry.starts_with(&format!("{}:", username)) {
                    let stored_password = entry.split(':').nth(1).unwrap_or("");
                    if stored_password == encrypt_password(&password) {
                        println!("Login successful!");
                        return;
                    } else {
                        println!("Error: Incorrect password.");
                        return;
                    }
                }
            }
        }
        println!("Error: Username not found.");
    } else {
        println!("Error: Unable to read credentials file.");
    }
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn validate_username(username: &str) -> bool {
    username.len() >= 3
}

fn validate_password(password: &str) -> bool {
    password.len() >= 8
}

fn encrypt_password(password: &str) -> String {
    let mut hex_output = String::new(); // Initialize an empty string for the hex output
    let bytes = password.as_bytes(); // Get the byte slice of the string

    for byte in bytes {
        // Convert each byte to a two-digit hex value and append to the output
        hex_output.push_str(&format!("{:02x}", byte));
    }

    hex_output // Return the final hex string
}
