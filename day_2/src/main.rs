use std::fs::File;
use std::io::prelude::*;
use std::cmp::Ordering;

fn main() {
    let mut f = File::open("input/input.txt").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    part_1(contents.clone());
    part_2(contents.clone());
}

fn part_1(contents: String) {
    let mut num_safe = 0;
    for line in contents.lines() {
        let mut levels = Vec::new();
        let level_is_valid: bool;
        for level in line.split(" ") {
            levels.push(level.parse::<i32>().unwrap());
        }
        match (levels[0] - levels[1]).cmp(&0) {
             Ordering::Less => {
                 level_is_valid = levels.iter()
                                           .zip(levels.iter().skip(1))
                                           .all(|(current, next)| ((current - next) <= -1) && ((current - next) >= -3));
                 if level_is_valid {
                     num_safe += 1;
                 }
             },
             Ordering::Greater => {     
                 level_is_valid = levels.iter()
                                        .zip(levels.iter().skip(1))
                                        .all(|(current, next)| ((current - next) >= 1) && ((current - next)) <= 3);
                 if level_is_valid {
                     num_safe += 1;
                 }

    },
             _ => {num_safe += 0},
        }
    }
    println!("{num_safe}")
}

fn part_2(contents: String) {
    let mut num_safe = 0;
    for line in contents.lines() {
        let mut levels = Vec::new();
        let level_is_valid: bool;
        for level in line.split(" ") {
            levels.push(level.parse::<i32>().unwrap());
        }
    }

    println!("{num_safe}");

}
