use inquire::Select;

mod day1;

pub fn menu() {
    let days: Vec<u8> = (1..2).collect();
    let selected_day = Select::new("Which day would you like to run", days)
        .prompt()
        .expect("There was an error, please try again");

    match selected_day {
        1 => crate::year2024::day1::run(),

        _ => panic!("Unknown day selected / day not implemented yet"),
    }
}
