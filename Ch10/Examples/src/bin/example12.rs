use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> std::io::Result<()> {
    let file = File::create("example_test2.txt")?;
    let mut writer = BufWriter::new(file);

    writer.write_all(b"This is a buffered write.\n")?;
    writer.flush()?;

    Ok(())
}
