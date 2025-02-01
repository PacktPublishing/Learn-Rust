struct SafeBuffer {
    ptr: *mut u8,
    size: usize,
}

impl SafeBuffer {
    fn new(size: usize) -> Self {
        let ptr = unsafe { std::alloc::alloc(std::alloc::Layout::array::<u8>(size).unwrap()) };
        if ptr.is_null() {
            panic!("Memory allocation failed");
        }
        Self { ptr, size }
    }

    fn get(&self, index: usize) -> u8 {
        assert!(index < self.size, "Index out of bounds");
        unsafe { *self.ptr.add(index) }
    }

    fn set(&mut self, index: usize, value: u8) {
        assert!(index < self.size, "Index out of bounds");
        unsafe { *self.ptr.add(index) = value; }
    }
}

impl Drop for SafeBuffer {
    fn drop(&mut self) {
        unsafe { std::alloc::dealloc(self.ptr, std::alloc::Layout::array::<u8>(self.size).unwrap()) };
    }
}

fn main() {
    let mut buffer = SafeBuffer::new(10);
    buffer.set(0, 42);
    let value = buffer.get(0);
    println!("Value at index 0: {}", value);
}