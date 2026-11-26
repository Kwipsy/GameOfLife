mod grid;

use std::{
    thread::sleep,
    time::Duration
};

use grid::Grid;

// Use main for game logic: looping & sleeping

fn main() {
    const WAIT_TIME: Duration = std::time::Duration::from_millis(500);
    let mut g = Grid::new(10, 10);
    g.set(0, 1, true);
    g.set(1, 1, true);
    g.set(2, 1, true);

    loop {
        g.print();
        std::thread::sleep(WAIT_TIME);
        g.step();
    }
}
