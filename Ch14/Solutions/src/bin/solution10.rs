struct NestedHolder<'a, 'b> {
    first: &'a str,
    second: &'b str,
}

impl<'a, 'b> NestedHolder<'a, 'b> {
    fn new(first: &'a str, second: &'b str) -> Self {
        Self { first, second }
    }

    fn longest(&self) -> &str {
        if self.first.len() > self.second.len() {
            self.first
        } else {
            self.second
        }
    }
}

fn main() {
    let string1 = String::from("Outer Scope longest String");
    let result;
    {
        let string2 = String::from("Inner Scope String");
        let holder = NestedHolder::new(&string1, &string2);
        result = holder.longest();
        println!("Longest string is: {}", result);
    }

    // Uncommenting the next line will cause a compile-time error since string2 is out of scope
    // it will return string1, but the lifetime of string one is the same as the lifetime of string2
    // println!("{}", result);
}