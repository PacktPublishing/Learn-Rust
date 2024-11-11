use std::path::{Path, PathBuf};

fn main() {
    let path = Path::new("/home/user");
    let file_name = "document.txt";

    let full_path: PathBuf = path.join(file_name);
    println!("Full path: {:?}", full_path);

    if let Some(parent) = full_path.parent() {
        println!("Parent directory: {}", parent);
    }
}
