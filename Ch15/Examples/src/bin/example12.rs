fn main() {
    let null_ptr: *const i32 = std::ptr::null();
    assert!(null_ptr.is_null());
}
