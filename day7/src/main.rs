use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone)]
struct File {
    size: u64,
}

#[derive(Clone)]
struct Directory {
    name: String,
    entries: HashMap<String, Entry>,
    parent: Option<Rc<RefCell<Directory>>>,
}

impl Directory {
    fn size(&self) -> u64 {
        let mut size = 0;

        for entry in self.entries.values() {
            match entry {
                Entry::File(file) => size += file.size,
                Entry::Directory(dir) => size += dir.borrow().size(),
            }
        }

        size
    }

    fn walk_directories(&self, callback: &mut dyn FnMut(&Directory)) {
        for entry in self.entries.values() {
            match entry {
                Entry::File(_) => {}
                Entry::Directory(dir) => {
                    callback(&dir.borrow());
                    dir.borrow().walk_directories(callback);
                }
            }
        }
    }
}

#[derive(Clone)]
enum Entry {
    File(File),
    Directory(Rc<RefCell<Directory>>),
}

fn main() {
    let file_contents =
        std::fs::read_to_string("./input.txt").expect("Something went wrong reading the file");

    let root_entry = Rc::new(RefCell::new(Directory {
        name: String::from("Tree root"),
        entries: HashMap::new(),
        parent: None,
    }));

    let mut current_entry = Rc::clone(&root_entry);

    for line in file_contents.lines() {
        match line.chars().nth(0) {
            Some('$') => {
                let parts: Vec<&str> = line.split_whitespace().collect();
                match parts[1] {
                    "cd" => {
                        if parts[2] == ".." {
                            let cc = current_entry.clone();

                            current_entry = cc.as_ref().borrow().parent.as_ref().unwrap().clone();
                        } else {
                            let c = current_entry.clone();

                            let mut b_entry = c.as_ref().borrow_mut();

                            let entry = b_entry.entries.get(parts[2]);
                            current_entry = match entry {
                                Some(Entry::Directory(dir)) => dir.clone(),
                                _ => {
                                    let new_directory = Rc::new(RefCell::new(Directory {
                                        name: String::from(parts[2]),
                                        entries: HashMap::new(),
                                        parent: Some(current_entry.clone()),
                                    }));

                                    b_entry.entries.insert(
                                        parts[2].to_string(),
                                        Entry::Directory(new_directory.clone()),
                                    );

                                    new_directory
                                }
                            }
                        }
                    }
                    "ls" => {}
                    _ => {}
                }
            }
            Some('1'..='9') => {
                let parts: Vec<&str> = line.split_whitespace().collect();
                current_entry.borrow_mut().entries.insert(
                    parts[1].to_string(),
                    Entry::File(File {
                        size: parts[0].parse().unwrap(),
                    }),
                );
            }
            _ => {}
        }
    }

    let mut part1 = 0;

    root_entry.borrow().walk_directories(&mut |dir| {
        let size = dir.size();

        if size < 100000 {
            part1 += size;
        }
    });

    println!("part1: {}", part1);

    let space_needed = 30000000;
    let space_available = 70000000;
    let space_used = root_entry.borrow().size();

    let min_space_to_free = space_needed - (space_available - space_used);

    let mut part2 = root_entry.borrow().size();

    root_entry.borrow().walk_directories(&mut |dir| {
        println!("{}: {}", dir.name, dir.size());

        let size = dir.size();

        if size >= min_space_to_free && size < part2 {
            part2 = size;
        }
    });

    println!("part2: {}", part2);
}
