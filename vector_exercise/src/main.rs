#[derive(Debug)]
struct File {
    name: String,
}

#[derive(Debug)]
struct Folder {
    name: String,
    contents: Vec<File>,
}

impl Folder {
    fn new(name: String) -> Folder {
        Folder {
            name,
            contents: Vec::new(),
        }
    }

    fn create_file(&mut self, name: String) {
        self.name = name;
        self.contents.push(File {
            name: self.name.clone(),
        });
    }

    fn delete_file(&mut self, index: usize) -> File {
        self.contents.remove(index)
    }

    fn get_file_(&self, index: usize) -> Option<&File> {
        self.contents.get(index)
    }
}

fn main() {
    let mut folder = Folder::new("test".to_string());
    folder.create_file("test.txt".to_string());
    folder.create_file("test2.txt".to_string());
    println!("{:?}", folder);

    let deleted_file = folder.delete_file(0);
    println!("{:?}", deleted_file);
    println!("{:?}", folder);

    let get_file = folder.get_file_(0);
    match get_file {
        Some(file) => println!("{:?}", file),
        None => println!("{:?}", folder),
    }
}
