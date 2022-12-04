// Advent of Code - Day 3-2

/* Within each group of 3 elves (lines) the badge is the only item carried by all
* three elves. If the group item is 'B' all elves will have 'B' and at most two
* of the elves will be carrying another type of item.
*/

// Step 1: Parse in groups of 3 lines.
// Step 2: Check which item all 3 groups share.
// Step 3: Calculate the value of that item and sum it to the total.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::usize;

// Main must return something so that "?" and "try!" work,
fn main() -> Result<(), std::io::Error> {
    println!("Advent of Code: Day 3-2!");
    const ELF_GROUP_SIZE: usize = 3;
    let mut total_score: usize = 0;

    // Creates an array of size 52, fills with the function of "index - index".
    // There's probably a better way to 0 fill the array (Like a loop?) but this works.
    let mut duplicate_array: [usize; 52] = core::array::from_fn(|i| i - i);
    let mut seen_array: [bool; 52] = [false; 52];

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut current_count: usize = 0;

    // Splits the input into lines.
    for line_result in reader.lines() {
        // Extracts the line from the result, (No Error Handling).
        if let Ok(val) = line_result {
            // For each character in the line, if it's seen at least once set the seen array to true.
            for i in val.chars() {
                let n = adjust_char_to_digit(i);
                seen_array[n - 1] = true;
            }
            println!("");
            // Go through the seen array and increase the instance counter by 1 if the character was seen.
            let mut index: usize = 0;
            for i in seen_array {
                print!("{}", i);
                if i == true {
                    duplicate_array[index] = duplicate_array[index] + 1;
                    seen_array[index] = false;
                }
                index = index + 1;
            }
        }
        // How many lines in this group we have done.
        current_count = current_count + 1;

        // Once the corrent number of elves have been checked, reset the array and counter.
        if current_count >= ELF_GROUP_SIZE {
            println!("");
            let mut array_index: usize = 0;
            for i in duplicate_array {
                if i >= ELF_GROUP_SIZE {
                    total_score = total_score + (array_index + 1);
                }
                array_index = array_index + 1;
            }
            current_count = 0;
            duplicate_array = core::array::from_fn(|i| i - i);
        }
    }
    println!("Total Score: {}", total_score);

    Ok(())
}

// Takes a character, converts to a usize (Breaks on all sorts of values, just using for ASCII Alpha-Num)
fn adjust_char_to_digit(i: char) -> usize {
    let n: usize = i as usize;
    // Points are supposed to be: a = 1, z = 26, A = 27, Z = 52
    // if n is lowercase adjust score by 96 (a = 97)
    if n >= 96 {
        return n - 96;
    }
    // if n is uppercase adjust score by: 38 (A = 65)
    if n <= 90 {
        return n - 38;
    }
    // Something went wrong if this returns.
    return 0;
}
