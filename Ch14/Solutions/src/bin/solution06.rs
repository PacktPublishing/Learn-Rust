struct Holder<'a> {
    data: &'a str,
}

impl<'a> Holder<'a> {
    fn get_ref(&'a self, input: &'a str) -> &'a str {
        input
    }
}

fn main() {
    let holder;
    let result;
    {
        let data = String::from("Hello, Rust!");
        holder = Holder { data: &data };

        let input = String::from("Scoped Input");
        result = holder.get_ref(&input);
        println!("{}", result);
    }

    // Uncommenting this would cause a borrow error as `input` no longer exists
    // println!("{}", result);
}
