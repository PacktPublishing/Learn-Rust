// When writing this in a functional style it can be made significantly shorter
// functional programming will be explained in chapter 11

fn main() {
    let input = "apple,banana,pear";
    let result = join_with_space(input);
    println!("{}", result);
}

fn join_with_space(input: &str) -> String {
    let mut result = String::new();
    let parts = input.split(',');
    for (i, part) in parts.enumerate() {
        if i > 0 {
            result.push(' ');
        }
        result.push_str(part);
    }

    result
}
