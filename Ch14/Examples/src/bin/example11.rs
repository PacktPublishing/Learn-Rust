struct Config<'a> {
    key: &'a str,
}

impl<'a> Config<'a> {
    fn get_key(&self) -> &'a str {
        self.key
    }
}

fn main() {
    let config = Config { key: "value" };
    println!("{}", config.get_key());
}