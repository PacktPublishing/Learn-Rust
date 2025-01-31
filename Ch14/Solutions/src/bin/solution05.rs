// Struct holding two independent string slice references with separate lifetimes
struct DualStrings<'a, 'b> {
    first: &'a str,
    second: &'b str,
}

impl<'a, 'b> DualStrings<'a, 'b> {
    // Method to get the first string slice
    fn get_first(&self) -> &'a str {
        self.first
    }

    // Method to get the second string slice
    fn get_second(&self) -> &'b str {
        self.second
    }
}

fn main() {
    let string1 = String::from("Rust is great!");
    let first;
    let second;
    {
        let string2 = String::from("Memory safety matters.");
        let dual = DualStrings {
            first: &string1,  // 'a lifetime
            second: &string2, // 'b lifetime
        };

        // Store reference to first string (valid after string2 is dropped)
        first = dual.get_first();
        second = dual.get_second();

        // Retrieve references
        println!("First: {}", first);
        println!("Second: {}", second);
    } // string2 goes out of scope here

    // first is still valid because it refers to `string1`, which is still in scope
    println!("Result after nested scope: {}", first);
}
