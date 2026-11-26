mod grid;
use macroquad::{
    prelude::*,
    input::{is_key_pressed, KeyCode}
};
use grid::Grid;

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    let mut grid = Grid::new(64, 64);
    const CELL_SIZE: f32 = 8.0; // Size of cells in pixels
    const SECONDS_PER_STEP: f32 = 0.5; // Seconds per 'step' aka generation
    let mut time_elapsed: f32 = 0.0;
    
    grid.set(10, 10, true);
    grid.set(11, 10, true);
    grid.set(12, 10, true);


    loop {
        clear_background(WHITE);

        for (x, y) in grid.get_alive_cells() {
            draw_rectangle(x as f32 * CELL_SIZE, y as f32 * CELL_SIZE, CELL_SIZE, CELL_SIZE, BLACK);
        }
        
        let delta = get_frame_time();
        time_elapsed += delta;
        if time_elapsed >= SECONDS_PER_STEP {
            grid.step();
            time_elapsed -= SECONDS_PER_STEP;
        }
        next_frame().await
    }
}