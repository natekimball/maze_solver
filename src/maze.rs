use std::{env::args, fs, fmt::{Display, self}};

use crate::maze_solver;

pub struct Maze {
    pub maze: Vec<Vec<Element>>,
    pub start: usize,
    pub solution_method: fn (maze: &mut Vec<Vec<Element>>, start: usize) -> bool
    // functional composition, strategy method is determined at runtime by a factory method
}

impl Maze {
    pub fn new() -> Maze {
        let (maze, start) = create_maze();
        let solution_method = maze_solver::solution_method_factory();
        Maze { maze, start, solution_method }
    }

    pub fn solve(&mut self) {
        println!("{self}");
        if (self.solution_method)(&mut self.maze, self.start) {
            println!("solution:");
            println!("{self}");
        } else {
            println!("no solution");
        }
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = self.maze.iter().map(|row| {
            row.iter().map(|element| {
                match element {
                    Element::Start => "S",
                    Element::End => "E",
                    Element::Wall => "X",
                    Element::Empty => " ",
                    Element::Path => "|",
                    _ => "?",
                }
            }).collect::<Vec<&str>>().join("")
        }).collect::<Vec<String>>().join("\n");
        write!(f, "{}", output)
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Element {
    Start,
    End,
    Wall,
    Empty,
    Path,
    Visiting
}
impl Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Element::Start => write!(f, "S"),
            Element::End => write!(f, "E"),
            Element::Wall => write!(f, "X"),
            Element::Empty => write!(f, " "),
            Element::Path => write!(f, "|"),
            Element::Visiting => write!(f, "?"),
        }
    }
}

pub fn create_maze() -> (Vec<Vec<Element>>, usize) {
    let args = args().collect::<Vec<String>>();
    if args.contains(&String::from("-f")) {
        let index = args.iter().position(|x| x == "-f").unwrap();
        let file = args.get(index+1).expect("no file specified");
        maze_from_file(file)
    } else {
        maze_from_file("resources/21x60maze.txt")
    }
}

fn maze_from_file(arg: &str) -> (Vec<Vec<Element>>, usize) {
    let content = fs::read_to_string(arg).expect("file not found");
    let mut start = 0;
    let mut end = 0;
    let maze: Vec<Vec<Element>> = content.lines().map(|line| {
        line.chars().enumerate().map(|(i,c)| {
            match c.to_ascii_uppercase() {
                'S' => {
                    start = i;
                    Element::Start
                },
                'E' => {
                    end = i;
                    Element::End
                },
                '#' => Element::Wall,
                ' ' => Element::Empty,
                _ => panic!("invalid character in maze")
            }
        }).collect()
    }).collect();
    if maze[0][start] != Element::Start || maze[maze.len()-1][end] != Element::End {
        panic!("invalid maze. start or end not found");
    }
    (maze, start)
}