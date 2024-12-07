use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    part_1(&contents);
    part_2(&contents);
}

fn part_1(contents: &str) {
    let mut total = 0;
    for line in contents.lines() {
        if let Some((target, vals)) = line.split_once(":") {
            let target = target.parse::<u64>().unwrap();
            let vals = vals.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
            if recur_p1(&vals, 0, vals[0], target) {
                total += target
            }
        }
    }
    println!("{total}")
}

fn part_2(contents: &str) {
    let mut total = 0;
    for line in contents.lines() {
        if let Some((target, vals)) = line.split_once(":") {
            let target = target.parse::<u64>().unwrap();
            let vals = vals.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
            if recur_p2(&vals, 0, vals[0], target) {
                total += target;
            } 
        }
    }
    println!("{total}")
}

enum Op {
    Add,
    Mul,
    Concat
}

impl Op {
    fn apply(self, lhs: u64, rhs: u64) -> u64{
        match self {
            Op::Add => { lhs + rhs },
            Op::Mul => { lhs * rhs },
            Op::Concat => {
                lhs * 10_u64.pow(rhs.ilog10() + 1) + rhs}
    }
}
}

fn recur_p1(vals: &Vec<u64>, i: usize, curr: u64, target: u64) -> bool {
    if i == vals.len() - 1 {
        return curr == target
    }
    if recur_p1(vals, i+1, Op::Add.apply(curr, vals[i+1]), target) {
        return true
    }
    if recur_p1(vals, i+1, Op::Mul.apply(curr, vals[i+1]), target) {
        return true
    }
    false
}

fn recur_p2(vals: &Vec<u64>, i: usize, curr: u64, target: u64) -> bool {
    if i == vals.len() - 1 {
        return curr == target
    }
    if recur_p2(vals, i+1, Op::Add.apply(curr, vals[i+1]), target) {
        return true
    }
    if recur_p2(vals, i+1, Op::Mul.apply(curr, vals[i+1]), target) {
        return true
    }
    if recur_p2(vals, i+1, Op::Concat.apply(curr, vals[i+1]), target) {
        return true
    }
    false
}
