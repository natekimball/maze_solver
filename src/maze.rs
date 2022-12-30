use std::{env::args, fs, fmt::{Display, self}};

pub struct Maze {
    pub maze: Vec<Vec<Element>>,
    pub start: usize,
}

impl Maze {
    pub fn new() -> Maze {
        let args = args().collect::<Vec<String>>();
        if args.contains(&String::from("-f")) {
            let index = args.iter().position(|x| x == "-f").unwrap();
            let file = args.get(index+1).expect("no file specified");
            maze_from_file(file)
        } else {
            maze_from_file("resources/21x60maze.txt")
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

fn maze_from_file(arg: &str) -> Maze {
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
    Maze{maze, start}
}