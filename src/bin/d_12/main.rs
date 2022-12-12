use advent_of_Code_2022::grid::Grid;
use std::{collections::HashMap, path::Path, vec};

const START: char = 'S';
const END: char = 'E';

fn get_input() -> Grid<char> {
    let s = include_str!("input.txt")
        .lines()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    Grid::new(
        s[0].len(),
        s.len(),
        s.into_iter().flatten().collect::<Vec<char>>(),
    )
}

fn generate_height_map() -> HashMap<char, u16> {
    let mut height_map = HashMap::new();
    for c in "abcdefghijklmnopqrstuvwxyz".chars() {
        height_map.insert(c, (c as u16) - 'a' as u16);
    }
    height_map.insert(START, height_map.get(&'a').unwrap().clone());
    height_map.insert(END, height_map.get(&'z').unwrap().clone());
    height_map
}

fn get_path_vec(path_map: HashMap<usize, usize>, start: usize, end: usize) -> Vec<usize> {
    let mut current = end;
    let mut path = vec![current];

    while current != start {
        current = *path_map.get(&current).unwrap();
        path.push(current);
    }

    path.reverse(); // Reverse so we get the correct order of path
    path
}

fn a_star(
    scenic: bool,
    grid: &Grid<char>,
    height_map: &HashMap<char, u16>,
    start: usize,
    end: usize,
) -> Option<Vec<usize>> {
    let mut queue: Vec<usize> = vec![start];
    let mut path_nodes: HashMap<usize, usize> = HashMap::new();
    let mut g_score: HashMap<usize, u64> = HashMap::new();
    g_score.insert(start, 0);

    while !queue.is_empty() {
        let current = queue.pop().unwrap();

        if !scenic && current == end {
            return Some(get_path_vec(path_nodes, start, end));
        } else if scenic && grid.points[current] == 'a' {
            return Some(get_path_vec(path_nodes, start, current));
        }

        let (xu, yu) = grid.get_xy(current);
        // map to i32 to allow for negative values
        // easier to check bounds
        let (x, y) = (xu as i32, yu as i32);

        // check each neighbor, up, down, left and right of current node
        for (x2, y2) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            if x2 < 0 || x2 >= grid.width as i32 || y2 < 0 || y2 >= grid.height as i32 {
                continue;
            }

            let neighbor_index = grid.get_index(x2 as usize, y2 as usize);
            let neighbor_char = grid.get_value(neighbor_index);
            let current_char = grid.get_value(current);
            let neighbor_value = height_map.get(neighbor_char).unwrap();
            let current_value = height_map.get(current_char).unwrap();
            if !scenic && neighbor_value > current_value && neighbor_value - current_value > 1 {
                continue;
            } else if scenic && neighbor_value < current_value && current_value - neighbor_value > 1
            {
                continue;
            }

            let temp_g_score = g_score.get(&current).unwrap().clone() + 1;
            let neighbor_g_score = g_score.entry(neighbor_index).or_insert(u64::MAX);
            if &temp_g_score < neighbor_g_score {
                path_nodes.insert(neighbor_index, current);
                g_score.insert(neighbor_index, temp_g_score);
                queue.push(neighbor_index);
            }
        }

        // Sort the queue to make smallest score priority
        queue.sort_by(|x, y| g_score[y].cmp(&g_score[x]));
    }

    None // No valid paths
}

use image::RgbImage;
fn draw_path(
    name: &Path,
    path: Vec<usize>,
    height_map: HashMap<char, u16>,
    grid: Grid<char>,
) -> Result<(), image::ImageError> {
    let mut img_buffer: Vec<u8> = vec![];
    let gen_fac = grid
        .points
        .iter()
        .map(|x| *height_map.get(&x).unwrap())
        .reduce(|a, b| if a > b { a } else { b })
        .unwrap() as f32;

    for pix in grid.points {
        let value = *height_map.get(&pix).unwrap() as u32;

        // Heatmap algo go brrrr...
        let a_fac = gen_fac * 0.66;
        let red = if value > a_fac as u32 {
            255.0 * (value as f32 / 6.0)
        } else {
            0.0
        };

        let g_fac = gen_fac * 0.33;
        let green = if value > g_fac as u32 && value < (a_fac + 2.0) as u32 {
            255.0 * (value as f32 / (a_fac + 2.0))
        } else {
            0.0
        };

        let blue = if value >= 1 && value < (g_fac + 2.0) as u32 {
            255.0 * (value as f32 / (g_fac + 2.0))
        } else {
            0.0
        };

        img_buffer.push(red as u8);
        img_buffer.push(green as u8);
        img_buffer.push(blue as u8);
    }

    for i in &path {
        img_buffer[*i * 3] = 255;
        img_buffer[*i * 3 + 1] = 255;
        img_buffer[*i * 3 + 2] = 255;
    }

    let img = RgbImage::from_raw(grid.width as u32, grid.height as u32, img_buffer)
        .expect("Width and height should match the buffer");
    img.save(name)?;
    Ok(())
}

fn part_one() -> usize {
    let height_map = generate_height_map();
    let input = get_input();
    let start = input.points.iter().position(|x| x == &START).unwrap();
    let end = input.points.iter().position(|x| x == &END).unwrap();

    let path = a_star(false, &input, &height_map, start, end);
    if path.is_some() {
        let pu = path.unwrap();
        draw_path(
            Path::new("src/bin/d_12/p1.png"),
            pu.clone(),
            height_map,
            input,
        )
        .expect("Problem creating the image");
        pu.len() - 1
    } else {
        0
    }
}

fn part_two() -> usize {
    let height_map = generate_height_map();
    let input = get_input();
    let start = input.points.iter().position(|x| x == &START).unwrap();
    let end = input.points.iter().position(|x| x == &END).unwrap();

    let path = a_star(true, &input, &height_map, end, start);
    if path.is_some() {
        let pu = path.unwrap();
        draw_path(
            Path::new("src/bin/d_12/p2.png"),
            pu.clone(),
            height_map,
            input,
        )
        .expect("Problem creating the image");
        pu.len() - 1
    } else {
        0
    }
}
fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}
