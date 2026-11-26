// Use Grid for grid logic: Struct, neighbour counting & next-generation logic

use std::vec;

pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<bool>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Grid { width, height, cells: vec![false; width * height] }
    }

    const NEIGHBORHOOD: [(isize, isize); 8] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

    pub fn get(&self, x: usize, y: usize) -> Option<bool> {
        if x < self.width && y < self.height {
            return Some(self.cells[y * self.width + x])
        }
        None
    }

    pub fn set(&mut self, x: usize, y: usize, is_alive: bool) {
        self.cells[y * self.width + x] = is_alive;
    }

    pub fn count_neighbors(&self, x: usize, y: usize) -> usize {
        Self::NEIGHBORHOOD
            .into_iter()
            .filter(|&offset| self.is_neighbor_alive(x, y, offset))
            .count()
    }

    fn is_neighbor_alive(&self, x: usize, y: usize, (dx, dy): (isize, isize)) -> bool {
        x.checked_add_signed(dx)
            .zip(y.checked_add_signed(dy))
            .and_then(|(a, b)| self.get(a, b))
            .unwrap_or(false)
    }

    pub fn step(&mut self) {
        self.cells = self.cells
            .iter()
            .enumerate()
            .map(|(i, &state)| {
                let (x, y) = (i % self.width, i / self.width);
                let neighbors = self.count_neighbors(x, y);
                match neighbors {
                    3 => true,
                    2 => state,
                    _ => false
                }
            })
            .collect();
    }

    pub fn print(&self) {
        for row in self.cells.chunks(self.width) {
            let row: String = row
                .iter()
                .map(|&b| if b {'#'} else {'.'})
                .collect();
            println!("{}", row);
        }
    }
}