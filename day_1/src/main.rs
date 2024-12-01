use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut f = File::open("input/input.txt")?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();
    for l in contents.lines() {
        let mut columns = l.split("   ");
        if let Some(col) = columns.next() {
            col1.push(col.parse::<i64>().unwrap());
        }
        if let Some(col) = columns.next() {
            col2.push(col.parse::<i64>().unwrap());
        }
    }
    let mut s: i64 = 0;
    col1.sort();
    col2.sort();
    for (x, y) in col1.iter().zip(col2.iter()) {
        s += (x - y).abs();
    }
    println!("{s}");
    Ok(())
}
