use std::collections::HashMap;

fn part_one() -> u16 {
    include_str!("input.txt")
        .lines()
        .map(|s| {
            let half = s.len() / 2;
            (s[0..half].to_string(), s[half..s.len()].to_string())
        })
    .map(|(a, b)| {
        a.chars()
        .filter(|x| b.contains(*x))
        .collect::<Vec<char>>()[0]
    })
    .map(|x| if x.is_lowercase() {(x as u16) - (('a' as u16) - 1)} else {26 + (x as u16) - (('A' as u16) - 1)})
    .sum::<u16>()
}

fn part_two() -> u16 {
    let input:Vec<Vec<char>> = include_str!("input.txt")
        .lines()
        .map(|x| x.chars().collect())
        .map(|mut x: Vec<char>| {x.sort(); x.dedup(); x})
        .collect();
    let c: Vec<&[Vec<char>]> = input.chunks(3).collect();

    let mut total = 0;
    for k in c {
        let mut map: HashMap<&char, u16> = HashMap::new();
        for chars in k {
            for key in chars {
                let count = map.entry(key).or_insert(0);
                *count += 1;
            }
        }

        for (k,v) in map {
            if v == 3 {
                total += if k.is_lowercase() {(*k as u16) - (('a' as u16) - 1)} else {26 + (*k as u16) - (('A' as u16) - 1)};
            }
        }
    }
    total
}

fn main() {
     println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}