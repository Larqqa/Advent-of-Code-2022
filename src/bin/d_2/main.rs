const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;
const LOST: u32 = 0;
const DRAW: u32 = 3;
const WON: u32 = 6;

fn get_input() -> Vec<(char, char)> {
    include_str!("input.txt")
        .lines()
        .map(|x| {
            let (targ, resp) = x.split_once(" ").unwrap();
            let a = targ.chars().collect::<Vec<char>>();
            let b = resp.chars().collect::<Vec<char>>();
            (*a.first().unwrap(), *b.first().unwrap())
        })
        .collect()
}

fn map_to_score(a: char) -> u32 {
    match a {
        'A' => ROCK,
        'X' => ROCK,
        'B' => PAPER,
        'Y' => PAPER,
        'C' => SCISSORS,
        'Z' => SCISSORS,
        _ => todo!(),
    }
}

fn part_one() -> u32 {
    let i = get_input();
    let input: Vec<(u32, u32)> = i
        .iter()
        .map(|(a, b)| (map_to_score(*a), map_to_score(*b)))
        .collect();

    let mut score = 0;
    for (op, me) in input {
        score += me;
        score += if op == me {
            DRAW
        } else {
            match (op, me) {
                (SCISSORS, ROCK) => WON,
                (ROCK, PAPER) => WON,
                (PAPER, SCISSORS) => WON,
                _ => LOST,
            }
        };
    }

    score
}

fn fetch_me(op: u32, game: char) -> Option<u32> {
    match game {
        'Y' => Some(op),
        'X' => match op {
            ROCK => Some(SCISSORS),
            PAPER => Some(ROCK),
            SCISSORS => Some(PAPER),
            _ => None,
        },
        _ => match op {
            ROCK => Some(PAPER),
            PAPER => Some(SCISSORS),
            SCISSORS => Some(ROCK),
            _ => None,
        },
    }
}

fn part_two() -> u32 {
    let i = get_input();
    let input: Vec<(u32, char)> = i.iter().map(|(a, b)| (map_to_score(*a), *b)).collect();
    let mut score = 0;
    for (op, game) in input {
        score += fetch_me(op, game).unwrap();
        score += match game {
            'X' => LOST,
            'Y' => DRAW,
            'Z' => WON,
            _ => 0,
        };
    }
    score
}

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}
