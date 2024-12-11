use std::{fs::File,
          io::prelude::*
};

fn main() {
    let mut file = File::open("input/input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.truncate(contents.len() - 1);
    let mut disk_map: Vec<u64> = contents.chars()
                                         .map(|c| c.to_digit(10)
                                                   .unwrap() as u64)
                                         .collect();
    //println!("{disk_map:?}");
    println!("{}", part_1(&mut disk_map));
}

fn part_1(disk_map: &mut Vec<u64>) -> u64 {
    let mut tail_idx = disk_map.len() - 1;
    let mut head_idx = 1;
    let mut checksum = 0;
    let mut free_space_idx = disk_map[0];
    while tail_idx > head_idx {
        while disk_map[head_idx] == 0 {
            let next_idx = free_space_idx + disk_map[head_idx + 1];
            while free_space_idx < next_idx {
                let file_id = ((head_idx + 1)/ 2 ) as u64;
                checksum += free_space_idx * file_id;
                free_space_idx += 1;
            }
            head_idx += 2;
        }
        // need to stop from overcounting. Certianly a better way but this works.
        if head_idx >= tail_idx {
            break;
        }
        let file_id = (tail_idx / 2) as u64;
        if disk_map[tail_idx] == 0 {
            tail_idx -= 2;
            continue;
        }
        checksum += file_id * free_space_idx;
        disk_map[tail_idx] -= 1;
        disk_map[head_idx] -= 1;
        free_space_idx += 1;
    }
    checksum
}
