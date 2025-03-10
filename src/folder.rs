use crate::traits::FileSystemItem;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Folder {
    name: String,
    items: Vec<Box<dyn FileSystemItem>>,
}

impl Folder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: Box<dyn FileSystemItem>) {
        self.items.push(item);
    }
}

impl FileSystemItem for Folder {
    fn get_size(&self) -> usize {
        self.items.iter().map(|item| item.get_size()).sum()
    }

    fn display(&self, indent: usize) {
        println!("{}📂 {}", "  ".repeat(indent), self.name);
        for item in &self.items {
            item.display(indent + 1);
        }
    }
}
