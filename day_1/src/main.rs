use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let (mut col1, mut col2) = parse_input();
    part1(&mut col1, &mut col2);
    Ok(())
}

fn parse_input() -> (Vec<i32>, Vec<i32>) {
    let mut f = File::open("input/input.txt").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();
    for l in contents.lines() {
        let mut columns = l.split("   ");
        if let Some(col) = columns.next() {
            col1.push(col.parse::<i32>().unwrap());
        }
        if let Some(col) = columns.next() {
            col2.push(col.parse::<i32>().unwrap());
        }
    }
    (col1, col2)
}

fn part1(col1: &mut Vec<i32>, col2: &mut Vec<i32>) {
    let mut s: i32 = 0;
    col1.sort();
    col2.sort();
    for (x, y) in col1.iter().zip(col2.iter()) {
        s += (x - y).abs();
    }
    println!("Part 1: {s}");
}

