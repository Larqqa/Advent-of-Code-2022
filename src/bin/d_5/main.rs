#[derive(Debug, Clone, Copy)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}
impl Instruction {
    fn new(amount: usize, from: usize, to: usize) -> Instruction {
        Instruction { amount, from, to }
    }
}

fn get_input() -> (Vec<Vec<char>>, Vec<Instruction>) {
    let (crates, instructions) = include_str!("input.txt").split_once("\r\n\r\n").unwrap();
    let mut parsed_rows: Vec<Vec<Vec<char>>> = crates
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .map(|y| y.chunks(4).map(|x| x.to_vec()).collect())
        .collect();
    parsed_rows.pop(); // remove index line from crates

    let mut crates_array: Vec<Vec<char>> = vec![vec![]];
    let mut x = 0;
    for row in parsed_rows {
        for c in row {
            if crates_array.len() <= x {
                crates_array.push(vec![]);
            }

            // index 1 is the character of the crate, or empty
            if c[1] != ' ' {
                crates_array[x].insert(0, c[1]);
            }
            x += 1;
        }
        x = 0;
    }

    let instructions_array: Vec<Instruction> = instructions
        .lines()
        .map(|x| {
            let (ins, to) = x.split_once(" to ").unwrap();
            let (ins2, from) = ins.split_once(" from ").unwrap();
            let (_, amount) = ins2.split_once("move ").unwrap();

            Instruction::new(
                amount.parse().unwrap(),
                from.parse::<usize>().unwrap() - 1,
                to.parse::<usize>().unwrap() - 1,
            )
        })
        .collect();

    (crates_array, instructions_array)
}

fn part_one() -> String {
    let (mut crates, instructions) = get_input();
    for ins in instructions {
        for _ in 0..ins.amount {
            let temp = crates[ins.from].pop().unwrap();
            crates[ins.to].push(temp);
        }
    }
    String::from_iter(crates.iter().map(|c| c.last().unwrap()))
}

fn part_two() -> String {
    let (mut crates, instructions) = get_input();
    for ins in instructions {
        let mut temp = vec![];
        for _ in 0..ins.amount {
            temp.push(crates[ins.from].pop().unwrap());
        }
        temp.reverse();
        crates[ins.to].append(&mut temp);
    }
    String::from_iter(crates.iter().map(|c| c.last().unwrap()))
}

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}
