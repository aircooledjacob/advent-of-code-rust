use std::collections::HashSet;
use crate::common;

static INPUT_TXT: &str = r"./inputs/year2022/day6.txt";

pub fn run() {
    part1();
    part2();
}

fn part1() {
    let raw_lines = common::read_file_to_vec_of_strings(&INPUT_TXT);

    let s = String::from(raw_lines.get(0).expect("input invalid"));

    let mut index: Option<usize> = None;

    //code to solve
    for i in 0..s.len()-3 {

        let unique_chars: HashSet<char> = s.chars().skip(i).take(4).collect();

        if unique_chars.len() == 4 {
            index = Some(i+4);
            break
        }

    }

    match index {
        Some(i) => {println!("Part 1: answer: {i}");}
        None => {println!("Error: No Match Found");}

    }

}

fn part2() {
    let raw_lines = common::read_file_to_vec_of_strings(&INPUT_TXT);

    let s = String::from(raw_lines.get(0).expect("input invalid"));

    let mut index: Option<usize> = None;

    //code to solve
    for i in 0..s.len()-13 {

        let unique_chars: HashSet<char> = s.chars().skip(i).take(14).collect();

        if unique_chars.len() == 14 {
            index = Some(i+14);
            break
        }

    }

    match index {
        Some(i) => {println!("Part 2: answer: {i}");}
        None => {println!("Error: No Match Found");}

    }

}