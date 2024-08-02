fn main() {
    let user = ("John", true, 101u64);
    let (user_name, active, signin_count) = user;
    println!("The value of user_name is: {user_name}");
    println!("The value of active is: {active}");
    println!("The value of signin_count is: {signin_count}");
}