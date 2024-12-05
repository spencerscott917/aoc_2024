use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;

type Rules = HashMap<i32, HashSet<i32>>;
type Updates = Vec<Vec<i32>>;
enum Parsing {
    Rules,
    Updates
}

fn main() {
    let args: Vec<String>  = env::args().collect();
    let fname: String;
    if args.len() < 2 {
        fname = String::from("input/input.txt");
    }
    else {
        fname = args[1].clone();
    }
    let (rules, updates) = parse_input(&fname);
    part_1(&rules, &updates);
}

fn parse_input(fname: &str) -> (Rules, Updates) {
    let mut file = File::open(fname).unwrap();
    let mut contents = String::new();
    let mut rules = Rules::new();
    let mut updates = Updates::new();
    file.read_to_string(&mut contents).unwrap();
    
    let mut parsing = Parsing::Rules;
    for line in contents.lines() {
        match parsing {
            Parsing::Rules => {
                if line.is_empty() { 
                    parsing = Parsing::Updates;
                    continue;
                }
                let (key, val) = line.split_once('|')
                                     .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
                                     .unwrap();
                rules.entry(key)
                     .or_default()
                     .insert(val);
            }
            Parsing::Updates => {
                updates.push(line.split(',')
                                 .map(|x| x.parse::<i32>().unwrap())
                                 .collect()
                                 )
            }
        }
    }
    (rules, updates)
}

fn comp_vals(rules: &Rules, key: &i32, to_check: &i32) -> bool {
    let vals = rules.get(&key).unwrap();
    if key == to_check {
        return false 
    }
    else if vals.contains(&to_check) {
        return true
    }
    false  
}

fn part_1(rules: &Rules, updates: &Updates) {
    let res: i32 = updates.iter()
                          .filter(|page| {page.is_sorted_by(|key, to_check| comp_vals(rules, key, to_check))})
                          .map(|page| page.get(page.len() / 2).unwrap())
                          .sum();
    println!("{res}")
}
