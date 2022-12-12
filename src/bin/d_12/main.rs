use std::{collections::{HashMap, HashSet}, iter::Map};
use advent_of_Code_2022::grid::Grid;

const START: char = 'S';
const END: char = 'E';
const height_chars: &str = "abcdefghijklmnopqrstuvwxyz";
const unicode_offset: u16 = ('a' as u16);

fn get_input() -> Grid<char> {
    let s = include_str!("input.txt")
        .lines()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    Grid::new(s[0].len(), s.len(), s.into_iter().flatten().collect::<Vec<char>>())
}

fn get_path_vec(path_map: HashMap<usize, usize>, start: usize, end: usize) -> Vec<usize> {
    let mut current = end;
    let mut path = vec![current];

    while current != start {
        current = *path_map.get(&current).unwrap();
        path.push(current);
    }

    path.reverse();
    path
}

fn a_star(scenic: bool, grid: &Grid<char>, height_map: &HashMap<char, u16>, start :usize, end:usize, heruistic: fn(usize) -> u64) -> Option<Vec<usize>> {
    let mut queue: Vec<usize> = vec![start];
    let mut path: HashMap<usize, usize> = HashMap::new();

    let mut g_score: HashMap<usize, u64> = HashMap::new();
    g_score.insert(start, 0);

    while !queue.is_empty() {
        queue.sort_by(|x, y| g_score[y].cmp(&g_score[x]));
        // println!("{:?}", queue.iter().map(|x| (x, g_score[x])).collect::<Vec<(&usize, u64)>>());
        let current = queue.pop().unwrap();

        if !scenic && current == end {
            return Some(get_path_vec(path, start, end));
        }

        if scenic && grid.points[current] == 'a' {
            return Some(get_path_vec(path, start, current));
        }

        let (xu, yu) = grid.get_xy(current);
        let (x, y) = (xu as i32, yu as i32);

        for (x2,y2) in [(x-1, y), (x+1, y), (x, y-1), (x, y+1)] {
            if x2 < 0 || x2 >= grid.width as i32 || y2 < 0 || y2 >= grid.height as i32 {
                continue;
            }

            let neighbor_index = grid.get_index(x2 as usize, y2 as usize);
            let neighbor_val = grid.get_value(neighbor_index);
            let current_val = grid.get_value(current);
            let n_v = height_map.get(neighbor_val).unwrap();
            let c_v = height_map.get(current_val).unwrap();
            if !scenic && n_v > c_v && n_v - c_v > 1 {
                continue;
            }
            if scenic && n_v < c_v && c_v - n_v > 1 {
                continue;
            }
            let t_g_score = g_score.get(&current).unwrap().clone() + 1;
            let n_g_score = g_score.entry(neighbor_index).or_insert(u64::MAX);
            if &t_g_score < n_g_score {
                path.insert(neighbor_index, current);
                g_score.insert(neighbor_index, t_g_score);
                queue.push(neighbor_index);
            }
        }
    }

    None // No valid paths
}

fn h(a: usize) -> u64 {
    0
}

fn part_one() -> usize {
    let mut height_map = HashMap::new();
    for c in height_chars.chars() {
        height_map.insert(c, (c as u16) - unicode_offset);
    }
    height_map.insert(START, height_map.get(&'a').unwrap().clone());
    height_map.insert(END, height_map.get(&'z').unwrap().clone());

    let mut input = get_input();
    // input.print_grid();
    let start = input.points.iter().position(|x| x == &START).unwrap();
    let end = input.points.iter().position(|x| x == &END).unwrap();

    let p = a_star(false, &input, &height_map, start, end, h);
    // println!("{:?}", p);

    if p.is_some() {
        let pu = p.unwrap();
        let len =      pu.len() - 1;
        input.points = input.points.iter().map(|_| '░').collect();
        for i in pu {
            input.points[i] = '█';
        }
        // input.print_grid();
        len
    } else {
        0
    }
}

fn part_two() -> usize {
    let mut height_map = HashMap::new();
    for c in height_chars.chars() {
        height_map.insert(c, (c as u16) - unicode_offset);
    }
    height_map.insert(START, height_map.get(&'a').unwrap().clone());
    height_map.insert(END, height_map.get(&'z').unwrap().clone());

    let mut input = get_input();
    // input.print_grid();
    let start = input.points.iter().position(|x| x == &START).unwrap();
    let end = input.points.iter().position(|x| x == &END).unwrap();

    let p = a_star(true, &input, &height_map, end, start, h);
    // println!("{:?}", p);

    if p.is_some() {
        let pu = p.unwrap();
        let len =      pu.len() - 1;
        input.points = input.points.iter().map(|_| '░').collect();
        for i in pu {
            input.points[i] = '█';
        }
        // input.print_grid();
        len
    } else {
        0
    }
}
fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}