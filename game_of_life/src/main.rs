mod grid;
use macroquad::{
    prelude::*,
};
use grid::Grid;

const CELL_SIZE: f32 = 10.0; // Size of cells in pixels

#[macroquad::main("Tobias' Game of Life")]
async fn main() {
    let mut grid = Grid::new(80, 60); // Arbitrary size, fits decently with the window size
    const SECONDS_PER_STEP: f32 = 0.5; // Seconds per 'step' aka generation
    let mut time_elapsed: f32 = 0.0; // Time since last step
    let mut paused: bool = false;
    
    grid.set(10, 10, true);
    grid.set(11, 10, true);
    grid.set(12, 10, true);


    loop {
        clear_background(WHITE);

        for (x, y) in grid.get_alive_cells() {
            let (x, y) = coords_to_pixels(x, y);
            draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, BLACK);
        }

        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        
        if paused {
            draw_text("PAUSED", screen_width() - 200.0, screen_height() - 60.0, 50.0, DARKGRAY);
            if is_mouse_button_pressed(MouseButton::Left) {
                let (x, y) = pixels_to_coords(mouse_position());
                grid.flip(x, y);
            }
        } else {
            time_elapsed += get_frame_time();
            if time_elapsed >= SECONDS_PER_STEP {
                grid.step();
                time_elapsed -= SECONDS_PER_STEP;
            }
        }
        next_frame().await
    }
}

fn coords_to_pixels(x: usize, y: usize) -> (f32, f32) {
    (x as f32 * CELL_SIZE, y as f32 * CELL_SIZE)
}

fn pixels_to_coords((x, y): (f32, f32)) -> (usize, usize) {
    ((x / CELL_SIZE) as usize, (y / CELL_SIZE) as usize)
}