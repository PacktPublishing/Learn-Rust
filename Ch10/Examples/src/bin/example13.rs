use std::path::{Path, PathBuf};

fn main() {
    let path = Path::new("Ch10/Examples");
    let file_name = "example.txt";

    let full_path: PathBuf = path.join(file_name);
    println!("Full path: {:?}", full_path);

    if let Some(parent) = full_path.parent() {
        println!("Parent directory: {:?}", parent);
    }
}
