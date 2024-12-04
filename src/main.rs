use advent_of_code_rust::year2022;
use inquire::Select;

fn main() {
    let years: Vec<&str> = vec!["Exit", "2022", "2024"];
    let selected_year: &str = Select::new("Which year would you like to run?", years)
        .prompt()
        .expect("There was an error, please try again");

    let days: Vec<u8> = (1..25).collect();
    let selected_day = Select::new("Which day would you like to run", days)
        .prompt()
        .expect("There was an error, please try again");

    match selected_year {
        "2022" => match selected_day {
            1 => {
                year2022::day1::part1();
                year2022::day1::part2();
            }
            2 => {
                year2022::day2::part1();
                year2022::day2::part2();
            }
            3 => {
                year2022::day3::part1();
                year2022::day3::part2();
            }
            4 => {
                year2022::day4::part1();
                year2022::day4::part2();
            }
            _ => panic!("Unknown day selected / day not implemented yet"),
        },
        "2024" => {}
        "Exit" => (),
        _ => panic!("Unknown year selected"),
    }
}