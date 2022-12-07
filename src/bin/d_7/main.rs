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

fn get_input() {
    let mut lines: Vec<&str> = include_str!("test.txt").lines().collect();
    lines.reverse();
    lines.pop();

    let root = Rc::new(RefCell::new(Directory::new(String::from("/"))));
    let mut current = Rc::clone(&root);
    while lines.len() > 0 {
        let instruction = lines.pop().unwrap().to_string();
        match &instruction[0..4] {
            "$ cd" => {
                let key = instruction.replace("$ cd ", "");
                // current_dir = *current_dir.children.get(&key).unwrap();
            }
            "$ ls" => {
                ls(&mut lines, current);
            }
            _ => (),
        }
    }
    println!("{:#?}", root);
}

fn ls(lines: &mut Vec<&str>, current_dir: Rc<RefCell<Directory>>) -> Rc<RefCell<Directory>>{
    let mut directory_contents: Vec<&str> = vec![];
    while lines.len() > 0 && !lines.last().unwrap().contains("$") {
        directory_contents.push(lines.pop().unwrap());
    }

    let (dirs, files): (Vec<&str>, Vec<&str>) =
        directory_contents.iter().partition(|x| x.contains("dir"));

    let child_keys = dirs
        .clone()
        .iter()
        .map(|x| x.replace("dir ", ""))
        .collect::<Vec<String>>();

    for key in child_keys {
        let mut d = Directory::new(key.clone());
        d.parent = Some(Rc::clone(&current_dir));
        current_dir
            .borrow_mut()
            .children
            .insert(key, Rc::new(RefCell::new(d)));
    }

    current_dir.borrow_mut().files.append(
        &mut files
            .iter()
            .map(|x| {
                let (size, name) = x.split_once(" ").unwrap();
                File {
                    size: size.parse().unwrap(),
                    name: name.to_string(),
                }
            })
            .collect(),
    );

    current_dir
}

// fn cd(file_system: &HashMap<String, Directory>, target: &String, current: Directory) -> Directory {
//     let dir: Option<&Directory>;
//     if target == ".." {
//         dir = file_system.get(&current.parent.unwrap());
//     } else {
//         dir = file_system.get(target);
//     }

//     if dir.is_none() {
//         panic!("aaaaaa")
//     } else {
//         dir.unwrap().clone()
//     }
// }

fn part_one() -> i128 {
    let mut input = get_input();
    println!("{:#?}", input);
    0
}

// fn part_two() -> i32 {
//     0
// }

fn main() {
    println!("part one: {}", part_one());
    // println!("part two: {}", part_two());
}
