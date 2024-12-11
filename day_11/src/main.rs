use std::{fs::File,
          io::prelude::*,
          collections::HashMap
};

fn main() {
    let mut file = File::open("input/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut stones = HashMap::new();
    for stone in contents.split_whitespace() {
        stones.insert(stone.parse().unwrap(), 1);
    }
    println!("{}", part_1(&mut stones));
    println!("{}", part_2(&mut stones));

}

fn part_1(mut stones: &mut HashMap<u64, u64>) -> u64{
    for _ in 0..25 {
        blink(&mut stones);
    }
    count(&stones)
}

fn part_2(mut stones: &mut HashMap<u64, u64>) -> u64{
    for _ in 0..50 {
        blink(&mut stones);
    }
    count(&stones)
}

fn blink(stones: &mut HashMap<u64, u64>) { 
    let mut new_stones: HashMap<u64, u64> = HashMap::new();
    for (stone, count) in stones.into_iter() {
        if *stone == 0 {
            let new_count = *(new_stones.entry(1).or_default()) + *count;
            new_stones.insert(1, new_count);
        } else if (stone.ilog10() + 1) % 2 == 0 {
            let digit_break = 10_u64.pow((stone.ilog10() + 1)/2);
            let lhs = stone / digit_break;
            let rhs = stone % (lhs * digit_break);
            let new_count = *(new_stones.entry(lhs).or_default()) + *count;
            new_stones.insert(lhs, new_count);
            let new_count = *(new_stones.entry(rhs).or_default()) + *count;
            new_stones.insert(rhs, new_count);
        }
        else {
            let new_count = *(new_stones.entry(stone*2024).or_default()) + *count;
            new_stones.insert(stone * 2024, new_count);
        }
    }
    *stones = new_stones;
}

fn count(stones: &HashMap<u64, u64>) -> u64 {
    let mut res = 0;
    for (_, count) in stones.into_iter() {
        res += *count
    }
    res
}
