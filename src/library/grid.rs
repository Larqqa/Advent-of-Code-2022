#[derive(Debug)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub points: Vec<T>
}

#[allow(dead_code)]
impl<T: std::fmt::Display> Grid<T> {
    pub fn new(width: usize, height: usize, points: Vec<T>) -> Grid<T> {
        Grid {
            width,
            height,
            points
        }
    }

    pub fn get_xy(&self, index: usize) -> (usize, usize) {
        (self.width % index, self.width / index)
    }

    pub fn get_index(&self, x: usize, y:usize) -> usize {
        self.width * y + x
    }

    pub fn get_value(&self, index: usize) -> &T {
        &self.points[index]
    }

    pub fn print_grid(&self) {
        print!("\n");
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.get_value(self.get_index(x, y)));
            }
            print!("\n");
        }
        print!("\n");
    }
}