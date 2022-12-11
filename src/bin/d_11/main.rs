#[derive(Debug, Clone)]
struct Monkey {
    id: i128,
    items: Vec<i128>,
    operation: char,
    operation_modifier: String,
    test_value: i128,
    test_true_target: i128,
    test_false_target: i128,
    inspections: u32,
}

impl Monkey {
    fn operate(&mut self) {
        for i in 0..self.items.len() {
            let item = self.items[i];
            let value = if self.operation_modifier == "old" {
                item
            } else {
                self.operation_modifier.parse().unwrap()
            };
            let result = match self.operation {
                '+' => item + value,
                '*' => item * value,
                _ => todo!(),
            };
            self.items[i] = result / 3;
            self.inspections += 1;
        }
    }

    fn operate_two(&mut self, product: i128) {
        for i in 0..self.items.len() {
            let item = self.items[i];
            let value = if self.operation_modifier == "old" {
                item
            } else {
                self.operation_modifier.parse().unwrap()
            };
            let result = match self.operation {
                '+' => item + value,
                '*' => item * value,
                _ => todo!(),
            };
            self.items[i] = result % product;
            self.inspections += 1;
        }
    }

    fn throw(&mut self, mut monkeys: Vec<Monkey>) -> Vec<Monkey> {
        for i in (0..self.items.len()).rev() {
            let item = self.items[i];
            self.items.remove(i);
            if (item % self.test_value) == 0 {
                let target = monkeys.iter_mut().find(|x| x.id == self.test_true_target);
                target.unwrap().items.push(item);
            } else {
                let target = monkeys.iter_mut().find(|x| x.id == self.test_false_target);
                target.unwrap().items.push(item);
            }
        }

        monkeys
    }
}

fn get_input() -> Vec<Monkey> {
    let input = include_str!("input.txt");
    let m_crude = input.split("\r\n\r\n").collect::<Vec<&str>>();
    let ms = m_crude
        .iter()
        .map(|m| m.lines().collect())
        .collect::<Vec<Vec<&str>>>();
    let mut monkeys: Vec<Monkey> = vec![];
    for m in ms {
        let id: i128 = m[0]
            .replace("Monkey ", "")
            .trim()
            .replace(":", "")
            .parse()
            .unwrap();
        let items: Vec<i128> = m[1]
            .replace("  Starting items: ", "")
            .trim()
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();
        let ops = m[2].replace(" Operation: new = old ", "");
        let (op, op_mod) = ops.trim().split_once(" ").unwrap();
        let operation = op.chars().collect::<Vec<char>>()[0];
        let operation_modifier = op_mod.to_string();
        let test_value: i128 = m[3]
            .replace("  Test: divisible by ", "")
            .trim()
            .parse()
            .unwrap();
        let test_true_target: i128 = m[4]
            .replace("    If true: throw to monkey ", "")
            .trim()
            .parse()
            .unwrap();
        let test_false_target: i128 = m[5]
            .replace("    If false: throw to monkey ", "")
            .trim()
            .parse()
            .unwrap();
        monkeys.push(Monkey {
            id,
            items,
            operation,
            operation_modifier,
            test_value,
            test_true_target,
            test_false_target,
            inspections: 0,
        });
    }
    monkeys
}

fn part_one() -> u32 {
    let mut input = get_input();
    for _ in 0..20 {
        for i in 0..input.len() {
            let mut monkey = input[i].clone();
            monkey.operate();
            input = monkey.throw(input.clone());
            input[i] = monkey;
        }
    }
    input.sort_by(|a, b| a.inspections.cmp(&b.inspections));
    input[(input.len() - 2)..]
        .iter()
        .fold(1, |a, b| a * b.inspections)
}

fn part_two() -> u128 {
    let mut input = get_input();
    let product = input.iter().map(|x| x.test_value).fold(1, |a, b| a * b);
    for _ in 0..10000 {
        for i in 0..input.len() {
            let mut monkey = input[i].clone();
            monkey.operate_two(product);
            input = monkey.throw(input.clone());
            input[i] = monkey;
        }
    }
    input.sort_by(|a, b| a.inspections.cmp(&b.inspections));
    input[(input.len() - 2)..]
        .iter()
        .fold(1, |a, b| a * b.inspections as u128)
}

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}
