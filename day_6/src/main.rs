use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Right,
    Left,
    Up,
    Down
}

fn turn(dir: Direction) -> Direction {
    // turn 90 degrees from dir
    match dir {
        Direction::Right => {return Direction::Down},
        Direction::Left => {return Direction::Up},
        Direction::Up => {return Direction::Right},
        Direction::Down => {return Direction::Left}
    }
}

fn main() {
    let mut file = File::open("input/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut grid = contents.lines()
                       .map(|line| line.chars().collect())
                       .collect::<Vec<Vec<char>>>();
    part_1(&mut grid);
}

#[derive(Debug, Copy, Clone)]
struct Position {
    row_idx: usize,
    col_idx: usize,
    dir: Direction
}

impl Position {
    fn new()->Position{
        Position{row_idx: 0, col_idx: 0, dir: Direction::Up}
    }
    fn find_starting_position(&mut self, grid: &Vec<Vec<char>>){
        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, &col_val) in row.iter().enumerate() {
                if col_val == '^' {
                    self.row_idx = row_idx;
                    self.col_idx = col_idx;
                }
            }
        }
    }

    fn next_step(self, grid: &Vec<Vec<char>>) -> Option<char>{
        match self.dir {
            Direction::Up => {
                if self.row_idx-1 >= grid.len() {
                    return None
                }
                Some(grid[self.row_idx-1][self.col_idx])
            },
            Direction::Down => {
                if self.row_idx + 1 >= grid.len() {
                    return None
                }
                Some(grid[self.row_idx+1][self.col_idx])
            },
            Direction::Left => {
                if self.col_idx - 1 >= grid[0].len() {
                    return None
                }
                Some(grid[self.row_idx][self.col_idx-1])
            },
            Direction::Right => {
                if self.col_idx + 1 >= grid[0].len() {
                    return None
                }
                Some(grid[self.row_idx][self.col_idx+1])
            }
        }
    }

    fn take_step(&mut self) {
        match self.dir {
            Direction::Up => {self.row_idx -= 1;},
            Direction::Down => {self.row_idx += 1;},
            Direction::Left => {self.col_idx -= 1;},
            Direction::Right =>{self.col_idx += 1;}
        }
    }
}


fn part_1(grid: &mut Vec<Vec<char>>) {
    let mut state = Position::new();
    state.find_starting_position(&grid);
    grid[state.row_idx][state.col_idx] = 'X';
    let mut total = 1;
    while let Some(val) = state.next_step(&grid) {
        match val {
            '#' => {
                state.dir = turn(state.dir);
            },
            '.' => {
                state.take_step();
                total += 1;
                grid[state.row_idx][state.col_idx] = 'X';
            },
            'X' => {state.take_step();},
            _ => {}
        }
    }
    println!("{total}");
}

