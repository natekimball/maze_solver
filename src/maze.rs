// OOPize

use std::{env::args, fs};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Element {
    Start,
    End,
    Wall,
    Empty,
    Path,
    Visiting
}

pub fn display(maze: &Vec<Vec<Element>>) {
    for i in 0..maze.len() {
        for j in 0..maze[i].len() {
            match maze[i][j] {
                Element::Start => print!("S"),
                Element::End => print!("E"),
                Element::Wall => print!("X"),
                Element::Empty => print!(" "),
                Element::Path => print!("|"),
                _ => print!("?"),
            }
        }
        println!();
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