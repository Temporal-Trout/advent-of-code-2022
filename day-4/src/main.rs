// Advent of Code 2022 4-2

// Given a pair of sets A = {i..n}, B = {a..b} determine if one set overlaps between another
// Since the sets are ordinal and contiguous we only have to check the boundaries.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::usize;
// Main must return something so that "?" and "try!" work,
fn main() -> Result<(), std::io::Error> {
    println!("Advent of Code: Day 4-2!");
    let mut subset_count: usize = 0;

    // Reads the file
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // Splits the input into lines.
    for line_result in reader.lines() {
        // Extracts the line (String) from the result.
        // TODO: Error Handling
        if let Ok(val) = line_result {
            // Splits the input string into two sets.
            let mut right_set: Vec<usize> = Vec::new();
            let mut left_set: Vec<usize> = Vec::new();
            for (i, val) in val.split(',').enumerate() {
                if i == 0 {
                    for val in val.to_owned().split("-") {
                        left_set.push(val.parse().expect("Left-Set Could not Parse Value"));
                    }
                } else {
                    for val in val.to_owned().split("-") {
                        right_set.push(val.parse().expect("Right-Set Could not Parse Value"));
                    }
                    let mut check: bool = false;
                    if &left_set[0] >= &right_set[0] && &left_set[0] <= &right_set[1] {
                        subset_count = subset_count + 1;
                        check = true;
                    }
                    if &right_set[0] >= &left_set[0] && &right_set[0] <= &left_set[1] {
                        if check == false {
                            subset_count = subset_count + 1;
                        }
                    }
                }
            }
        }
    }
    println!("Subset Count: {}", subset_count);

    Ok(())
}
