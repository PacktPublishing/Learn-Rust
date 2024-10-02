fn main() {
    let success_result: Result<i32, &str> = Ok(10);
    let failure_result: Result<i32, &str> = Err("Error!");

    let success_option: Option<i32> = success_result.ok();
    let failure_option: Option<i32> = failure_result.ok();

    println!("Success as Option: {:?}", success_option);
    println!("Failure as Option: {:?}", failure_option);
}
