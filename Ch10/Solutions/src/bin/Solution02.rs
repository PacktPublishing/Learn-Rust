use std::fs;
use std::io;

fn main() -> io::Result<()> {
    // Specify the input and output file paths
    let input_file = "input02.txt";
    let output_file = "output02.txt";

    // Read the contents of the input file
    let contents = fs::read_to_string(input_file)?;

    // Reverse the content
    let mut reversed_content = String::new();
    for char in contents.chars().rev() {
        reversed_content.push(char);
    }

    // Write the reversed content to the output file
    fs::write(output_file, reversed_content)?;

    println!("Reversed content written to '{}'", output_file);
    Ok(())
}
