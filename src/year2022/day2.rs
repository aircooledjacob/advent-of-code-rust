use crate::common;

static INPUT_TXT: &str = r".\inputs\year2022\day2.txt";

pub fn part1() {
    let raw_lines = common::read_file_to_vec_of_strings(&INPUT_TXT);

    let tuple_vec = parse_lines_to_char_tuples(&raw_lines);

    let score = calculate_score(&tuple_vec);

    println!("Your final score is {score}")
}

pub fn part2() {
    let raw_lines = common::read_file_to_vec_of_strings(&INPUT_TXT);

    let input_tuple_vec = parse_lines_to_char_tuples(&raw_lines);

    let converted_tuple_vec: Vec<(char, char)> = input_tuple_vec
        .iter()
        .map(
            |(opponent_choice, outcome)| match (opponent_choice, outcome) {
                ('A', 'X') => ('A', 'Z'),
                ('A', 'Y') => ('A', 'X'),
                ('A', 'Z') => ('A', 'Y'),
                ('B', 'X') => ('B', 'X'),
                ('B', 'Y') => ('B', 'Y'),
                ('B', 'Z') => ('B', 'Z'),
                ('C', 'X') => ('C', 'Y'),
                ('C', 'Y') => ('C', 'Z'),
                ('C', 'Z') => ('C', 'X'),
                _ => panic!("Unexpected char in file."),
            },
        )
        .collect();

    let score = calculate_score(&converted_tuple_vec);

    println!("Your final score is {score}")
}

fn parse_lines_to_char_tuples(raw_lines_as_string: &Vec<String>) -> Vec<(char, char)> {
    raw_lines_as_string
        .iter()
        .map(|line| {
            let split_line = line.split_once(' ').expect("Malformed input");
            (
                split_line.0.chars().collect::<Vec<char>>()[0],
                split_line.1.chars().collect::<Vec<char>>()[0],
            )
        })
        .collect()
}

fn calculate_score(moves_tuple_vec: &Vec<(char, char)>) -> u32 {
    let mut score: u32 = 0;
    for (opponent_choice, your_choice) in moves_tuple_vec {
        //increase score for outcome
        match (opponent_choice, your_choice) {
            ('A', 'X') => score += 3,
            ('A', 'Y') => score += 6,
            ('A', 'Z') => score += 0,
            ('B', 'X') => score += 0,
            ('B', 'Y') => score += 3,
            ('B', 'Z') => score += 6,
            ('C', 'X') => score += 6,
            ('C', 'Y') => score += 0,
            ('C', 'Z') => score += 3,
            _ => panic!("Unexpected value in first column, acceptable values are: A, B, C"),
        }

        //increase score for choice
        match your_choice {
            'X' => score += 1,
            'Y' => score += 2,
            'Z' => score += 3,
            _ => panic!("unexpected value in second column, acceptable values are: X, Y, Z"),
        }
    }
    score
}
