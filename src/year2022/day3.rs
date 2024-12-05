use crate::common;
use std::collections::HashSet;

static INPUT_TXT: &str = r".\inputs\year2022\day3.txt";

pub fn run() {
    part1();
    part2();
}

fn part1() {
    let raw_lines = common::read_file_to_vec_of_strings(&INPUT_TXT);

    //code to solve
    let mut sum_of_duplicate_priorities: u32 = 0;

    for rucksack in raw_lines {
        let (compartment_1, compartment_2) = rucksack.split_at(rucksack.len() / 2);

        let duplicates: HashSet<char> = compartment_2
            .chars()
            .filter(|item| compartment_1.contains(*item))
            .collect();

        let duplicate_priorities = parse_items_to_priorities(&duplicates);

        sum_of_duplicate_priorities += u32::from(duplicate_priorities.iter().sum::<u8>());
    }

    println!("{sum_of_duplicate_priorities}")
}

fn part2() {
    let raw_lines = common::read_file_to_vec_of_strings(&INPUT_TXT);

    let mut sum_of_badge_priorities: u32 = 0;

    for (i, _) in raw_lines.iter().enumerate().step_by(3) {
        let badge: HashSet<char> = raw_lines[i]
            .chars()
            .filter(|item| raw_lines[i + 1].contains(*item))
            .filter(|item| raw_lines[i + 2].contains(*item))
            .collect();

        // println!("{badge:?}");

        let badge_priorities = parse_items_to_priorities(&badge);

        sum_of_badge_priorities += u32::from(badge_priorities.iter().sum::<u8>());
    }

    println!("{sum_of_badge_priorities}")
}

fn parse_items_to_priorities(items: &HashSet<char>) -> HashSet<u8> {
    items
        .iter()
        .map(|item| {
            if *item as u8 >= b'a' {
                (*item as u8 - b'a') + 1
            } else {
                (*item as u8 - b'A') + 27
            }
        })
        .collect()
}
