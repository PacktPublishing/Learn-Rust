struct Fibonacci {
    a: usize,
    b: usize,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next_value = self.a;
        self.a = self.b;
        self.b += next_value;
        Some(next_value)
    }
}

fn main() {
    let fib = Fibonacci::new();
    let first_ten: Vec<_> = fib.take(10).collect();
    println!("First 10 Fibonacci numbers: {:?}", first_ten);
}