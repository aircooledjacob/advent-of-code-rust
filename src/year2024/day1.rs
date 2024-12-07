use crate::common;
use std::collections::HashMap;

static INPUT_TXT: &str = r".\inputs\year2024\day1.txt";

pub fn run() {
    part1();
    part2();
}

fn part1() {
    let raw_lines = common::read_file_to_vec_of_strings(&INPUT_TXT);

    let (mut list_1, mut list_2) = parse_input_lists(raw_lines);

    list_1.sort();
    list_2.sort();

    let zipped_sorted_lists = list_1.iter().zip(list_2.iter());
    let mut distance_sum: u64 = 0;

    for (list_item_1, list_item_2) in zipped_sorted_lists {
        let diff: u32;
        if list_item_1 > list_item_2 {
            diff = list_item_1 - list_item_2;
        } else if list_item_1 < list_item_2 {
            diff = list_item_2 - list_item_1;
        } else {
            diff = 0;
        }
        distance_sum += diff as u64;
    }

    println!("Part 1: total distances between the lists: {distance_sum}")
}

fn part2() {
    let raw_lines = common::read_file_to_vec_of_strings(&INPUT_TXT);

    let (mut list_1, mut list_2) = parse_input_lists(raw_lines);

    list_1.sort();
    list_2.sort();

    let frequency_map = list_2
        .iter()
        .copied()
        .fold(HashMap::new(), |mut map, val| {
        map.entry(val)
            .and_modify(| frq| *frq += 1)
            .or_insert(1);
        map
    });
    
    let mut similarity_score_sum:u32 = 0;
    for num in list_1 {
        match frequency_map.get(&num) {
            Some(frequency) => {
                similarity_score_sum += num * frequency;
            }
            None => ()
        }
    }

    println!("Part 2: similarity score: {similarity_score_sum}")
}

fn parse_input_lists(raw_lines: Vec<String>) -> (Vec<u32>, Vec<u32>) {
    let (list_1, list_2): (Vec<u32>, Vec<u32>) = raw_lines
        .iter()
        .map(|line| line.split_once("   ").expect("input file malformed"))
        .map(|(str_1, str_2)| {
            (
                str_1.parse::<u32>().expect("could not parse input to u32"),
                str_2.parse::<u32>().expect("could not parse input to u32"),
            )
        })
        .unzip();
    
    (list_1, list_2)
    
}