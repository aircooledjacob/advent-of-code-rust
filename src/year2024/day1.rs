use std::arch::x86_64::_addcarryx_u32;
use crate::common;

static INPUT_TXT: &str = r".\inputs\year2024\day1.txt";

pub fn run() {
    part1();
    part2();
}

fn part1() {
    let raw_lines = common::read_file_to_vec_of_strings(&INPUT_TXT);

    let (mut list_1, mut list_2): (Vec<u32>, Vec<u32>) = raw_lines
        .iter()
        .map(|line| line.split_once("   ").expect("input file malformed"))
        .map(|(str_1, str_2)| {
            (
                str_1
                    .parse::<u32>()
                    .expect("could not parse input to u32"),
                str_2
                    .parse::<u32>()
                    .expect("could not parse input to u32"),
            )
        })
        .unzip();
    
    list_1.sort();
    list_2.sort();
    
    let zipped_sorted_lists = list_1.iter().zip(list_2.iter());;
    let mut distance_sum:u64 = 0;
    
    for (list_item_1, list_item_2) in zipped_sorted_lists {
        let diff:u32 ;
        if list_item_1 > list_item_2 {
            diff = list_item_1 - list_item_2 ;
        } else if list_item_1 < list_item_2 {
            diff = list_item_2 - list_item_1 ;
        } else {
            diff = 0;
        }
        distance_sum += diff as u64;
    }

    println!("Part 1: total distances between the lists: {distance_sum}")
}

fn part2() {
    let raw_lines = common::read_file_to_vec_of_strings(&INPUT_TXT);

    //code to solve
    let answer = String::from("answer");

    println!("Part 2: answer: {answer}")
}
