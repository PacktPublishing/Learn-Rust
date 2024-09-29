fn main() {
    let config_file = std::fs::File::open("config.txt")
        .expect("Failed to open the configuration file");
    println!("Config file opened: {:?}", config_file);
}
