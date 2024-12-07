use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

fn main() {
    let mut file = File::open("input/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let now = Instant::now();
    part_1(&contents);
    let elapsed = now.elapsed();
    println!("Completed Part 1 in {elapsed:.2?}");

    let now = Instant::now();
    part_2(&contents);
    let elapsed = now.elapsed();
    println!("Completed Part 1 in {elapsed:.2?}");
}

fn part_1(contents: &str) {
    solve(&contents, false);
}

fn part_2(contents: &str) {
    solve(&contents, true);
}

fn solve(contents: &str, use_concat: bool) {
    let total: u64 = contents.lines()
                             .map(|line| line.split_once(":").unwrap())
                             .map(|(target, vals)| (target.parse::<u64>().unwrap(), 
                                                    vals.split_whitespace()
                                                        .map(|val| val.parse::<u64>().unwrap())
                                                        .collect::<Vec<u64>>()))
                             .filter(|(target, vals)| {
                                 check_vals(&vals, 0, vals[0], *target, use_concat)
                             })
                             .map(|(target, _)| target)
                             .sum();
    println!("{total}");
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
            Op::Concat => { lhs * 10_u64.pow(rhs.ilog10() + 1) + rhs }
    }
}
}

fn check_vals(vals: &Vec<u64>, i: usize, curr: u64, target: u64, with_concat: bool) -> bool {
    if i == vals.len() - 1 {
        return curr == target
    }
    if check_vals(vals, i+1, Op::Add.apply(curr, vals[i+1]), target, with_concat) {
        return true
    }
    if check_vals(vals, i+1, Op::Mul.apply(curr, vals[i+1]), target, with_concat) {
        return true
    }
    if with_concat && check_vals(vals, i+1, Op::Concat.apply(curr, vals[i+1]), target, with_concat) {
        return true
    }
    false
}
