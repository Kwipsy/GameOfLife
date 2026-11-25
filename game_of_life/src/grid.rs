// Use Grid for grid logic: Struct, neighbour counting & next-generation logic

pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<bool>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Grid { width, height, cells: vec![false; width * height] }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        self.cells[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, alive: bool) {
        self.cells[y * self.width + x] = alive;
    }

    pub fn dimensions(&self) -> (usize, usize) {
        return (self.width, self.height)
    }

    pub fn print(&self) {
        for row in self.cells.chunks(self.width) {
            let row: String = row.iter()
                .map(|&b| if b {'#'} else {'.'})
                .collect();
            println!("{}", row);
        }
    }
}