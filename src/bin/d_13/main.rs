#[derive(Debug, Clone)]
struct Pair {
    left: Vec<char>,
    right: Vec<char>,
}

const TEN: char = 'x';
const DIVIDER_ONE: &str = "[[2]]";
const DIVIDER_TWO: &str = "[[6]]";

fn get_input() -> Vec<Pair> {
    include_str!("input.txt")
        .replace("10", String::from(TEN).as_str())
        .split("\r\n\r\n")
        .map(|x| {
            let (left, right) = x.split_once("\r\n").unwrap();
            Pair {
                left: left.chars().rev().collect::<Vec<char>>(),
                right: right.chars().rev().collect::<Vec<char>>(),
            }
        })
        .collect::<Vec<Pair>>()
}

fn is_a_number(c: char) -> bool {
    c.is_numeric() || c == TEN
}

fn is_packet_order_correct(mut pair: Pair) -> bool {
    loop {
        let left = pair.left.pop().unwrap();
        let right = pair.right.pop().unwrap();

        let ends = (left == ']', right == ']');
        match ends {
            (true, false) => {
                // Left list ran out first
                return true;
            }
            (false, true) => {
                // Right list ran out first
                return false;
            }
            _ => (),
        }

        if "[],".contains(left) && "[],".contains(right) {
            // Skip checks since we're correctly moving in or into a list
            continue;
        }

        if is_a_number(left) && is_a_number(right) {
            // Compare the integers by casting to their unicode value
            if (left as u16) > (right as u16) {
                return false;
            }
            if (left as u16) == (right as u16) {
                continue;
            }
            if (left as u16) < (right as u16) {
                return true;
            }
        }

        // Check if exactly one value is an integer
        let which_side = (is_a_number(left), is_a_number(right));
        if (which_side.0 || which_side.1) && ("[]".contains(left) || "[]".contains(right)) {
            // To make integer into a list, push closing bracket and the integer to stack
            match which_side {
                (true, false) => {
                    pair.left.push(']');
                    pair.left.push(left);
                }
                (false, true) => {
                    pair.right.push(']');
                    pair.right.push(right);
                }
                _ => (),
            }
        }
    }
}

fn part_one() -> u32 {
    let mut indices = vec![];
    let mut index = 1;
    for packet in get_input() {
        if is_packet_order_correct(packet) {
            indices.push(index);
        }
        index += 1;
    }
    indices.iter().sum()
}

fn part_two() -> usize {
    // Flatten the pair list
    let mut packets: Vec<Vec<char>> = get_input()
        .iter()
        .map(|pair| [pair.left.clone(), pair.right.clone()])
        .flatten()
        .collect();

    // Add divider packets
    packets.push(DIVIDER_ONE.chars().rev().collect());
    packets.push(DIVIDER_TWO.chars().rev().collect());

    // Sort into correct order
    packets.sort_by(|a, b| {
        is_packet_order_correct(Pair {
            left: b.clone(),
            right: a.clone(),
        })
        .cmp(&is_packet_order_correct(Pair {
            left: a.clone(),
            right: b.clone(),
        }))
    });

    // Concat packet stacks into strings
    let strings = packets
        .iter()
        .map(|x| String::from_iter(x.iter().rev()))
        .collect::<Vec<String>>();

    // Find positions of divider packets, indices start at 1, blegh...
    let d1 = strings.iter().position(|x| x == DIVIDER_ONE).unwrap() + 1;
    let d2 = strings.iter().position(|x| x == DIVIDER_TWO).unwrap() + 1;

    d1 * d2
}

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}
