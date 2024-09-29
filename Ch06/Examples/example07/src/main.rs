fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn addition_works() {
        assert_eq!(2 + 2, 5);
    }
}
