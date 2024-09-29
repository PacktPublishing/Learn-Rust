use regex::Regex;

fn main() {
    let re = Regex::new(r"\d+").unwrap();
    let text = "The house number is 118.";
    println!("Found match: {:?}", re.find(text));
}
