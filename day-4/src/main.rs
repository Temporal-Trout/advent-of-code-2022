// Advent of Code 2022 4-1

// Given a pair of sets A = {i..n}, B = {a..b} determine if one set is a subset of the other. That is: (A ⊂ B) || (B ⊂ A)
// Since the sets are ordinal and contiguous we only have to check the boundaries.

// Step 1: Split the String into left (A) and right (B) sets, using "," as a separator.
// Step 2: Get the numeric value of the start (i) and end (n) from each set: i.e: Extract i and n from "i-n"
// Step 3: Check if they both start at the same position (Ai == Bi) if so they must be the same set or 1 is a sub-set.
// Step 4: If one starts before the other check if the end of that one is larger than or equal to the end of the other, if it is there's another subset.
// Step 5: Total the number of times one is a subset of the other, print it out.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::usize;
// Main must return something so that "?" and "try!" work,
fn main() -> Result<(), std::io::Error> {
    println!("Advent of Code: Day 4-1!");
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
                    // If they both start at the same value they must either be the same set or one must be a sub-set.
                    if &left_set[0] == &right_set[0] {
                        subset_count = subset_count + 1;
                    }
                    // If the left is bigger (Starts further along) and also smaller (Ends Sooner) then it fits in the right and is a subset.
                    if &left_set[0] > &right_set[0] {
                        if &left_set[1] <= &right_set[1] {
                            subset_count = subset_count + 1;
                        }
                    }
                    // If the right starts further along and ends sooner it's a subset.
                    if &right_set[0] > &left_set[0] {
                        if &right_set[1] <= &left_set[1] {
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
