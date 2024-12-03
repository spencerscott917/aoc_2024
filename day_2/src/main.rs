use std::fs::File;
use std::io::prelude::*;
use std::cmp::Ordering;

fn main() {
    let mut f = File::open("input/input.txt").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    let levels = contents.lines()
                         .map(|line| {line.split_whitespace()
                                         .map(|val| val.parse::<i32>().unwrap())
                                         .collect::<Vec<i32>>()})
                         .collect::<Vec<Vec<i32>>>();
    part_1(&levels);
    part_2(&levels);
}

fn part_1(levels: &Vec<Vec<i32>>) {
    let mut num_safe = 0;
    for row in levels {
        if row_is_valid(&row) {
            num_safe += 1
        }
    }
    println!("{num_safe}")
}

fn row_is_valid(row: &Vec<i32>) -> bool {
    match (row[0] - row[1]).cmp(&0) {
        Ordering::Less => {
            row.iter()
               .zip(row.iter().skip(1))
               .all(|(current, next)| ((current - next) <= -1) && ((current - next) >= -3))
        },
        Ordering::Greater => {     
            row.iter()
               .zip(row.iter().skip(1))
               .all(|(current, next)| ((current - next) >= 1) && ((current - next)) <= 3)
        },
        _ => false,
    }
}

fn part_2(levels: &Vec<Vec<i32>>) {
    let mut num_safe = 0;
    for row in levels {
        if row_is_valid(&row) {
            num_safe += 1
        }
        else {
            for i in 0..row.len(){
                let mut row_copy = (*row).clone();
                row_copy.remove(i);
                if row_is_valid(&row_copy) {
                    num_safe += 1;
                    break;
                }
            }
        }
    }
    println!("{num_safe}");
}
