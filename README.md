# Game of Life

An interactive implementation of Conway's Game of Life, written in Rust
with [macroquad](https://macroquad.rs/). Built as my first real Rust project.

## Controls

| Input | Action |
|-------|--------|
| `Space` | Pause / resume the simulation |
| Left click (while paused) | Toggle a cell alive / dead |

## Running it

You need a recent [Rust toolchain](https://rustup.rs/). Then:

```
git clone <repo-url>
cd game-of-life
cargo run --release
```

## How it works

- The grid is a flat `Vec<bool>` indexed as `y * width + x`. The `Grid`
  type (in `src/grid.rs`) owns that layout, so nothing outside it needs
  to know how cells are stored.
- Each generation is computed from the previous state before any cell
  is updated, so all cells change simultaneously.
- Cells outside the grid count as dead, so patterns such as the glider
  stop at the edges.
- Rendering and input live in `src/main.rs`. The simulation steps on a
  timer, independent of the frame rate.

## Ideas for later

- Random fill and clear-grid keys
- Toroidal (wrap-around) edges
- Loading starting patterns from a file
- Adjustable simulation speed