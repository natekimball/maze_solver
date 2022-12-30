use crate::maze::Maze;

mod maze;
mod maze_solver;

fn main() {
    let mut maze = Maze::new();
    let solver = maze_solver::strategy_factory();
    println!("{maze}");
    if solver.solve(&mut maze) {
        println!("Solved:");
        println!("{maze}");
    } else {
        println!("No solution found");
    }
}