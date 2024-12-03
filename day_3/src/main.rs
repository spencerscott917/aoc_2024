use std::{fs::File, io::prelude::*};
use regex::Regex;

fn main() {
    let mut file = File::open("input/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read file contents to string");
    let re = Regex::new(r"mul\([0-9][0-9]?[0-9]?,[0-9][0-9]?[0-9]?\)").unwrap();
    for mat in re.find_iter(&contents) {
            println!("-> {}", mat.as_str());
    }
}
