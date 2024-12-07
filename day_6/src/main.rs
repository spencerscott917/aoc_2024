use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;


fn main() {
    let mut file = File::open("input/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut grid = contents.lines()
                       .map(|line| line.chars().collect())
                       .collect::<Vec<Vec<char>>>();

    let mut state = Position::new();
    part_1(&mut grid, &mut state);
    part_2(&mut grid, &mut state);
}

fn part_1(grid: &mut Vec<Vec<char>>, state: &mut Position) {
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
            },
            'X' => {state.take_step();},
            _ => {}
        }
        grid[state.row_idx][state.col_idx] = 'X';
    }
    grid[state.row_idx][state.col_idx] = 'X';
    println!("{total}");
}

fn part_2(grid: &mut Vec<Vec<char>>, state: &mut Position) {
    state.back_to_start();
    let mut loops = 0;
    let mut xs = find_xs(&grid); 
    xs.remove(&(state.start[0] as u32, state.start[1] as u32));
    for (x_row, x_col) in xs { 
        let mut collisions = HashSet::<(usize, usize, Direction)>::new();
        grid[x_row as usize][x_col as usize] = '#';
        while let Some(next_step) = state.next_step(&grid) {
            match next_step {
                '#' => {
                    if collisions.contains(&(state.row_idx, state.col_idx, state.dir)) {
                        loops += 1;
                        break;
                    }
                    collisions.insert((state.row_idx, state.col_idx, state.dir));
                    state.dir = turn(state.dir);
                },
                '.' => {
                    state.take_step();
                },
                'X' => {state.take_step();},
                _ => {state.take_step();}
            }
        }
        grid[x_row as usize][x_col as usize] = 'X';
        state.back_to_start();

    }
    println!("{loops}");
}

fn find_xs(grid: &Vec<Vec<char>>) -> HashSet<(u32, u32)> {
    let mut xs = HashSet::new();
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &col_val) in row.iter().enumerate() {
            if col_val == 'X' {
                xs.insert((row_idx as u32, col_idx as u32));
            }
        }
    }
    xs
}


#[derive(Debug, Copy, Clone,Eq, Hash, PartialEq)]
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

#[derive(Debug, Copy, Clone)]
struct Position {
    row_idx: usize,
    col_idx: usize,
    dir: Direction,
    start: [usize; 2]
}

impl Position {
    fn new()->Position{
        Position{row_idx: 0, col_idx: 0, dir: Direction::Up, start: [0,0]}
    }
    fn find_starting_position(&mut self, grid: &Vec<Vec<char>>){
        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, &col_val) in row.iter().enumerate() {
                if col_val == '^' {
                    self.row_idx = row_idx;
                    self.col_idx = col_idx;
                    self.start = [row_idx, col_idx];
                }
            }
        }
    }

    fn next_step(self, grid: &Vec<Vec<char>>) -> Option<char>{
        match self.dir {
            Direction::Up => {
                if (self.row_idx as isize) - 1 < 0 {
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
                if (self.col_idx as isize) - 1 < 0 {
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

    fn back_to_start(&mut self) {
        self.row_idx = self.start[0];
        self.col_idx = self.start[1];
        self.dir = Direction::Up;
    }
}
