use std::path::PathBuf;

fn main() {
    // Define the base path and file name
    let base_path = PathBuf::from("/home/user/documents");
    let file_name = "report.txt";

    // Append the file name to the base path
    let full_path = base_path.join(file_name);

    // Print the full path
    println!("Full path: {}", full_path.display());

    // Print the parent directory
    match full_path.parent() {
        Some(parent) => println!("Parent directory: {}", parent.display()),
        None => println!("The full path has no parent directory."),
    }
}
