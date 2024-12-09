use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = File::open("example.txt")?;
    let mut buffer = [0; 16];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        let text = String::from_utf8_lossy(&buffer[..bytes_read]);
        print!("{}", text);
    }

    Ok(())
}
