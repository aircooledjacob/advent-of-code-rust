use std::collections::{HashMap, HashSet};
use std::usize;
use crate::common;

static INPUT_TXT: &str = r"./inputs/year2022/day7.txt";

static TOTAL_DISK_SPACE: usize = 70000000;
static REQUIRED_DISK_SPACE: usize = 30000000;

pub fn run() {
    part1();
    part2();
}

fn part1() {
    let raw_lines = common::read_file_to_vec_of_strings(&INPUT_TXT);

    //code to solve
    // get unique dirs and files
    let (directories, files) = parse_directories_files(raw_lines);
    let directory_sizes = calc_directory_sizes(directories, files);

    //sum top 10 sizes
    let (_, mut sizes): (Vec<_>, Vec<usize>)= directory_sizes.into_iter().unzip();
    sizes.sort();


    let sum: usize = sizes.iter().filter(|s| &s < &&&100001).rev().sum();

    println!("Part 1: answer: {sum}")
}

fn part2() {
    let raw_lines = common::read_file_to_vec_of_strings(&INPUT_TXT);

    //code to solve
    // get unique dirs and files
    let (directories, files) = parse_directories_files(raw_lines);
    let directory_sizes = calc_directory_sizes(directories, files);

    let dir_size_map: HashMap<String, usize> = directory_sizes.clone().into_iter().collect();

    let used_disk_space = dir_size_map.get("/").unwrap();
    let free_disk_space = TOTAL_DISK_SPACE - used_disk_space;
    let required_disk_space_remaining = REQUIRED_DISK_SPACE - free_disk_space;

    let mut dir_sizes_vec: Vec<usize> = directory_sizes.into_iter().map(|(_, size)| size).collect();
    dir_sizes_vec.sort();

    let ans = dir_sizes_vec.into_iter().filter(|size| &size >= &&required_disk_space_remaining).nth(0).unwrap();

    println!("Part 2: answer: {ans}")
}

fn parse_directories_files(raw_lines: Vec<String>) -> (HashSet<String>, HashSet<(String, String, usize)>) {

    let mut cwd: String = String::from("/");
    let mut directories: HashSet<String> = HashSet::new();
    let mut files: HashSet<(String, String, usize)> = HashSet::new();

    for line in raw_lines {
        if line.starts_with("$ cd /") {
            cwd = String::from("/");

        } else if line.starts_with("$ cd ..") {
            if cwd == "/" { continue }
            cwd = cwd.rsplit_once("/").unwrap().0.to_string();
            if cwd == "" { cwd = String::from("/") }

        } else if line.starts_with("$ cd ") {
            if cwd == "/" {
                cwd = cwd + line.split_at(5).1
            } else {
                cwd = cwd + "/" + line.split_at(5).1
            }

        } else if line.starts_with("$ ls") { continue; }
        else if line.starts_with("dir ") { continue; }
        else {
            let filesize: usize = line.split_once(" ").unwrap().0.parse().unwrap();
            let filename: String = line.split_once(" ").unwrap().1.to_string();

            files.insert((cwd.clone(), filename, filesize));

        }

        directories.insert(cwd.clone());

    }

    (directories, files)

}

fn calc_directory_sizes(directories: HashSet<String>, files: HashSet<(String, String, usize)>) -> HashSet<(String, usize)> {


    // calculate directory sizes
    let mut directory_sizes: HashSet<(String, usize)> = HashSet::new();

    for dir in &directories {

        let mut dir_size:usize = 0;

        for (file_dir, _filename, filesize) in &files {

            if file_dir.starts_with(dir) {
                dir_size += filesize;
            }

        }

        directory_sizes.insert((dir.to_string(), dir_size));

    }

    directory_sizes

}

