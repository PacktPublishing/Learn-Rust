fn main() {
    let first_ok: Result<i32, &str> = Ok(10);
    let second_ok: Result<i32, &str> = Ok(20);

    let first_err: Result<i32, &str> = Err("first error");
    let second_err: Result<i32, &str> = Err("second error");

    println!("ok - ok: {:?}", first_ok.or(second_ok));
    println!("ok - err: {:?}", first_ok.or(second_err));
    println!("err - ok: {:?}", first_err.or(second_ok));
    println!("err - err: {:?}", first_err.or(second_err));
}