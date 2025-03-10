use crate::traits::FileSystemItem;

#[derive(Debug, Clone, PartialEq)]
pub struct File {
    name: String,
    size: usize,
    content: String,
}

impl File {
    pub fn new(name: &str, content: &str) -> Self {
        Self {
            name: name.to_string(),
            size: content.len(),
            content: content.to_string(),
        }
    }
}

impl FileSystemItem for File {
    fn get_size(&self) -> usize {
        self.size
    }

    fn display(&self, indent: usize) {
        println!(
            "{}📄 {} ({} bytes)",
            "  ".repeat(indent),
            self.name,
            self.size
        );
    }
}
