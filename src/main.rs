mod file;
mod folder;
mod traits;

use file::File;
use folder::Folder;
use traits::FileSystemItem;

fn main() {
    let mut root = Folder::new("root");

    let file1 = Box::new(File::new("file1.txt", "Hello, world!"));
    let file2 = Box::new(File::new("file2.txt", "Rust is awesome!"));

    let mut subfolder = Folder::new("subfolder");
    let file3 = Box::new(File::new("file3.txt", "Inside subfolder"));

    subfolder.add_item(file3);
    root.add_item(file1);
    root.add_item(file2);
    root.add_item(Box::new(subfolder));

    root.display(0);
}
