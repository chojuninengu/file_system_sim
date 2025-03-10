# 📂 Simple Rust File System Explained

## 🚀 What Are We Building?
We are making a **mini file system** in Rust! This means we can create **folders** (📂) and **files** (📄), put files inside folders, and even add folders inside other folders.

Think of it like how you organize your computer:
- A **Folder** (📂) can hold **Files** (📄) and other **Folders** (📂).
- Each **File** has a **name**, **size**, and **content**.
- Each **Folder** knows how big it is by adding up the sizes of everything inside it.

---

## 🏗️ How It Works
### 1️⃣ The **File** Structure (📄)
A **file** has a **name**, some **text inside it**, and a **size** (counting how many letters are inside).

```rust
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
```

📌 **Real-life Example:**
- A **File** is like a **notebook** 📖. The notebook has a **title** (name), some **writing inside** (content), and the more you write, the **heavier** it gets (size).

---

### 2️⃣ The **Folder** Structure (📂)
A **folder** holds **files** and **other folders**.

```rust
#[derive(Debug)]
pub struct Folder {
    name: String,
    items: Vec<Box<dyn FileSystemItem>>, // Can hold both files and folders
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
```

📌 **Real-life Example:**
- A **Folder** is like a **backpack** 🎒. You can put **books (files)** inside, and even **smaller bags (subfolders)** with more books!

---

### 3️⃣ The **Trait** (Common Rules)
Since both **Files** and **Folders** have a **size** and a way to **display** themselves, we create a **FileSystemItem** trait to make sure they follow the same rules.

```rust
pub trait FileSystemItem: Debug {
    fn get_size(&self) -> usize;
    fn display(&self, indent: usize);
}
```

---

### 4️⃣ Making It Work (👨‍💻 Main Function)
Now, we **put everything together** in `main.rs`.

```rust
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
```

📌 **What Happens When We Run It?**
```sh
📂 root
  📄 file1.txt (13 bytes)
  📄 file2.txt (16 bytes)
  📂 subfolder
    📄 file3.txt (17 bytes)
```

🎉 We have a **working file system**! 🚀

