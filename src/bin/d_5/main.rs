const EMPTY_CRATE: char = '#';

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
    let (crates, instructions) = include_str!("test.txt").split_once("\n\n").unwrap();

    let mut parsed_strings: Vec<Vec<Vec<char>>> = crates
        .lines()
        .map(|x| x.chars()
            .collect::<Vec<char>>()
            .chunks(x.to_string().len() / 4)
            // .map(|y| if y[1] == ' ' {EMPTY_CRATE} else {y[1]})
            .collect::<Vec<Vec<char>>>())
        .collect();
    parsed_strings.pop(); // Remove index line of crates

    println!("{:?}", parsed_strings);

    let mut crates_array: Vec<Vec<char>> = vec![vec![]];
    let mut x = 0;
    for row in parsed_strings {
        for c in row {
            if crates_array.len() <= x {
                crates_array.push(vec![]);
            }

            if c != EMPTY_CRATE {
                crates_array[x].insert(0, c);
            }
            x += 1;
        }
        x = 0;
    }
    // println!("{:?}", crates_array);

    let i: Vec<Instruction> = instructions
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
    // println!("{:?}", i);

    (crates_array, i)
}

fn part_one() -> String {
    let (mut crates, instructions) = get_input();
    println!("{:?}", crates);

    // println!("{:?}", crates);
    // for ins in instructions {
    //     for _ in 0..ins.amount {
    //         let temp = crates[ins.from].pop().unwrap();
    //         crates[ins.to].push(temp);
    //     }
    // }
    // println!("{:?}", crates);

    // String::from_iter(crates.iter().map(|c| c.last().unwrap()))
    String::new()
}

// fn part_two() -> i32 {
//     0
// }

fn main() {
    println!("part one: {}", part_one());
    // println!("part two: {}", part_two());
}
