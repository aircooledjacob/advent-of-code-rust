use crate::common;

static INPUT_TXT: &str = r".\inputs\year2022\day1.txt";

pub fn run() {
    part1();
    part2();
}

fn part1() {
    let raw_lines = common::read_file_to_vec_of_strings(&INPUT_TXT);

    let largest_calories_sum = find_largest_calories_sum(&raw_lines);

    println!(
        "Part 1: The elf carrying the most calories is carrying {largest_calories_sum} calories."
    )
}

fn part2() {
    let raw_lines = common::read_file_to_vec_of_strings(&INPUT_TXT);

    let mut calorie_sums_vec = calculate_calorie_sums(&raw_lines);

    calorie_sums_vec.sort();

    let sum_of_top_3_calorie_sums: u32 = calorie_sums_vec.iter().rev().take(3).sum();

    println!("Part 2: The sum of the top 3 calorie sums is {sum_of_top_3_calorie_sums}")
}

fn find_largest_calories_sum(input_vec: &Vec<String>) -> u32 {
    let mut current_sum: u32 = 0;
    let mut largest_sum: u32 = 0;

    for str_value in input_vec {
        match str_value.parse::<u32>() {
            Ok(int_value) => {
                current_sum += int_value;
            }
            Err(_) => {
                if current_sum > largest_sum {
                    largest_sum = current_sum;
                }
                current_sum = 0;
            }
        }
    }

    largest_sum
}

fn calculate_calorie_sums(input_vec: &Vec<String>) -> Vec<u32> {
    let mut current_sum: u32 = 0;
    let mut calorie_sums: Vec<u32> = vec![];

    for str_value in input_vec {
        match str_value.parse::<u32>() {
            Ok(int_value) => {
                current_sum += int_value;
            }
            Err(_) => {
                calorie_sums.push(current_sum);
                current_sum = 0;
            }
        }
    }

    calorie_sums
}
