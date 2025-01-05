struct NumberRange {
    end: i32,
    current: i32,
}

impl NumberRange {
    fn new(start: i32, end: i32) -> Self {
        NumberRange { end, current: start }
    }
}

impl Iterator for NumberRange {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let value = self.current;
            self.current += 1;
            Some(value)
        } else {
            None
        }
    }
}

fn main() {
    let range = NumberRange::new(3, 6);

    for num in range {
        println!("{}", num);
    }
}
