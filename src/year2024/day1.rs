use crate::common;

static INPUT_TXT: &str = r".\inputs\year2024\day1.txt";

pub fn run() {
    part1();
    part2();
}

fn part1() {
    let raw_lines = common::read_file_to_vec_of_strings(&INPUT_TXT);

    let (list_1, list_2): (Vec<u16>, Vec<u16>) = raw_lines
        .iter()
        .map(|line| line.split_once("   ").expect("input file malformed"))
        .map(|(str_1, str_2)| {
            (
                str_1
                    .parse::<u16>()
                    .expect("alpha character found in input, numbers only"),
                str_2
                    .parse::<u16>()
                    .expect("alpha character found in input, numbers only"),
            )
        })
        .unzip();

    println!("Part 1: total distances between the lists: ")
}

fn part2() {
    let raw_lines = common::read_file_to_vec_of_strings(&INPUT_TXT);

    //code to solve
    let answer = String::from("answer");

    println!("Part 2: answer: {answer}")
}
