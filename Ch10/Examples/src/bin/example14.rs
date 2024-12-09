use std::fs;
use std::io;
use std::env;

fn main() -> io::Result<()> {
    let working_dir = env::current_dir()?;
    println!("Current working directory: {:?}", working_dir);

    for entry in fs::read_dir(working_dir)? {
        let entry = entry?;
        let file_name = entry.file_name();
        println!("File or dir: {:?}", file_name);
    }

    Ok(())
}