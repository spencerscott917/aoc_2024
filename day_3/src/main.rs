use std::{fs::File, io::prelude::*};
use regex::Regex;

fn main() {
    let mut file = File::open("input/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read file contents to string");
    part_1(&contents);
    part_2(&contents);
}

fn part_1(contents: &str) {
    let re = Regex::new(r"mul\((\d{1,3},\d{1,3})\)").unwrap();
    let mut sum = 0;
    for mat in re.captures_iter(&contents) {
        let vals: Vec<i32> = mat.get(1).unwrap().as_str().split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        sum += vals[0] * vals[1];
    }
    println!("{sum}")

}

fn part_2(contents: &str) {
    let re = Regex::new(r"mul\((\d{1,3},\d{1,3})\)|(do\(\)|don't\(\))").unwrap();
    let mut sum = 0;
    let mut enabled = true;
    for mat in re.captures_iter(&contents) {
        if let Some(flag) = mat.get(2) {
            match flag.as_str() {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ => {},
            }
            continue;
        }
        let vals: Vec<i32> = mat.get(1).unwrap().as_str().split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        if enabled {
            sum += vals[0] * vals[1];
        }
    }
    println!("{sum}")

}
