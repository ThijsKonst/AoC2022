use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn day1() {
    let mut totals = BinaryHeap::new();
    if let Ok(lines) = read_lines("./src/day1.txt") {
        let mut total: u32 = 0;
        for line in lines {
            if let Ok(num) = line {
                if !num.is_empty() {
                    let current: u32 = num.parse().expect("Not a number!");
                    total += current;
                } else {
                    totals.push(total);
                    total = 0;
                }
            }
        }
    }
    let sum: u32 = totals.iter().take(3).sum();
    println!("{:?}", sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
