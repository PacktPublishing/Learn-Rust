use std::fs;

fn main() -> std::io::Result<()> {
    // Specify the root directory
    let root_dir = "/";

    // Read the directory entries
    let entries = fs::read_dir(root_dir)?;

    // Iterate over the entries and print each one
    println!("Contents of root directory ({root_dir}):");
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        println!("{}", path.display());
    }

    Ok(())
}
