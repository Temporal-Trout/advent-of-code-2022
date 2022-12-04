// Advent of Code - Day 3-1

// Step 1: Split Each Line in half based on length.
// Step 2: Check through each half to see which character(s) overlap.
// Step 3: Get the "Priority Value" of that character and add it to a total.
// Step 4: Print out total.

use std::fs::File;
use std::io::{BufRead, BufReader};

// Main must return something so that "?" and "try!" work,
fn main() -> Result<(), std::io::Error> {
    println!("Advent of Code: Day 3-1!");
    let mut total_score: usize = 0;
    let mut wrong_item: Vec<char> = Vec::new();

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // Splits the input into lines.
    for line_result in reader.lines() {
        // Extracts the line from the result, (No Error Handling).
        if let Ok(val) = line_result {
            // Gets half the length, shows where the compartment splits
            let length_split = val.len() / 2;
            let mut right_compartment: Vec<char> = Vec::new();
            let mut left_compartment: Vec<char> = Vec::new();
            let mut current_count: usize = 0;

            for i in val.chars() {
                if current_count < length_split {
                    left_compartment.push(i);
                } else {
                    right_compartment.push(i);
                }
                current_count = current_count + 1;
            }
            // Assumes that there will only ever be 1 mismatched item in a rucksack.
            let mut already_matched: bool = false;
            print!("Left Compartment: ");
            for n in &left_compartment {
                for i in &right_compartment {
                    if n == i {
                        if already_matched == false {
                            wrong_item.push(*n);
                            already_matched = true;
                        }
                    }
                }
                print!("{}", n);
            }
            println!("");
            print!("Right Compartment: ");
            for n in &right_compartment {
                print!("{}", n);
            }
            println!("");
        }
    }
    // a = 1, z = 26, A = 27, Z = 52
    println!("");
    for i in wrong_item {
        let n: usize = i as usize;
        // if n is lowercase adjust score by 96 (a = 97)
        if n >= 96 {
            total_score = total_score + (n - 96);
        }
        // if n is uppercase adjust score by: 38 (A = 65)
        if n <= 90 {
            total_score = total_score + (n - 38);
        }
    }

    println!("");
    println!("---- Results ----");
    println!("Total Score: {}", total_score);

    Ok(())
}
