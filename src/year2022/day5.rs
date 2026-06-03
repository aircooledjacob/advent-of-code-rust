use crate::common;
use regex::Regex;

static INPUT_TXT: &str = r"./inputs/year2022/day5.txt";

pub fn run() {
    part1();
    part2();
}

fn calc_number_of_stacks(raw_lines: &Vec<String>) -> Option<usize> {
    let mut number_of_stacks: Option<usize> = None;

    let mut bottom_crate_stack_line_index: Option<usize> = None;

    // find bottom row of stacks (higher rows will be unreliable to calculate total num of stacks)
    for (i, line) in raw_lines.iter().enumerate() {
        // go to stack numbers (under
        if line.starts_with("[") {
            continue;
        };

        bottom_crate_stack_line_index = Some(i - 1);

        break;
    }

    //count number of stacks in bottom row
    match bottom_crate_stack_line_index {
        None => {}
        Some(bottom_crate_stack_line_index) => {
            number_of_stacks = Some(
                raw_lines[bottom_crate_stack_line_index]
                    .matches("[")
                    .collect::<Vec<&str>>()
                    .len(),
            );
        }
    }

    number_of_stacks
}

fn parse_crate_stacks(raw_lines: &Vec<String>) -> Vec<Vec<char>> {
    let number_of_stacks = calc_number_of_stacks(&raw_lines).expect("malformed input");

    let mut crate_stacks: Vec<Vec<char>> = vec![Vec::new(); number_of_stacks];

    for line in raw_lines {
        if !line.starts_with("[") {
            break;
        };

        let mut stack_index: usize = 1;

        for i in 0..number_of_stacks {
            crate_stacks[i].insert(0, line.chars().nth(stack_index).unwrap_or(' '));

            stack_index += 4
        }
    }

    // remove blanks:
    let mut filtered_crate_stacks: Vec<Vec<char>> = vec![Vec::new(); number_of_stacks];
    for (i, stack) in crate_stacks.into_iter().enumerate() {
        filtered_crate_stacks[i] = stack.into_iter().filter(|c| *c != ' ').collect();
    }

    filtered_crate_stacks
}

fn parse_instructions(raw_lines: &Vec<String>) -> Vec<(usize, usize, usize)> {
    let mut instructions: Vec<(usize, usize, usize)> = Vec::new();

    let re = Regex::new(
        r"^move (?<number_to_move>\d+) from (?<starting_stack>\d+) to (?<ending_stack>\d+)$",
    )
    .unwrap();

    for line in raw_lines {
        let captures = re.captures(line);

        match captures {
            None => continue,
            Some(captures) => instructions.push((
                captures
                    .name("number_to_move")
                    .expect("Malformed input")
                    .as_str()
                    .parse()
                    .expect("Malformed input"),
                captures
                    .name("starting_stack")
                    .expect("Malformed input")
                    .as_str()
                    .parse()
                    .expect("Malformed input"),
                captures
                    .name("ending_stack")
                    .expect("Malformed input")
                    .as_str()
                    .parse()
                    .expect("Malformed input"),
            )),
        }
    }

    instructions
}

fn follow_instructions(
    instructions: Vec<(usize, usize, usize)>,
    mut crate_stacks: Vec<Vec<char>>,
) -> Vec<Vec<char>> {
    for (number_of_crates, from_stack, to_stack) in instructions {
        for _ in 0..number_of_crates {
            let temp_crate = crate_stacks[from_stack - 1].pop().unwrap();
            crate_stacks[to_stack - 1].push(temp_crate);
        }
    }

    crate_stacks
}

fn check_crates_on_top(crate_stacks: Vec<Vec<char>>) -> Vec<char> {
    let mut top_crates: Vec<char> = Vec::new();

    for mut stack in crate_stacks {
        top_crates.push(stack.pop().unwrap());
    }

    top_crates
}

fn part1() {
    let raw_lines = common::read_file_to_vec_of_strings(&INPUT_TXT);

    let crate_stacks = parse_crate_stacks(&raw_lines);

    let instructions = parse_instructions(&raw_lines);

    let rearranged_crate_stacks = follow_instructions(instructions, crate_stacks);

    let answer: String = check_crates_on_top(rearranged_crate_stacks)
        .iter()
        .collect();

    println!("Part 1: answer: {answer}")
}

fn part2() {
    let raw_lines = common::read_file_to_vec_of_strings(&INPUT_TXT);

    //code to solve
    let answer = String::from("answer");

    println!("Part 2: answer: {answer}")
}
