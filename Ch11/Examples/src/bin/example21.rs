fn parse_numbers(input: Vec<&str>) -> Result<Vec<i32>, String> {
    input
        .into_iter()
        .map(|s| {
            s.parse::<i32>()
                .map_err(|_| format!("Failed to parse '{}'", s))
        })
        .collect()
}

fn main() {
    let data = vec!["42", "93", "invalid", "17"];
/*    match parse_numbers(data) {
        Ok(numbers) => println!("Parsed numbers: {:?}", numbers),
        Err(err) => println!("Error: {}", err),
    }*/

    let x:Result<Vec<i32>, String> = data
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("{:?}", x);
}
