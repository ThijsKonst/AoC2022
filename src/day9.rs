use crate::utils::read_lines;
use std::collections::HashSet;

#[test]
fn test_part1() {
    assert_eq!(part1_fn("./src/day9_test.txt"), 13);
}

#[test]
fn test_part2() {
    assert_eq!(part2_fn("./src/day9_test.txt"), 36);
}

fn part1_fn(path: &str) -> usize {
    let file = read_lines(path);
    let mut positions: HashSet<(i32, i32)> = HashSet::new();
    let mut head_pos: (i32, i32) = (0, 0);
    let mut tail_pos: (i32, i32) = (0, 0);
    for line in file.split('\n').into_iter() {
        if line.is_empty() {
            continue;
        }
        let direction = line.split(" ").next().unwrap();
        let amount = line.split(" ").last().unwrap().parse().expect("Joe");
        for _ in 0..amount {
            match direction {
                "R" => head_pos.1 += 1,
                "L" => head_pos.1 -= 1,
                "U" => head_pos.0 += 1,
                "D" => head_pos.0 -= 1,
                _ => println!("Should not happen`"),
            }
            tail_pos = move_tail(head_pos, tail_pos);
            positions.insert(tail_pos);
        }
    }
    return positions.len();
}

fn part2_fn(path: &str) -> usize {
    let file = read_lines(path);
    let mut positions: HashSet<(i32, i32)> = HashSet::new();
    let mut rope: Vec<(i32, i32)> = vec![(0, 0); 8];
    let mut head_pos: (i32, i32) = (0, 0);
    let mut tail_pos: (i32, i32) = (0, 0);
    for line in file.split('\n').into_iter() {
        if line.is_empty() {
            continue;
        }
        let direction = line.split(" ").next().unwrap();
        let amount = line.split(" ").last().unwrap().parse().expect("Joe");
        for _ in 0..amount {
            match direction {
                "R" => head_pos.1 += 1,
                "L" => head_pos.1 -= 1,
                "U" => head_pos.0 += 1,
                "D" => head_pos.0 -= 1,
                _ => println!("Should not happen`"),
            }
            let mut last_pos = head_pos;
            for x in 0..8 {
                rope[x] = move_tail(last_pos, rope[x]);
                last_pos = rope[x];
            }
            tail_pos = move_tail(last_pos, tail_pos);

            positions.insert(tail_pos);
        }
    }
    return positions.len();
}

fn move_tail(head_pos: (i32, i32), mut tail_pos: (i32, i32)) -> (i32, i32) {
    let vertical_diff: i32 = head_pos.0 - tail_pos.0;
    let horizontal_diff: i32 = head_pos.1 - tail_pos.1;

    if vertical_diff.abs() == 2 && horizontal_diff.abs() == 2 {
        tail_pos.1 += horizontal_diff / 2;
        tail_pos.0 += vertical_diff / 2;
    } else if vertical_diff == 2 {
        tail_pos.1 += horizontal_diff;
        tail_pos.0 += 1
    } else if vertical_diff == -2 {
        tail_pos.1 += horizontal_diff;
        tail_pos.0 -= 1
    } else if horizontal_diff == 2 {
        tail_pos.0 += vertical_diff;
        tail_pos.1 += 1
    } else if horizontal_diff == -2 {
        tail_pos.0 += vertical_diff;
        tail_pos.1 -= 1
    }

    return tail_pos;
}

pub fn day9() {
    println!("{}", part1_fn("./src/day9.txt"));
    println!("{}", part2_fn("./src/day9.txt"));
}
