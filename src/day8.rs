use crate::utils::read_lines;

#[test]
fn test_part1() {
    assert_eq!(part1_fn("./src/day8_test.txt"), 21);
}

#[test]
fn test_part2() {
    assert_eq!(part2_fn("./src/day8_test.txt"), 8);
}

fn part1_fn(path: &str) -> u32 {
    let file = read_lines(path);
    let grid = parse_file(file);
    let total = calculate_total(grid);
    return total;
}

fn part2_fn(path: &str) -> u32 {
    let file = read_lines(path);
    let grid = parse_file(file);
    let score = calculate_total_senic_score(grid);
    return score;
}

fn parse_file(file: String) -> Vec<Vec<u32>> {
    let mut new_file = vec![];
    for line in file.split('\n').into_iter() {
        if line.is_empty() {
            continue;
        }
        let mut new_line = vec![];
        for char in line.chars() {
            new_line.push(char.to_digit(10).unwrap());
        }
        new_file.push(new_line);
    }
    new_file
}

fn calculate_total(grid: Vec<Vec<u32>>) -> u32 {
    let mut total = 0;
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            let up_down = grid[0..grid.len()]
                .iter()
                .map(|x| x[j])
                .collect::<Vec<u32>>();

            if i == 0 || i == grid.len() - 1 || j == 0 || j == grid[i].len() - 1 {
                total += 1
            } else if &grid[i][j] > grid[i][0..j].iter().max().unwrap()
                || &grid[i][j] > grid[i][j + 1..grid[i].len()].iter().max().unwrap()
                || &grid[i][j] > up_down[0..i].iter().max().unwrap()
                || &grid[i][j] > up_down[i + 1..grid.len()].iter().max().unwrap()
            {
                total += 1
            }
        }
    }
    total
}

fn calculate_total_senic_score(grid: Vec<Vec<u32>>) -> u32 {
    let mut max = 0;
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            let up_down = grid[0..grid.len()]
                .iter()
                .map(|x| x[j])
                .collect::<Vec<u32>>();
            let score_left = calculate_senic_score(grid[i][0..j].to_vec(), grid[i][j], true);
            let score_right =
                calculate_senic_score(grid[i][j + 1..grid[i].len()].to_vec(), grid[i][j], false);
            let score_up = calculate_senic_score(up_down[0..i].to_vec(), grid[i][j], true);
            let score_down =
                calculate_senic_score(up_down[i + 1..grid.len()].to_vec(), grid[i][j], false);
            if score_left * score_right * score_up * score_down > max {
                max = score_left * score_right * score_up * score_down;
            }
        }
    }
    max
}

fn calculate_senic_score(mut row: Vec<u32>, curr: u32, reverse: bool) -> u32 {
    let mut total = 0;
    if reverse {
        row = row.into_iter().rev().collect()
    }
    for x in row {
        if x < curr {
            total += 1;
        }
        if x >= curr {
            total += 1;
            return total;
        }
    }
    total
}

pub fn day8() {
    println!("{}", part1_fn("./src/day8.txt"));
    println!("{}", part2_fn("./src/day8.txt"));
}
