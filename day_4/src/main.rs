use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mat: Vec<Vec<char>> = contents.lines().map(|line| line.chars().collect()).collect();
    part_1(&mat);
    part_2(&mat);
}

fn part_1(chars: &Vec<Vec<char>>) {
    fn count_words(chars: &Vec<Vec<char>>, row: isize, col: isize) -> i32 {
        let to_match = "XMAS";
        let mut count = 0;
        for row_dir in -1..=1 {
            for col_dir in -1..=1 {
                let word = get_word(&chars, row_dir, col_dir, row, col); 
                if word == to_match {count += 1}
            }
        }
        count
    }

    let row_len = chars[0].len();
    let col_len = chars.len();
    let mut count = 0;
    for i in 0..row_len {
        for j in 0..col_len {
            match chars[i][j]{
                'X' => {count += count_words(&chars, i as isize, j as isize)},
                _ => {},
            }
        }
    }
    println!("{count}")
}


fn get_word(chars: &Vec<Vec<char>>, row_dir: isize, col_dir: isize, row: isize, col: isize) -> String {
    let mut word = String::new();
    let (mut curr_row, mut curr_col) = (row, col);
    for _  in 0..4{
        word.push(chars[curr_row as usize][curr_col as usize]);
        curr_row += row_dir;
        curr_col += col_dir;
        if (curr_row < 0) || (curr_col < 0) || (curr_row >= chars.len() as isize) || (curr_col >= chars[0].len() as isize) { break }
    }
    word
}

fn part_2(chars: &Vec<Vec<char>>) {
    let row_len = chars[0].len();
    let col_len = chars.len();
    let mut count = 0;
    for i in 0..row_len {
        for j in 0..col_len {
            if (chars[i][j] == 'A') & (is_valid_x(&chars, i as isize, j as isize)) {
                count += 1
            }
        }
    }
    println!("{count}") 
}

fn is_valid_x(chars: &Vec<Vec<char>>, row: isize, col: isize) -> bool {
    if (row - 1 < 0) || (col - 1 < 0) || (row + 1 >= chars.len() as isize) || (col + 1 >= chars[0].len() as isize) { return false}
    match chars[(row-1) as usize][(col-1) as usize] {
        'M' => {
            if chars[(row+1) as usize][(col+1) as usize] != 'S' {return false}
        },
        'S' => {
            if chars[(row+1) as usize][(col+1) as usize] != 'M' {return false}
        },
        _ => return false
    }
    match chars[(row-1) as usize][(col+1) as usize] {
        'M' => {
            if chars[(row+1) as usize][(col-1) as usize] != 'S' {return false}
        },
        'S' => {
            if chars[(row+1) as usize][(col-1) as usize] != 'M' {return false}
        },
        _ => return false
    }
    return true
}

