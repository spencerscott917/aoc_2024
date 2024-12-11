use std::{fs::File, 
          io::prelude::*,
          collections::{HashSet, HashMap}
};

type Grid = Vec<Vec<char>>;

fn main() {
    let grid = parse_input("input/input.txt");
    println!("{}" ,part_1(&grid));
    println!("{}" ,part_2(&grid));
}

fn part_1(grid: &Grid) -> usize {
    let mut nodes = HashMap::new();
    let mut antinodes = HashSet::new();
    let row_bound = grid.len();
    let col_bound = grid[0].len();
    for (row_idx, line) in grid.iter().enumerate() {
        for (col_idx, c) in line.iter().enumerate() {
                    match c {
                        '.' => {},
                        node => {
                            find_antinodes_p1(node, row_idx as isize, col_idx as isize, &row_bound, &col_bound, &mut nodes, &mut antinodes);
                        }
                    }
                }
    }
    antinodes.len()
}

fn part_2(grid: &Grid) -> usize {
    let mut nodes = HashMap::new();
    let mut antinodes = HashSet::new();
    let row_bound = grid.len();
    let col_bound = grid[0].len();
    for (row_idx, line) in grid.iter().enumerate() {
        for (col_idx, c) in line.iter().enumerate() {
                    match c {
                        '.' => {},
                        node => {
                            find_antinodes_p2(node, row_idx as isize, col_idx as isize, &row_bound, &col_bound, &mut nodes, &mut antinodes);
                        }
                    }
                }
    }
    antinodes.len()
}

fn find_antinodes_p1(node: &char, row_idx: isize, col_idx: isize, row_bound: &usize, col_bound: &usize, 
                  nodes: &mut HashMap<char, Vec<[isize; 2]>>, antinodes: &mut HashSet<[isize; 2]>) {
    let curr_pos = [row_idx, col_idx];
    if let Some(positions) = nodes.get_mut(node) {
        for prev_pos in positions.iter() {
            let diff = [curr_pos[0] - prev_pos[0], curr_pos[1] - prev_pos[1]];
            if let Some(antinode) = add2d_checked(&curr_pos, &diff, &row_bound, &col_bound) {
                antinodes.insert(antinode);
            }
            if let Some(antinode) = sub2d_checked(&prev_pos, &diff, &row_bound, &col_bound) {
                antinodes.insert(antinode);
            }
        }
        positions.push(curr_pos);
    } else {
        let positions = vec!([row_idx, col_idx]);
        nodes.insert(*node, positions);
    }
}

fn find_antinodes_p2(node: &char, row_idx: isize, col_idx: isize, row_bound: &usize, col_bound: &usize, 
                  nodes: &mut HashMap<char, Vec<[isize; 2]>>, antinodes: &mut HashSet<[isize; 2]>) {
    let curr_pos = [row_idx, col_idx];
    if let Some(positions) = nodes.get_mut(node) {
        for prev_pos in positions.iter() {
            match positions.len(){
                0 => {},
                1 => {
                    antinodes.insert(curr_pos);
                    antinodes.insert(*prev_pos);
                },
                _ => {antinodes.insert(curr_pos);}
            }
            let diff = [curr_pos[0] - prev_pos[0], curr_pos[1] - prev_pos[1]];
            let mut p = curr_pos;
            while let Some(antinode) = add2d_checked(&p, &diff, &row_bound, &col_bound) {
                antinodes.insert(antinode);
                p = antinode;
            }
            let mut p = *prev_pos;
            while let Some(antinode) = sub2d_checked(&p, &diff, &row_bound, &col_bound) {
                antinodes.insert(antinode);
                p = antinode;
            }
        }
        positions.push(curr_pos);
    } else {
        let positions = vec!([row_idx, col_idx]);
        nodes.insert(*node, positions);
    }
}

fn sub2d_checked(arr1: &[isize; 2], arr2: &[isize; 2], row_bound: &usize, col_bound: &usize) -> Option<[isize; 2]> {
    let res = [arr1[0] - arr2[0], arr1[1] - arr2[1]];
    if (res[0] < 0) || (res[0] >= *row_bound as isize) || (res[1] < 0) || (res[1] >= *col_bound as isize) {
        None
    } else {
        Some(res)
    }
}

fn add2d_checked(arr1: &[isize; 2], arr2: &[isize; 2], row_bound: &usize, col_bound: &usize) -> Option<[isize; 2]> {
    let res = [arr1[0] + arr2[0], arr1[1] + arr2[1]];
    if (res[0] < 0) || (res[0] >= *row_bound as isize) || (res[1] < 0) || (res[1] >= *col_bound as isize) {
        None
    } else {
        Some(res)
    }
}

fn parse_input(fname: &str) -> Grid {
    let mut file = File::open(fname).unwrap_or_else(|_| panic!("Could not open file {}", fname));
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read file contents to string");
    let res: Grid  = contents.lines()
                                       .map(|line| line.chars().collect())
                                       .collect();
    res
}
