use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

fn main() {
    let now = Instant::now();
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

    part_1(&input);
    part_2(&input);
    let elapsed = now.elapsed();
    println!("Completed in {elapsed:.2?}");
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
                                 check_vals(&vals[1..], vals[0], *target, use_concat)
                          })
                          .map(|(target, _)| target)
                          .sum::<u64>();
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

fn check_vals(vals: &[u64], curr: u64, target: u64, with_concat: bool) -> bool {
    if curr > target {
        return false
    }
    match vals {
        [] => return curr == target,
        [n] =>  return Op::Mul.apply(curr, *n) == target ||
               (with_concat && Op::Concat.apply(curr, *n) == target ) ||
                Op::Add.apply(curr, *n) == target,
        [n, rest @ ..] => return check_vals(rest, Op::Add.apply(curr, *n), target, with_concat) ||
                                 check_vals(rest, Op::Mul.apply(curr, *n), target, with_concat) ||
                                 (with_concat && check_vals(rest, Op::Concat.apply(curr, *n), target, with_concat))

    }
}
