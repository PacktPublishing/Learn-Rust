trait Printable {
    fn print(&self);
}

struct Document {
    content: String,
}

impl Printable for Document {
    fn print(&self) {
        println!("Document content: {}", self.content);
    }
}

struct Image {
    data: Vec<u8>,
}

impl Printable for Image {
    fn print(&self) {
        println!("Image with {} bytes", self.data.len());
    }
}

fn main() {}