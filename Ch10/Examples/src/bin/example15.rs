use std::fs::metadata;
use std::io;

fn main() -> io::Result<()> {
    let file_metadata = metadata("example.txt")?;

    if file_metadata.permissions().readonly() {
        println!("The file is read-only.");
    } else {
        println!("The file is writable.");
    }

    let file_size = file_metadata.len();
    println!("The file size is {} bytes.", file_size);

    Ok(())
}
