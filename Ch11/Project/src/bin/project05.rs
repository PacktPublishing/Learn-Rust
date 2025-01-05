fn process_products(products: Vec<(&str, f64)>, price_threshold: f64) -> Vec<String> {
    products
        .into_iter()
        .filter(|&(_, price)| price >= price_threshold)
        .map(|(name, price)| (name.to_string(), price.round() as i32))
        .collect::<Vec<_>>()
        .into_iter()
        .map(|(name, _)| name)
        .collect::<Vec<_>>()
}

fn main() {
    let products = vec![
        ("Apple", 1.49),
        ("Banana", 0.99),
        ("Orange", 2.15),
        ("Mango", 5.75),
        ("Grape", 2.05),
    ];

    let result = process_products(products, 2.0);
    println!("Products with prices >= 2.0: {:?}", result);
}
