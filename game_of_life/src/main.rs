mod grid;

use grid::Grid;

// Use main for game logic: looping & sleeping

fn main() {
    let mut g = Grid::new(10, 10);
    g.set(0, 0, true);
    g.set(1, 0, true);
    g.set(0, 1, true);
    g.print();
}