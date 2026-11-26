mod grid;

use grid::Grid;

// Use main for game logic: looping & sleeping

fn main() {
    let mut g = Grid::new(10, 10);
    g.set(8, 7, true);
    g.set(9, 8, true);
    g.set(7, 9, true);
    g.set(8, 9, true);
    g.set(9, 9, true);
    for _ in 1..5 {
        println!("");
        g.step();
        g.print();
    }
}