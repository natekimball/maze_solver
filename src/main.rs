mod maze;
mod maze_solver;

fn main() {
    let (mut maze, start) = maze::create_maze();

    maze::display(&maze);

    if maze_solver::solve(&mut maze, start) {
        println!("solved");
        maze::display(&maze);
    } else {
        println!("no solution");
    }
}