use crate::utils::read_lines;

fn part1_fn(path: &str) -> u32 {
    let mut total: u32 = 0;
    for line in read_lines(path).split('\n').into_iter() {
        if !line.is_empty() {
            let first_range = extract(line.split(',').collect::<Vec<&str>>()[0]);
            let second_range = extract(line.split(',').collect::<Vec<&str>>()[1]);
            if full_overlap(first_range, second_range) {
                total += 1
            }
        }
    }
    return total;
}

fn part2_fn(path: &str) -> u32 {
    let mut total: u32 = 0;
    for line in read_lines(path).split('\n').into_iter() {
        if !line.is_empty() {
            let first_range = extract(line.split(',').collect::<Vec<&str>>()[0]);
            let second_range = extract(line.split(',').collect::<Vec<&str>>()[1]);
            if partial_overlap(first_range, second_range)
                || partial_overlap(second_range, first_range)
            {
                total += 1
            }
        }
    }
    return total;
}

fn extract(range: &str) -> (u32, u32) {
    return (
        range.split('-').collect::<Vec<&str>>()[0]
            .parse()
            .expect("Not a number!"),
        range.split('-').collect::<Vec<&str>>()[1]
            .parse()
            .expect("Not a number!"),
    );
}

fn full_overlap(first: (u32, u32), second: (u32, u32)) -> bool {
    return (first.0 >= second.0 && first.1 <= second.1)
        || (first.0 <= second.0 && first.1 >= second.1);
}

fn partial_overlap(first: (u32, u32), second: (u32, u32)) -> bool {
    return (second.0 <= first.0 && first.0 <= second.1)
        || (second.0 <= first.1 && first.1 <= second.1);
}

pub fn day4() {
    println!("Test: {}", part1_fn("./src/day4.txt"));
    println!("Test: {}", part2_fn("./src/day4.txt"));
}
