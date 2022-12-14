use crate::utils::read_lines;

fn part1_fn(path: &str) -> String {
    let mut init_part: Vec<String> = vec![];
    let mut init_state: Vec<Vec<char>> = vec![vec![]];
    let mut first_done: bool = false;
    for line in read_lines(path).split('\n').into_iter() {
        if first_done {
            if !line.is_empty() {
                init_state = mutate_state(line, init_state);
                for row in &init_state {
                    println!("{}", row.into_iter().collect::<String>());
                }
            }
        } else {
            if line.is_empty() {
                init_state = parse_init_state(&init_part);
                first_done = true
            } else {
                init_part.push(line.to_string());
            }
        }
    }

    let mut output: String = String::from("");
    for stack in init_state {
        output.push(stack.last().unwrap_or(&' ').to_owned())
    }
    return output;
}

fn parse_init_state(init_part: &Vec<String>) -> Vec<Vec<char>> {
    let stacks = init_part.last().unwrap();
    let n_stacks = stacks
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .len();
    let mut output: Vec<Vec<char>> = vec![vec![]; n_stacks];
    for i in 0..n_stacks {
        let index = (i + 1) * 4 - 3;
        for line in init_part.into_iter().rev() {
            if line.chars().nth(index).unwrap() != ' '
                && !stacks.contains(line.chars().nth(index).unwrap())
            {
                output[i].push(line.chars().nth(index).unwrap())
            }
        }
    }
    output
}

fn mutate_state(line: &str, state: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let amount: usize = line.split(" from ").collect::<Vec<&str>>()[0]
        .split(" ")
        .last()
        .unwrap()
        .parse()
        .expect("JOe ");
    let movement: Vec<&str> = line.split(" from ").collect::<Vec<&str>>()[1]
        .split(" to ")
        .collect::<Vec<&str>>();

    let index_1: usize = movement[0].parse().expect("JOe ");
    let index_2: usize = movement[1].parse().expect("JOe ");

    let mut new_state = state.to_owned();
    let mut value: Vec<char> = vec![];
    for _ in 0..amount {
        value.push(new_state[index_1 - 1].pop().unwrap());
    }
    new_state[index_2 - 1].append(&mut value.into_iter().rev().collect::<Vec<char>>());

    return new_state;
}

pub fn day5() {
    println!("Test: {}", part1_fn("./src/day5.txt"));
    //println!("Test: {}", part2_fn("./src/day5.txt"));
}
