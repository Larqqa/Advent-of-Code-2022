fn get_input() -> Vec<i32> {
    return include_str!("test.txt")
        .lines()
        .map(|s| s.trim().parse().unwrap())
        .collect();
}

fn part_one() -> i32 {
    let input = get_input();
    println!("{:?}", input);
    0
}

// fn part_two() -> i32 {
//     0
// }

fn main() {
    println!("part one: {}", part_one());
    // println!("part two: {}", part_two());
}