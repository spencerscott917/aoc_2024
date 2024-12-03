use std::{fs::File, io::prelude::*};
use regex::Regex;

fn main() {
    let mut file = File::open("input/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read file contents to string");
    let re1 = Regex::new(r"mul\([0-9][0-9]?[0-9]?,[0-9][0-9]?[0-9]?\)").unwrap();
    let re2 = Regex::new(r"[0-9][0-9]?[0-9]?,[0-9][0-9]?[0-9]?").unwrap();
    let mut sum = 0;
    for mat1 in re1.find_iter(&contents) {
        for mat2 in re2.find_iter(mat1.as_str()) {
            let vals: Vec<i32> = mat2.as_str().split(",").map(|x| x.parse::<i32>().unwrap()).collect();
            sum += vals[0] * vals[1];
        }
    }
    println!("{sum}")
}
