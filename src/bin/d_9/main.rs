use std::f64::consts::PI;
const TWO_PI: f64 = 2.0 * PI;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coord {
    x: f64,
    y: f64,
}

impl Coord {
    fn new(x: f64, y: f64) -> Coord {
        Coord { x, y }
    }

    fn get_angle(&self, tar: Coord) -> f64 {
        let deg = f64::atan2(tar.y - self.y, tar.x - self.x);
        ((TWO_PI + deg) * 180.0 / PI) % 360.0
    }

    fn get_distance(&self, tar: Coord) -> f64 {
        ((tar.x - self.x).powf(2.0) + (tar.y - self.y).powf(2.0)).sqrt()
    }
}

fn get_input() -> Vec<(&'static str, usize)> {
    return include_str!("input.txt")
        .lines()
        .map(|s| {
            let s = s.split_once(" ").unwrap();
            (s.0, s.1.parse().unwrap())
        })
        .collect();
}

fn move_head(head: &mut Coord, dir: &str) {
    match dir {
        "U" => {
            head.y += 1.0;
        }
        "D" => {
            head.y -= 1.0;
        }
        "L" => {
            head.x -= 1.0;
        }
        "R" => {
            head.x += 1.0;
        }
        _ => (),
    }
}

fn move_tail(tail: &mut Coord, head: Coord) {
    // Tail doesn't move unless distance to head is 2 or above
    if tail.get_distance(head).floor() <= 1.0 {
        return;
    }

    let angle = tail.get_angle(head).floor() as i64;
    match angle {
        0 => tail.x += 1.0,
        90 => tail.y += 1.0,
        180 => tail.x -= 1.0,
        270 => tail.y -= 1.0,
        1..=89 => {
            tail.x += 1.0;
            tail.y += 1.0;
        }
        91..=179 => {
            tail.x -= 1.0;
            tail.y += 1.0;
        }
        181..=269 => {
            tail.x -= 1.0;
            tail.y -= 1.0;
        }
        271..=359 => {
            tail.x += 1.0;
            tail.y -= 1.0;
        }
        _ => (),
    }
}

#[allow(dead_code)]
// This was mainly used for testing.
// Real input would require animation or something.
// todo neato simulation animation?
fn draw_map(coords: Vec<Coord>) {
    print!("\n");
    for y in (0..=26).rev() {
        for x in 0..=20 {
            let off_x = x as i64 - 10;
            let off_y = y as i64 - 13;

            let found = coords
                .iter()
                .any(|z| off_x == z.x as i64 && off_y == z.y as i64);
            let ch = if found {
                '#'
            } else {
                if x == 10 && y == 13 {
                    's'
                } else {
                    '.'
                }
            };
            print!("{}", ch);
        }
        print!("\n");
    }
    print!("\n");
}

fn part_one() -> usize {
    let input = get_input();
    let start = Coord::new(0.0, 0.0);
    let mut head = start.clone();
    let mut tail = start.clone();
    let mut tail_pos: Vec<Coord> = vec![];

    for (dir, amount) in input {
        for _ in 0..amount {
            move_head(&mut head, dir);
            move_tail(&mut tail, head);

            if !tail_pos.contains(&tail) {
                tail_pos.push(tail.clone());
            }
        }
        // draw_map(vec![head, tail]);
    }

    tail_pos.len()
}

fn part_two() -> usize {
    let input = get_input();
    let start = Coord::new(0.0, 0.0);
    let mut head = start.clone();
    let mut tail = start.clone();
    let mut tail_pos: Vec<Coord> = vec![];

    // Add 8 subsegments of the rope
    let mut rope: Vec<Coord> = vec![];
    for _ in 0..8 {
        rope.push(start.clone());
    }

    for (dir, amount) in input {
        for _ in 0..amount {
            move_head(&mut head, dir);

            for i in 0..rope.len() {
                let h = if i == 0 { head } else { rope[i - 1] };
                move_tail(&mut rope[i], h);
            }

            move_tail(&mut tail, *rope.last().unwrap());

            if !tail_pos.contains(&tail) {
                tail_pos.push(tail.clone());
            }
        }

        // let mut printable = vec![head, tail];
        // printable.append(&mut rope.clone());
        // draw_map2(printable);
    }

    tail_pos.len()
}

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}
