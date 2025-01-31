struct StringSlicer<'a> {
    source: &'a str,
}

impl<'a> StringSlicer<'a> {
    fn new(source: &'a str) -> Self {
        Self { source }
    }

    fn slice_by_delimiter(&self, delimiter: char) -> Vec<&'a str> {
        self.source.split(delimiter).collect()
    }

    fn slice_by_indices(&self, start: usize, end: usize) -> &'a str {
        &self.source[start..end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice_by_delimiter() {
        let text = "hello,world,test";
        let slicer = StringSlicer::new(text);
        let slices: Vec<&str> = slicer.slice_by_delimiter(',');
        assert_eq!(slices, vec!["hello", "world", "test"]);
    }

    #[test]
    fn test_slice_by_indices() {
        let text = "RustProgramming";
        let slicer = StringSlicer::new(text);
        let slice = slicer.slice_by_indices(0, 4);
        assert_eq!(slice, "Rust");
    }
}

fn main() {
    let text = "This is a test string";
    let slicer = StringSlicer::new(text);
    let words = slicer.slice_by_delimiter(' ');
    println!("Words: {:?}", words);
    let substring = slicer.slice_by_indices(0, 4);
    println!("Substring: {}", substring);
}
