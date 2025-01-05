fn process_sensor_readings(readings: Vec<f64>) -> Option<f64> {
    let mut valid_readings: Vec<i32> = readings
        .into_iter()
        .filter(|&x| x >= 0.0 && x <= 100.0)
        .map(|x| x.round() as i32)
        .collect();

    valid_readings.sort();
    valid_readings.dedup();

    let sum: i32 = valid_readings.iter().sum();
    let count = valid_readings.len();

    if count > 0 {
        Some(sum as f64 / count as f64)
    } else {
        None
    }
}

fn main() {
    let sensor_readings = vec![23.4, 45.6, 101.0, -5.0, 76.2, 45.6, 98.9, 76.2];
    match process_sensor_readings(sensor_readings) {
        Some(avg) => println!("Average of valid readings: {:.2}", avg),
        None => println!("No valid readings to process."),
    }
}
