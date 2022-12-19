use queues::*;
use std::{env::args, collections::HashMap};

use crate::maze::Element;

//returns the proper stategy for maze solving
pub fn solution_method_factory() -> fn (maze: &mut Vec<Vec<Element>>, start: usize) -> bool {
    let args = args().collect::<Vec<String>>();
    if args.contains(&String::from("-s")) || args.contains(&String::from("--shortest")) {
        find_shortest_path
    } else if args.contains(&String::from("-a")) || args.contains(&String::from("--any")) {
        find_path
    } else {
        find_all_paths
    }
}

// breadth first search
fn find_shortest_path(maze: &mut Vec<Vec<Element>>, start: usize) -> bool {
    let mut parents = HashMap::new();
    let mut queue = Queue::new();
    queue.add((0,start)).unwrap();

    while queue.size() > 0 {
        let (i,j) = queue.remove().unwrap();
        // println!("maze[{i}][{j}] = {:?}", maze[i][j]);
        for (x,y) in [(0,1),(1,0),(0,-1 as i32),(-1 as i32,0)] {
            let (new_x, new_y) = (i as i32 + x, j as i32 + y);
            if (i==0 && x==-1) || new_x >= maze.len() as i32 || (j==0 && y==-1) || new_y >= maze[0].len() as i32 {
                continue;
            }
            let (new_x, new_y) = (new_x as usize, new_y as usize);
            if maze[new_x][new_y] == Element::Wall || maze[new_x][new_y] == Element::Visiting || maze[new_x][new_y] == Element::Start {
                continue;
            }

            parents.insert((new_x,new_y), (i, j) );
            if maze[new_x][new_y] == Element::End {
                set_path(maze, &parents, (new_x,new_y));
                return true;
            }
            maze[new_x][new_y] = Element::Visiting;
            queue.add((new_x,new_y)).unwrap();
        }
    }
    false
}

fn set_path(maze: &mut Vec<Vec<Element>>, parents: &HashMap<(usize, usize), (usize, usize)>, c: (usize, usize)) {
    let (mut i,mut j) = c;
    if maze[i][j] != Element::End {
        panic!("not end");
    }
    (i,j) = parents[&c];
    let mut x = 1;
    while maze[i][j] != Element::Start {
        x+=1;
        // println!("maze[{i}][{j}]={:?}", maze[i][j]);
        maze[i][j] = Element::Path;
        (i,j) = parents[&(i,j)];
    }
    println!("shortest path length: {}", x);
    maze.iter_mut().for_each(|row| row.iter_mut().for_each(|e| if *e == Element::Visiting {*e = Element::Empty;}));
}

// comprehensive depth first search
fn find_all_paths(maze: &mut Vec<Vec<Element>>, start: usize) -> bool {
    let result = all_paths(maze, 0, start);
    maze[0][start] = Element::Start;
    result
}

fn all_paths(maze: &mut Vec<Vec<Element>>, i: usize, j: usize) -> bool {
    // println!("i: {i}, j: {j}, maze[i][j]: {:?}", maze[i][j]);
    if maze[i][j] == Element::End || maze[i][j] == Element::Path {
        return true;
    }
    if maze[i][j] == Element::Wall || maze[i][j] == Element::Visiting {
        return false;
    }
    maze[i][j] = Element::Visiting;

    let left = if j>0 {all_paths(maze, i, j-1)}  else {false};
    let right = if j<maze[0].len()-1 {all_paths(maze, i, j+1)} else {false};
    let up = if i>0 {all_paths(maze, i-1, j)} else {false};
    let down = if i<maze.len()-1 {all_paths(maze, i+1, j)} else {false};

    maze[i][j] = if left || right || up || down {Element::Path} else {Element::Empty};
    maze[i][j] == Element::Path
}

//depth first search
fn find_path(maze: &mut Vec<Vec<Element>>, start: usize) -> bool {
    let result = path(maze, 0, start);
    maze[0][start] = Element::Start;
    result
}

fn path(maze: &mut Vec<Vec<Element>>, i: usize, j: usize) -> bool {
    if maze[i][j] == Element::End {
        return true;
    }
    if maze[i][j] == Element::Wall || maze[i][j] == Element::Visiting || maze[i][j] == Element::Path {
        return false;
    }
    maze[i][j] = Element::Visiting;

    let result = if j>0 {path(maze, i, j-1)}  else {false} || if j<maze[0].len()-1 {path(maze, i, j+1)} else {false} ||
        if i>0 {path(maze, i-1, j)} else {false} || if i<maze.len()-1 {path(maze, i+1, j)} else {false};

    maze[i][j] = if result {Element::Path} else {Element::Empty};
    maze[i][j] == Element::Path
}