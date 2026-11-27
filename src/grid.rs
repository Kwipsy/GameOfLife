// Use Grid for grid logic: Struct, neighbour counting & next-generation logic

use std::vec;

pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<bool>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            width,
            height,
            cells: vec![false; width * height],
        }
    }

    const NEIGHBORHOOD: [(isize, isize); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn index_to(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<bool> {
        if x < self.width && y < self.height {
            return Some(self.cells[self.index(x, y)]);
        }
        None
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut bool> {
        if x < self.width && y < self.height {
            let index = self.index(x, y);
            return Some(&mut self.cells[index]);
        }
        None
    }

    pub fn get_alive_cells(&self) -> impl Iterator<Item = (usize, usize)> {
        self.cells
            .iter()
            .enumerate()
            .filter(|&(_, &isalive)| isalive)
            .map(|(index, _)| self.index_to(index))
    }

    pub fn set(&mut self, x: usize, y: usize, is_alive: bool) {
        if let Some(cell) = self.get_mut(x, y) {
            *cell = is_alive
        }
    }

    pub fn flip(&mut self, x: usize, y: usize) {
        if let Some(cell) = self.get_mut(x, y) {
            *cell = !*cell;
        }
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
        self.cells = self
            .cells
            .iter()
            .enumerate()
            .map(|(i, &state)| {
                let (x, y) = self.index_to(i);
                let neighbors = self.count_neighbors(x, y);
                match neighbors {
                    3 => true,
                    2 => state,
                    _ => false,
                }
            })
            .collect();
    }
}
