use std::error::Error;
use std::fs::File;
use std::io::Read;

fn read_and_parse_file(path: &str) -> Result<i32, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let num: i32 = contents.trim().parse()?;
    Ok(num)
}

fn main() -> Result<(), Box<dyn Error>> {
    match read_and_parse_file("data.txt") {
        Ok(n) => println!("Parsed number: {}", n),
        Err(e) => println!("An error occurred: {}", e),
    }
    Ok(())
}
