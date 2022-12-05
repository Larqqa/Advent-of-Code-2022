fn get_input() -> Vec<Vec<(u32, u32)>> {
    return include_str!("input.txt")
        .lines()
        .map(|x| {
            x.split(",")
                .map(|y| {
                    let mut z = y.split("-");
                    (
                        z.next().unwrap().parse().unwrap(),
                        z.next().unwrap().parse().unwrap(),
                    )
                })
                .collect::<Vec<(u32, u32)>>()
        })
        .collect();
}

fn part_one() -> usize {
    get_input().iter().fold(0, |mut sum, val| {
        let first = val.first().unwrap();
        let last = val.last().unwrap();
        if first.0 <= last.0 && first.1 >= last.1 || first.0 >= last.0 && first.1 <= last.1 {
            sum += 1;
        }
        sum
    })
}

fn part_two() -> usize {
    get_input().iter().fold(0, |mut sum, val| {
        let first = val.first().unwrap();
        let last = val.last().unwrap();
        if first.0 >= last.0 && first.0 <= last.1 {
            sum += 1;
        } else if first.1 >= last.0 && first.1 <= last.1 {
            sum += 1;
        } else if last.0 >= first.0 && last.0 <= first.1 {
            sum += 1;
        } else if last.1 >= first.0 && last.1 <= first.1 {
            sum += 1;
        }
        sum
    })
}

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}
