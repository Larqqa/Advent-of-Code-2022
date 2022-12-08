use advent_of_Code_2022::grid::Grid;

fn get_input() -> Grid<u32> {
    let s = include_str!("input.txt")
        .lines()
        .map(|s| s.chars().map(|x| x.to_string().parse().unwrap()).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();

    Grid::new(s[0].len(), s.len(), s.into_iter().flatten().collect::<Vec<u32>>())
}

fn calculate_visible(grid: Grid<u32>) -> (usize, i32) {
    let mut biggest_scenic_score = 0;
    let mut total_visible = (grid.height - 2) * 2 + grid.width * 2; // edges are always visible
    for y in 1..grid.height - 1 {
        for x in 1..grid.width - 1 {
            let i = grid.get_index(x, y);
            let val = grid.get_value(i);
            let mut any_visible = false;
            let mut scenic_score = 1;

            let mut direction_clear = true;
            let mut x2 = x - 1;
            let mut distance = 0;
            loop {
                let i2 = grid.get_index(x2, y);
                let val2 = grid.get_value(i2);
                distance += 1;
                if val <= val2 {direction_clear = false; break;}
                if x2 == 0 {break}
                x2 -= 1;
            }
            if direction_clear {any_visible = true;}
            scenic_score = scenic_score * distance;

            direction_clear = true;
            x2 = x + 1;
            distance = 0;
            while x2 < grid.width {
                let i2 = grid.get_index(x2, y);
                let val2 = grid.get_value(i2);
                distance += 1;
                if val <= val2 {direction_clear = false; break;}
                x2 += 1;
            }
            if direction_clear {any_visible = true;}
            scenic_score = scenic_score * distance;

            direction_clear = true;
            let mut y2 = y - 1;
            distance = 0;
            loop {
                let i2 = grid.get_index(x, y2);
                let val2 = grid.get_value(i2);
                distance += 1;
                if val <= val2 {direction_clear = false; break;}
                if y2 == 0 {break}
                y2 -= 1;
            }
            if direction_clear {any_visible = true;}
            scenic_score = scenic_score * distance;

            direction_clear = true;
            y2 = y + 1;
            distance = 0;
            while y2 < grid.height {
                let i2 = grid.get_index(x, y2);
                let val2 = grid.get_value(i2);
                distance += 1;
                if val <= val2 {direction_clear = false; break;}
                y2 += 1;
            }
            if direction_clear {any_visible = true;}
            scenic_score = scenic_score * distance;

            if any_visible {
                total_visible += 1;
            }

            if biggest_scenic_score < scenic_score  {
                biggest_scenic_score = scenic_score;
            }
        }
    }

    (total_visible, biggest_scenic_score)
}

fn main() {
    let input = get_input();
    let (total, score) = calculate_visible(input);
    println!("part one: {}", total);
    println!("part two: {}", score);
}