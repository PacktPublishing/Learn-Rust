fn parse_csv_rows(rows: Vec<String>) -> Vec<Vec<i32>> {
    rows.into_iter()
        .filter_map(|row| {
            let parsed_row: Vec<_> = row
                .split(',')
                .map(|s| s.trim().parse::<i32>())
                .collect();

            if parsed_row.iter().all(|x| x.is_ok()) {
                Some(parsed_row.into_iter().map(Result::unwrap).collect())
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    let csv_rows = vec![
        "1, 2, 3".to_string(),
        "4, 5, a".to_string(),
        "6, 7, 8".to_string(),
        "9, x, 11".to_string(),
    ];

    let result = parse_csv_rows(csv_rows);
    println!("Parsed CSV rows: {:?}", result);
}
