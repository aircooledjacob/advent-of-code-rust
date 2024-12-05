use crate::common;
use regex::Regex;
use std::ops::RangeInclusive;

static INPUT_TXT: &str = r".\inputs\year2022\day4.txt";

pub fn run() {
    part1();
    part2();
}

fn part1() {
    let raw_lines = common::read_file_to_vec_of_strings(&INPUT_TXT);

    let mut fully_contained_count = 0;

    let re: Regex = Regex::new(r"[,\-]").unwrap();
    for line in raw_lines {
        let split_string: Vec<usize> = re
            .split(&line)
            .map(|string_value| string_value.parse::<usize>().unwrap())
            .collect();

        let range1 = split_string[0]..=split_string[1];
        let range2 = split_string[2]..=split_string[3];

        if ranges_can_fit_within_another(range1, range2) {
            fully_contained_count += 1;
        }
    }

    println!("number of section assignments that are fully overlapped: {fully_contained_count}");
}

fn part2() {
    let raw_lines = common::read_file_to_vec_of_strings(&INPUT_TXT);

    let re: Regex = Regex::new(r"[,\-]").unwrap();

    let mut overlapping_assignments = 0;

    for line in raw_lines {
        let split_string: Vec<usize> = re
            .split(&line)
            .map(|string_value| string_value.parse::<usize>().unwrap())
            .collect();

        let range1 = split_string[0]..=split_string[1];
        let range2 = split_string[2]..=split_string[3];

        if ranges_have_overlapping_values(range1, range2) {
            overlapping_assignments += 1;
        }
    }

    println!("number of section assignments that have overlapping assignments: {overlapping_assignments}");
}

fn ranges_can_fit_within_another(
    range1: RangeInclusive<usize>,
    range2: RangeInclusive<usize>,
) -> bool {
    if range1.contains(&range2.start()) && range1.contains(&range2.end()) {
        return true;
    }
    if range2.contains(&range1.start()) && range2.contains(&range1.end()) {
        return true;
    }

    false
}

fn ranges_have_overlapping_values(
    range1: RangeInclusive<usize>,
    range2: RangeInclusive<usize>,
) -> bool {
    for (e1, e2) in range1.clone().zip(range2.clone()) {
        if range1.contains(&e2) || range2.contains(&e1) {
            return true;
        }
    }

    false
}
