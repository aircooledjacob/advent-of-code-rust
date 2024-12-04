use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file_to_vec_of_strings(input_filename: &&str) -> Vec<String> {
    let file: File = File::open(input_filename).expect("Unable to open input file");
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = vec![];
    for line in reader.lines() {
        lines.push(line.expect("error reading file"))
    }

    lines
}
