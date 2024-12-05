use inquire::Select;

mod day1;
mod day2;
mod day3;
mod day4;

pub fn menu() {
    let days: Vec<u8> = (1..5).collect();
    let selected_day = Select::new("Which day would you like to run", days)
        .prompt()
        .expect("There was an error, please try again");

    match selected_day {
        1 => day1::run(),
        2 => day2::run(),
        3 => day3::run(),
        4 => day4::run(),

        _ => panic!("Unknown day selected / day not implemented yet"),
    }
}
