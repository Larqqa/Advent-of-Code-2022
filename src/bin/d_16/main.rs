use std::{cell::RefCell, collections::HashMap, fmt::Formatter, rc::Rc};

#[derive(Clone)]
struct Node {
    neighbors: HashMap<String, Rc<RefCell<Node>>>,
    value: u32,
}

impl Node {
    fn new(value: u32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            neighbors: HashMap::new(),
            value,
        }))
    }
}

impl core::fmt::Debug for Node {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        write!(
            f,
            "value: {}, neighbors: {:?}",
            self.value,
            self.neighbors.keys()
        )
    }
}

fn get_input() -> HashMap<String, Rc<RefCell<Node>>> {
    let mut map: HashMap<String, Rc<RefCell<Node>>> = HashMap::new();
    let s = include_str!("test.txt").lines().collect::<Vec<_>>();
    for y in s {
        let x = y
            .replace("Valve ", "")
            .replace(" has flow rate", "")
            .replace(" tunnels lead to valves ", "")
            .replace(" tunnel leads to valve ", "");

        let (v, c) = x.split_once(";").unwrap();
        let (name, flow) = v.split_once("=").unwrap();
        let childs = c.split(", ").collect::<Vec<_>>();

        let cave = Rc::clone(map.entry(name.to_string()).or_insert(Node::new(0)));
        cave.borrow_mut().value = flow.parse().unwrap();

        childs.iter().for_each(|child| {
            let c = if !map.contains_key(&child.to_string()) {
                let c = Node::new(0);
                let mut cm = c.borrow_mut();
                cm.neighbors.insert(name.to_string(), Rc::clone(&cave));
                map.insert(child.to_string(), Rc::clone(&c));
                Rc::clone(&c)
            } else {
                Rc::clone(map.get(&child.to_string()).unwrap())
            };

            cave.borrow_mut()
                .neighbors
                .insert(child.to_string(), Rc::clone(&c));
        });
    }
    map
}

fn get_node(key: String, map: &HashMap<String, Rc<RefCell<Node>>>) -> Rc<RefCell<Node>> {
    Rc::clone(map.get(&key).unwrap())
}

fn part_one() -> i32 {
    let input = get_input();
    println!("{:#?}", input);

    let a = get_node(String::from("AA"), &input);
    let b = get_node(String::from("BB"), &input);
    let c = get_node(String::from("CC"), &b.borrow().neighbors);

    println!("{:#?}, {:#?}, {:#?}", a, b, c);

    0
}

// fn part_two() -> i32 {
//     0
// }

fn main() {
    println!("part one: {}", part_one());
    // println!("part two: {}", part_two());
}
