use std::arch::asm;

fn main() {
    unsafe {
        asm!("nop"); // Perform a no-operation instruction
    }
}
