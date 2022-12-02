use crate::utils::read_lines;

#[allow(dead_code)]
pub fn part1() {
    if let Ok(lines) = read_lines("./src/day2.txt") {
        let mut total: i32 = 0;
        for line in lines {
            if let Ok(line) = line {
                let split: Vec<&str> = line.split(" ").collect();
                let first = split[0].chars().next().unwrap();
                let first_num: i32;
                match first {
                    'A' => first_num = 1,
                    'B' => first_num = 2,
                    'C' => first_num = 3,
                    _ => first_num = 0,
                }
                let second = split[1].chars().next().unwrap();
                let second_num: i32;
                match second {
                    'X' => second_num = 1,
                    'Y' => second_num = 2,
                    'Z' => second_num = 3,
                    _ => second_num = 0,
                }
                total += second_num;
                if first_num == second_num {
                    total += 3
                } else if (first_num + 1 == second_num) || (first_num - 2 == second_num) {
                    total += 6
                }
            }
        }
        println!("{}", total)
    }
}

pub fn part2() {
    if let Ok(lines) = read_lines("./src/day2.txt") {
        let mut total: i32 = 0;
        for line in lines {
            if let Ok(line) = line {
                let split: Vec<&str> = line.split(" ").collect();
                let first = split[0].chars().next().unwrap();
                let first_num: i32;
                match first {
                    'A' => first_num = 1,
                    'B' => first_num = 2,
                    'C' => first_num = 3,
                    _ => first_num = 0,
                }
                let second = split[1].chars().next().unwrap();
                let mut second_num: i32;
                match second {
                    'X' => second_num = 0,
                    'Y' => second_num = 3,
                    'Z' => second_num = 6,
                    _ => second_num = 0,
                }
                total += second_num;
                second_num = second_num / 3 + 1;
                if second_num == 2 {
                    total += first_num
                } else if second_num == 3 {
                    if first_num == 3 {
                        total += 1
                    } else {
                        total += first_num + 1
                    }
                } else if second_num == 1 {
                    if first_num == 1 {
                        total += 3
                    } else {
                        total += first_num - 1
                    }
                }
            }
        }
        println!("{}", total)
    }
}
