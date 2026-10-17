
use crate::common;

static INPUT_TXT: &str = r"./inputs/year2022/day8.txt";

pub fn run() {
    part1();
    part2();
}

fn part1() {
    let raw_lines = common::read_file_to_vec_of_strings(&INPUT_TXT);

    //code to solve
    let tree_heights = parse_lines_to_2d_vec(raw_lines);

    // init visible datastructure
    let mut trees_visible: Vec<Vec<bool>> = Vec::new();

    for i in 0..tree_heights.len() {
        let mut visible_row: Vec<bool> = Vec::new();
        for _ in 0..tree_heights[i].len() {
            visible_row.push(false);
        }
        trees_visible.push(visible_row);
    }

    // mark all external trees visible
    for i in 0..trees_visible.len() {
        for j in 0..trees_visible[i].len() {
            if i == 0 || i == trees_visible.len()-1 {
                trees_visible[i][j] = true;
            }
            if j == 0 || j == tree_heights[i].len()-1 {
                trees_visible[i][j] = true;
            }
        }
    }

    // calculate if internal trees are visible
    for i in 1..trees_visible.len()-1 {
        for j in 1..trees_visible[i].len()-1 {

            // search up
            let mut max_height = 0;
            for k in 0..i {
                if tree_heights[k][j] > max_height {
                    max_height = tree_heights[k][j];
                }
            }
            if tree_heights[i][j] > max_height { trees_visible[i][j] = true; }

            // search down
            max_height = 0;
            for k in i+1..trees_visible.len() {
                if tree_heights[k][j] > max_height {
                    max_height = tree_heights[k][j];
                }
            }
            if tree_heights[i][j] > max_height { trees_visible[i][j] = true; }

            // search left
            max_height = 0;
            for k in 0..j {
                if tree_heights[i][k] > max_height {
                    max_height = tree_heights[i][k];
                }
            }
            if tree_heights[i][j] > max_height { trees_visible[i][j] = true; }

            // search right
            max_height = 0;
            for k in j+1..trees_visible.len() {
                if tree_heights[i][k] > max_height {
                    max_height = tree_heights[i][k];
                }
            }
            if tree_heights[i][j] > max_height { trees_visible[i][j] = true; }


        }
    }

    // sum visible trees
    let mut count = 0;
    for i in 0..trees_visible.len() {
        for j in 0..trees_visible[i].len() {
            if trees_visible[i][j] {
                count += 1;
            }
        }
    }

    println!("Part 1: answer: {count}")
}

fn part2() {
    let raw_lines = common::read_file_to_vec_of_strings(&INPUT_TXT);

    //code to solve
    let tree_heights = parse_lines_to_2d_vec(raw_lines);

    let mut scenic_scores: Vec<Vec<usize>> = Vec::new();

    for i in 0..tree_heights.len() {
        scenic_scores.push(Vec::new());
        for j in 0..tree_heights[i].len() {

            // search up
            let mut distance_up: usize = 0;
            for k in (0..i).rev() {
                distance_up += 1;
                if tree_heights[k][j] >= tree_heights[i][j] {
                    break
                }
            }

            // search down
            let mut distance_down: usize = 0;
            for k in (i+1)..tree_heights.len() {
                distance_down += 1;
                if tree_heights[k][j] >= tree_heights[i][j] {
                    break
                }
            }

            // search left
            let mut distance_left: usize = 0;
            for k in (0..j).rev() {
                distance_left += 1;
                if tree_heights[i][k] >= tree_heights[i][j] {
                    break
                }
            }

            // search right
            let mut distance_right: usize = 0;
            for k in (j+1)..tree_heights[i].len() {
                distance_right += 1;
                if tree_heights[i][k] >= tree_heights[i][j] {
                    break
                }
            }

            // calc + save scenic score
            let scenic_score = distance_up * distance_down * distance_left * distance_right;

            scenic_scores[i].push(scenic_score);

        }
    }


    // find highest value
    let mut highest_value: usize = 0;
    for i in 0..scenic_scores.len() {
        for j in 0..scenic_scores[i].len() {

            if scenic_scores[i][j] > highest_value {
                highest_value = scenic_scores[i][j];
            }

        }
    }

    println!("Part 2: answer: {highest_value}")
}


fn parse_lines_to_2d_vec(raw_lines: Vec<String>) -> Vec<Vec<usize>> {

    let mut vec_2d: Vec<Vec<usize>> = Vec::new();

    for line in raw_lines {

        vec_2d.push(line.chars().map(|x| x as usize).collect());

    }

    vec_2d

}

