use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

/* Advent of Code Day 1-1
* Goal: Get the top elf by Calories
* Step 1: Manually move the list to a .txt file.
* Step 2: Read Text-File into memory.
* Step 3: Split List based on New-lines.
* Step 4: Loop through the iterator summing for each elf, breaking whenever an empty line is hit.
* Step 5: Compare the record vs the new elf, if the new elf is greater then it replaces the old elf. (Ties are won by the first elf).
* Step 6: Print out the highest total calories.
*/

/* Advent of Code Day 1-2
* Goal: Get the top 3 elves by Calories
*
*
*/

// Main must return something so that "?" and "try!" work,
fn main() -> Result<(), std::io::Error> {
    println!("Advent of Code: Day 1-1!");
    let mut highest_total: u32 = 0;
    let mut current_total: u32 = 0;
    let mut elf_number: u32 = 1;
    let elf_total_vector: Vec<u32> = Vec::new();

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(val) = line {
            let i = match val.trim().parse() {
                Ok(i) => i,
                Err(_) => 0,
            };
            if i > 0 {
                current_total = current_total + i;
            } else {
                match current_total.cmp(&highest_total) {
                    Ordering::Less => {} // Do nothing if the current total is less than the highest total.
                    Ordering::Equal => {} // Does nothing if the new total equals the old total, favors the older value as a tie-breaker. "You don't set a record tying the old one".
                    Ordering::Greater => {
                        // Sets the highest total to be equal to the new current total.
                        highest_total = current_total;
                    }
                }
                // Increase the elf-count and reset the current total as we are moving onto the next elf.
                elf_number = elf_number + 1;
                current_total = 0;
            }

            println!("{}", val);
        }
    }

    println!(
        "Elves Checked: {}, Highest Total Calories: {}",
        elf_number, highest_total
    );
    Ok(())
}
