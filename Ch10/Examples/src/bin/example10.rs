use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = File::create("example_test1.txt")?;
    let text = "Writing to a file";

    file.write_fmt(format_args!("{}", text))?;
    Ok(())
}
