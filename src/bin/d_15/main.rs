use std::cmp::{max, min};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
struct Sensor {
    location: Point,
    closest: Point,
    distance: i64,
}

fn get_input() -> Vec<Sensor> {
    let s: Vec<&str> = include_str!("input.txt").lines().collect();
    let mut sensors = vec![];
    s.iter().for_each(|x| {
        let a = x
            .replace("closest beacon is at", "")
            .replace("Sensor at ", "");
        let (s, b) = a.split_once(": ").unwrap();

        let (bx, by) = b.split_once(", ").unwrap();
        let beacon = Point {
            x: bx.split_once("=").unwrap().1.parse().unwrap(),
            y: by.split_once("=").unwrap().1.parse().unwrap(),
        };

        let (sx, sy) = s.split_once(", ").unwrap();
        let sensor = Point {
            x: sx.split_once("=").unwrap().1.parse().unwrap(),
            y: sy.split_once("=").unwrap().1.parse().unwrap(),
        };

        sensors.push(Sensor {
            location: sensor,
            closest: beacon.clone(),
            distance: manhattan_distance(sensor, beacon),
        });
    });
    sensors
}

fn manhattan_distance(p1: Point, p2: Point) -> i64 {
    let x = max(p2.x, p1.x) - min(p2.x, p1.x);
    let y = max(p2.y, p1.y) - min(p2.y, p1.y);
    x + y
}

fn ccw(a: Point, b: Point, c: Point) -> bool {
    (c.y - a.y) * (b.x - a.x) > (b.y - a.y) * (c.x - a.x)
}

fn lines_intersect(p1: Point, p2: Point, d1: Point, d2: Point) -> bool {
    ccw(p1, d1, d2) != ccw(p2, d1, d2) && ccw(p1, p2, d1) != ccw(p1, p2, d2)
}

fn find_x(p1: Point, p2: Point, y: i64) -> i64 {
    let a = (p1.y - p2.y) as f64;
    let b = (p1.x - p2.x) as f64;
    let m = a / b;
    let c = p1.y as f64 - m * p1.x as f64;
    ((y as f64 - c) / m).round() as i64
}

fn find_x_ranges_at_y(sensors: &Vec<Sensor>, y: i64) -> Vec<(i64, i64)> {
    let mut ranges = vec![];
    for sensor in sensors {
        let top = Point {
            x: sensor.location.x,
            y: sensor.location.y - sensor.distance,
        };
        let bottom = Point {
            x: sensor.location.x,
            y: sensor.location.y + sensor.distance,
        };
        let left = Point {
            x: sensor.location.x - sensor.distance,
            y: sensor.location.y,
        };
        let right = Point {
            x: sensor.location.x + sensor.distance,
            y: sensor.location.y,
        };

        let left_bound = Point {
            x: -500000000000,
            y,
        };
        let right_bound = Point { x: 500000000000, y };

        let mut x = 0;
        if lines_intersect(left, top, left_bound, right_bound) {
            x = find_x(left, top, y);
            ranges.push((x, sensor.location.x + sensor.location.x - x));
        } else if lines_intersect(left, bottom, left_bound, right_bound) {
            x = find_x(left, bottom, y);
            ranges.push((x, sensor.location.x + sensor.location.x - x));
        }
    }

    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut i = 1;
    let mut last_len = ranges.len();
    loop {
        while i < ranges.len() {
            let prev = ranges[i - 1];
            let curr = ranges[i];
            if prev.0 <= curr.0 && prev.1 >= curr.1 {
                ranges.remove(i);
            } else if prev.1 >= curr.0 || prev.1 == curr.0 - 1 {
                ranges[i - 1].1 = curr.1;
                ranges.remove(i);
            } else {
                i += 1;
            }
        }

        if last_len == ranges.len() {
            break;
        } else {
            last_len = ranges.len();
        }
    }
    ranges
}

fn part_one() -> i64 {
    let sensors = get_input();
    let ranges = find_x_ranges_at_y(&sensors, 2000000);
    ranges[0].1 - ranges[0].0
}

fn part_two() -> i64 {
    let sensors = get_input();
    let mut ranges = vec![];

    let mut y = 0;
    while y <= 4000000 {
        ranges = find_x_ranges_at_y(&sensors, y);

        if ranges.len() > 1 {
            break;
        }

        ranges.clear();
        y += 1;
    }

    (ranges[0].1 + 1) * 4000000 + y
}

fn main() {
    println!("part one: {}", part_one());
    println!("part two: {}", part_two());
}
