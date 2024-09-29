fn get_username(user_id: i32) -> Option<String> {
    if user_id == 1 {
        Some(String::from("Alice"))
    } else {
        None
    }
}

fn main() {
    let username = get_username(2);

    match username {
        Some(name) => println!("Username: {}", name),
        None => println!("User not found"),
    }
}
