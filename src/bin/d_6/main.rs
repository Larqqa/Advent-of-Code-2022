use std::collections::HashSet;

fn get_input() -> String {
    return include_str!("input.txt").to_string();
}

fn find_unique_substring(input: &String, length: usize) -> usize {
    let mut i = 0;
    let mut uniq = HashSet::new();
    loop {
        if input[i..(i + length)].chars().all(|x| uniq.insert(x)) {
            break;
        }
        uniq.clear();
        i += 1;
    }
    i + length
}

fn main() {
    let input = get_input();
    println!("part one: {}", find_unique_substring(&input, 4));
    println!("part two: {}", find_unique_substring(&input, 14));
}
