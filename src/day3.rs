use crate::utils::read_lines;

fn calculate_score(chr: char) -> u32 {
    return "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .find(chr)
        .unwrap() as u32
        + 1;
}

fn part1_fn(path: &str) -> u32 {
    let strings = read_lines(path);
    let mut total: u32 = 0;
    for line in strings.split("\n").into_iter() {
        if !line.is_empty() {
            let half_of_line = line.len() / 2;
            let first_half = &line[0..half_of_line];
            let second_half = &line[half_of_line..line.len()];
            for character in first_half.chars() {
                if second_half.contains(character) {
                    total += calculate_score(character);
                    break;
                }
            }
        }
    }
    return total;
}

fn part2_fn(path: &str) -> u32 {
    let strings = read_lines(path);
    let lines: Vec<&str> = strings.split("\n").collect();
    let mut total: u32 = 0;
    for i in 0..lines.len() / 3 {
        let line1 = lines[i * 3];
        let line2 = lines[i * 3 + 1];
        let line3 = lines[i * 3 + 2];
        for character in line1.chars() {
            if line2.contains(character) && line3.contains(character) {
                println!("{}", character);
                total += calculate_score(character);
                break;
            }
        }
    }

    return total;
}

pub fn day3() {
    println!("Test: {}", part1_fn("./src/day3.txt"));
    println!("Test: {}", part2_fn("./src/day3.txt"));
}
