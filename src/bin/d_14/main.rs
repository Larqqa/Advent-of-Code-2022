use advent_of_Code_2022::grid::Grid;
use std::cmp::{max, min};

const ROCK: char = '#';
const SAND: char = 'o';
const AIR: char = '.';
const SAND_START: (usize, usize) = (500, 0);

fn get_input() -> Vec<Vec<(usize, usize)>> {
    include_str!("input.txt")
        .lines()
        .map(|z| {
            z.split(" -> ")
                .map(|x| x.split_once(",").unwrap())
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<Vec<(usize, usize)>>>()
}

fn map_to_grid(
    input: Vec<Vec<(usize, usize)>>,
    floor: bool,
) -> (Grid<char>, Vec<Vec<(usize, usize)>>, (usize, usize)) {
    let xs = input.iter().flatten().map(|x| x.0);
    let ys = input.iter().flatten().map(|x| x.1);

    let x_max = xs.clone().max().unwrap();
    let mut x_min = xs.min().unwrap();

    let mut y_max = ys.clone().max().unwrap();
    // let y_min = ys.min().unwrap();

    let width = if floor {
        y_max = y_max + 2;
        x_min = x_min - (y_max - (SAND_START.0 - x_min));
        y_max * 2 + 1
    } else {
        (x_max - x_min) + 1
    };

    let height = y_max + 1;

    let start = (SAND_START.0 - x_min, SAND_START.1);

    let mut coords: Vec<Vec<(usize, usize)>> = input
        .iter()
        .map(|z| z.iter().map(|(x, y)| (x - x_min, *y)).collect())
        .collect();

    // Add floor to rock coords
    if floor {
        coords.push(vec![(0, y_max), (width - 1, y_max)]);
    }

    let mut map = Grid::new(width, height, vec![AIR; width * height]);

    // Fill map with rocks
    for coord in coords.clone() {
        let mut last = coord[0];
        for (x, y) in coord {
            let xs = (min(x, last.0), max(x, last.0));
            let ys = (min(y, last.1), max(y, last.1));
            for y2 in ys.0..=ys.1 {
                for x2 in xs.0..=xs.1 {
                    map.set_value(map.get_index(x2, y2), ROCK);
                }
            }

            last = (x, y);
        }
    }

    map.set_value(map.get_index(start.0, start.1), '+');

    (map, coords, start)
}

#[derive(Debug, Clone, Copy)]
struct Sand {
    x: usize,
    y: usize,
}

impl Sand {
    fn new(x: usize, y: usize) -> Sand {
        Sand { x, y }
    }

    fn fall(&mut self, grid: &Grid<char>) -> bool {
        let mut last_y = self.y;
        loop {
            if self.x == 0 || self.x >= grid.width || self.y >= grid.height {
                return false;
            }

            self.y += 1;

            if grid.get_value(grid.get_index(self.x, self.y)) != &AIR {
                if grid.get_value(grid.get_index(self.x - 1, self.y)) == &AIR {
                    self.x -= 1;
                } else if grid.get_value(grid.get_index(self.x + 1, self.y)) == &AIR {
                    self.x += 1;
                } else {
                    self.y -= 1;
                }
            }

            // If stopped
            if last_y == 0 && self.y == 0 {
                return false;
            } else if last_y == self.y {
                break;
            }
            last_y = self.y;
        }
        true // continue sending sand, false means map's all full
    }
}

fn part_one() -> usize {
    let (mut map, coords, start) = map_to_grid(get_input(), false);

    let mut i = 0;
    loop {
        let mut s = Sand::new(start.0, start.1);

        if s.fall(&map) {
            map.set_value(map.get_index(s.x, s.y), SAND);
            i += 1;
        } else {
            break;
        }
    }

    // map.print_grid();

    i
}

fn part_two() -> usize {
    let (mut map, coords, start) = map_to_grid(get_input(), true);
    let mut i = 0;
    loop {
        let mut s = Sand::new(start.0, start.1);

        if s.fall(&map) {
            map.set_value(map.get_index(s.x, s.y), SAND);
            i += 1;
        } else {
            break;
        }
    }

    // map.print_grid();

    i + 1
}

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}
