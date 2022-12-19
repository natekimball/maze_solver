use crate::maze::Maze;

mod maze;
mod maze_solver;

fn main() {
    let mut maze = Maze::new();
    maze.solve();
}