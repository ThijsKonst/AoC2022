use std::{collections::HashSet, hash::Hash};

use crate::utils::read_lines;

#[test]
fn test_part1() {
    assert_eq!(part1_fn("./src/day6_test.txt"), 7);
}

#[test]
fn test_part2() {
    assert_eq!(part2_fn("./src/day6_test.txt"), 19);
}

fn part1_fn(path: &str) -> usize {
    let line = read_lines(path);
    let mut characters = line.char_indices();
    let (mut first, mut second, mut third) = (
        characters.next().unwrap().1,
        characters.next().unwrap().1,
        characters.next().unwrap().1,
    );
    for val in characters {
        if has_unique_elements([first, second, third, val.1]) {
            return val.0 + 1;
        }
        first = second;
        second = third;
        third = val.1;
    }
    return 100000;
}

fn part2_fn(path: &str) -> usize {
    let line = read_lines(path);
    let characters = line.char_indices();
    let mut first_thirteen = vec![];
    for val in characters {
        first_thirteen.push(val.1);
        if val.0 > 12 {
            if has_unique_elements(&first_thirteen) {
                return val.0 + 1;
            }
            first_thirteen.remove(0);
        }
    }
    return 100000;
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

pub fn day6() {
    println!("{}", part1_fn("./src/day6.txt"));
    println!("{}", part2_fn("./src/day6.txt"));
}
