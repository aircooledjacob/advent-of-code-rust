use inquire::Select;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

pub fn menu() {
    let days: Vec<u8> = (1..8).collect();
    let selected_day = Select::new("Which day would you like to run", days)
        .prompt()
        .expect("There was an error, please try again");

    match selected_day {
        1 => day1::run(),
        2 => day2::run(),
        3 => day3::run(),
        4 => day4::run(),
        5 => day5::run(),
        6 => day6::run(),
        7 => day7::run(),

        _ => panic!("Unknown day selected / day not implemented yet"),
    }
}
