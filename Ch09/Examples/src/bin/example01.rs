fn fibonacci(n: usize) -> Vec<u64> {
    match n {
        0 => vec![],
        1 => vec![0],
        _ => {
            let mut fibs = vec![0, 1];
            for i in 2..n {
                fibs.push(fibs[i - 1] + fibs[i - 2]);
            }
            fibs
        }
    }
}

fn main() {
    for i in 0..=10 {
        println!("{:?}", fibonacci(i));
    }
}
