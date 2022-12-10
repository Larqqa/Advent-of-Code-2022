const WIDTH: usize = 40;
const HEIGHT: usize = 6;

fn run_program() -> i32 {
    let mut input: Vec<&str> = include_str!("input.txt").lines().rev().collect();
    let mut wait = 0;
    let mut add: Option<i32> = None;
    let mut cycles: usize = 0;
    let mut register = 1;
    let mut check_cycles = 20;
    let mut signals: Vec<i32> = vec![];
    let mut crt = ['.'; (WIDTH * HEIGHT)];

    while input.len() > 0 {
        cycles += 1;

        if wait == 0 {
            if add.is_some() {
                register += add.unwrap();
                add = None;
            }

            // println!("{}: {}, add: {:?}, wait: {}", cycles, register, add, wait);

            let instruction = input.pop().unwrap();
            if instruction != "noop" {
                let (_, cs) = instruction.split_once(" ").unwrap();
                wait = 1;
                add = Some(cs.parse().unwrap());
            }
        } else {
            wait -= 1;
            // println!("{}: waiting...", cycles);
        }

        let index = cycles - 1;
        let x = (index % WIDTH) as i32;
        if [x - 1, x, x + 1].contains(&register) {
            crt[index] = '#';
        }

        if check_cycles == cycles {
            signals.push(register * cycles as i32);
            check_cycles += 40;
        }
    }

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let i = x + WIDTH * y;
            print!("{}", crt[i]);
        }
        print!("\n");
    }

    signals.iter().sum()
}

fn main() {
    println!("part one: {}", run_program());
}
