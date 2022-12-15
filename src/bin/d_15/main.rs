use std::cmp::{max, min};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
struct Sensor {
    location: Point,
    closest: Point,
    distance: i64,
}

fn get_input() -> (Vec<Sensor>, Vec<Point>) {
    let s: Vec<&str> = include_str!("input.txt").lines().collect();
    let mut sensors = vec![];
    let mut beacons = vec![];
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

        beacons.push(beacon);
    });
    (sensors, beacons)
}

fn manhattan_distance(p1: Point, p2: Point) -> i64 {
    let x = max(p2.x, p1.x) - min(p2.x, p1.x);
    let y = max(p2.y, p1.y) - min(p2.y, p1.y);
    x + y
    // let a = f64::powf(2.0, x as f64);
    // let b = f64::powf(2.0, y as f64);
    // f64::sqrt(a + b).ceil() as i64
}

//https://stackoverflow.com/questions/3838329/how-can-i-check-if-two-segments-intersect
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

fn part_one() -> i64 {
    let (sensors, beacons) = get_input();
    // println!("{:?}", sensors);
    let sxs = sensors.iter().map(|x| x.location.x).collect::<Vec<i64>>();
    let sys = sensors.iter().map(|x| x.location.y).collect::<Vec<i64>>();
    let bxs = beacons.iter().map(|x| x.x).collect::<Vec<i64>>();
    let bys = beacons.iter().map(|x| x.y).collect::<Vec<i64>>();

    let min_x = *min(sxs.iter().min(), bxs.iter().min()).unwrap();
    let max_x = *max(sxs.iter().max(), bxs.iter().max()).unwrap();
    let min_y = *min(sys.iter().min(), bys.iter().min()).unwrap();
    let max_y = *max(sys.iter().max(), bys.iter().max()).unwrap();
    // println!("bounds: {},{},{},{}", min_x, min_y, max_x, max_y);
    let mut ranges = vec![];

    let y = 2000000;
    // let y = 10;
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
    // println!("{:?}", ranges);

    let mut i = 1;
    let mut last_len = ranges.len();
    loop {
        while i < ranges.len() {
            let prev = ranges[i - 1];
            let curr = ranges[i];
            if prev.0 < curr.0 && prev.1 > curr.1 {
                ranges.remove(i);
            } else if prev.1 >= curr.0 {
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

    println!("{:?}", ranges);

    ranges[0].1 - ranges[0].0
}

// fn part_two() -> i32 {
//     0
// }

fn main() {
    println!("part one: {}", part_one());
    // println!("part two: {}", part_two());
}
