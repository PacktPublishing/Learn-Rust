struct Resource {
    name: String,
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("Cleaning up resource: {}", self.name);
    }
}

fn main() {
    let res1 = Resource { name: String::from("File1") };
    let res2 = Resource { name: String::from("Database Connection") };
    println!("Resources created");
}
