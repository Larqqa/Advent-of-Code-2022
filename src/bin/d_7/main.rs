use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone)]
struct Directory {
    parent: Option<Rc<RefCell<Directory>>>,
    name: String,
    children: HashMap<String, Rc<RefCell<Directory>>>,
    files: Vec<File>,
}
impl Directory {
    fn new(name: String) -> Directory {
        Directory {
            parent: None,
            name,
            children: HashMap::new(),
            files: vec![],
        }
    }

    fn size(&self) -> i128 {
        self.files.iter().map(|x| x.size).sum()
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct File {
    size: i128,
    name: String,
}

fn get_input() -> Directory {
    let mut lines: Vec<&str> = include_str!("input.txt").lines().collect();
    lines.reverse();
    lines.pop();

    let root = Rc::new(RefCell::new(Directory::new(String::from("/"))));
    let mut current = Rc::clone(&root);

    while lines.len() > 0 {
        let instruction = lines.pop().unwrap().to_string();
        match &instruction[0..4] {
            "$ cd" => {
                let cm = current.borrow_mut().clone();
                let key = instruction.replace("$ cd ", "");

                if key == ".." {
                    current = Rc::clone(&cm.parent.unwrap());
                } else {
                    current = Rc::clone(cm.children.get(&key).unwrap());
                }
            }
            "$ ls" => {
                let mut directory_contents: Vec<&str> = vec![];
                while lines.len() > 0 && !lines.last().unwrap().contains("$") {
                    directory_contents.push(lines.pop().unwrap());
                }

                let (dir_names, files): (Vec<&str>, Vec<&str>) =
                    directory_contents.iter().partition(|x| x.contains("dir"));

                let child_dirs = dir_names
                    .iter()
                    .map(|x| x.replace("dir ", ""))
                    .map(|key| {
                        let mut dir = Directory::new(key);
                        dir.parent = Some(Rc::clone(&current));
                        dir
                    })
                    .collect::<Vec<Directory>>();

                let mut cm = current.borrow_mut();
                for child in child_dirs {
                    cm.children
                        .insert(child.name.clone(), Rc::new(RefCell::new(child)));
                }

                cm.files = files
                    .iter()
                    .map(|x| {
                        let (size, name) = x.split_once(" ").unwrap();
                        File {
                            size: size.parse().unwrap(),
                            name: name.to_string(),
                        }
                    })
                    .collect::<Vec<File>>();
            }
            _ => (),
        }
    }
    root.clone().borrow_mut().to_owned()
}

fn find_dir_sizes(get_all: bool, root: Directory, found: &mut Vec<i128>) -> (&mut Vec<i128>, i128) {
    let mut total = root.size();
    for (_, child) in root.children {
        let (_, total_from_children) = find_dir_sizes(get_all, child.borrow_mut().clone(), found);
        total += total_from_children;
    }

    if get_all || total <= 100000 {
        found.push(total);
    }

    (found, total)
}

fn part_one() -> i128 {
    let mut found = vec![];
    let (smalls, _) = find_dir_sizes(false, get_input(), &mut found);
    smalls.iter().sum()
}

fn part_two() -> i128 {
    let mut found = vec![];
    let (all, total_dir_size) = find_dir_sizes(true, get_input(), &mut found);
    let total_space = 70000000;
    let required_space = 30000000;
    let free_space = total_space - total_dir_size;

    let potential_removable = all
        .iter()
        .filter(|x| free_space + **x > required_space)
        .collect::<Vec<&i128>>();
    **potential_removable.iter().min().unwrap()
}

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}
