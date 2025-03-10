use std::fmt::Debug;

pub trait FileSystemItem: Debug {
    fn get_size(&self) -> usize;
    fn display(&self, indent: usize);
}
