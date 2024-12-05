use advent_of_code_rust::year2022;
use inquire::Select;

fn main() {
    
    let years: Vec<&str> = vec!["Exit", "2022", "2024"];
    let mut user_requested_exit = false;
    
    while !user_requested_exit {
        let selected_year: &str = Select::new("Which year would you like to run?", years.clone())
            .prompt()
            .expect("There was an error, please try again");

        match selected_year {
            "2022" => year2022::menu(),
            "2024" => (),
            "Exit" => user_requested_exit = true,
            _ => panic!("Unknown year selected"),
        }
    }

}
