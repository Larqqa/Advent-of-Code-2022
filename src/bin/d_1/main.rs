fn get_input() -> Vec<Vec<i64>> {
    return include_str!("input.txt")
        .split("\n\n")
        .map(|x| x.lines().map(|s| s.trim().parse().unwrap()).collect())
        .collect();
}

fn part_one() -> i64 {
    get_input().iter().map(|x| x.iter().sum()).max().unwrap()
}

fn part_two() -> i64 {
    let mut summed: Vec<i64> = get_input().iter().map(|x| x.iter().sum()).collect();
    summed.sort();
    summed[(summed.len() - 3)..].iter().map(|x| *x).sum()
}

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}
