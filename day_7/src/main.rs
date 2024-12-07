use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

fn main() {
    let mut file = File::open("input/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let input = contents.lines()
                        .map(|line| line.split_once(":").unwrap())
                        .map(|(target, vals)| (target.parse::<u64>().unwrap(), 
                                               vals.split_whitespace()
                                                   .map(|val| val.parse::<u64>().unwrap())
                                                   .collect::<Vec<u64>>()))
                        .collect();

    let now = Instant::now();
    part_1(&input);
    let elapsed = now.elapsed();
    println!("Completed Part 1 in {elapsed:.2?}");

    let now = Instant::now();
    part_2(&input);
    let elapsed = now.elapsed();
    println!("Completed Part 1 in {elapsed:.2?}");
}

fn part_1(input: &Vec<(u64, Vec<u64>)>) {
    solve(&input, false);
}

fn part_2(input: &Vec<(u64, Vec<u64>)>) {
    solve(&input, true);
}

fn solve(input: &Vec<(u64, Vec<u64>)>, use_concat: bool) {
    let total: u64 = input.iter()
                          .filter(|(target, vals)| {
                                 check_vals(&vals, 0, vals[0], target, use_concat)
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

fn check_vals(vals: &Vec<u64>, i: usize, curr: u64, target: &u64, with_concat: bool) -> bool {
    if i == vals.len() - 1 {
        return curr == *target
    }
    if curr > *target {
        return false
    }
    let next = vals[i+1];
    if with_concat && check_vals(vals, i+1, Op::Concat.apply(curr, next), &target, with_concat) {
        return true
    }
    if check_vals(vals, i+1, Op::Mul.apply(curr, next), &target, with_concat) {
        return true
    }
    if check_vals(vals, i+1, Op::Add.apply(curr, next), &target, with_concat) {
        return true
    }
    
    false
}
