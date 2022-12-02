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
* Step 1: Refactor to store all the elf-totals into a vector
* Step 2: Sort said vector
* Step 3: Get N (3) largest values and sum them.
*/

// Main must return something so that "?" and "try!" work,
fn main() -> Result<(), std::io::Error> {
    println!("Advent of Code: Day 1-1!");
    let mut current_total: u32 = 0;
    let mut elf_number: usize = 1;
    let mut elf_total_vector: Vec<usize> = Vec::new();

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
                // Increase the elf-count and reset the current total as we are moving onto the next elf.
                elf_number = elf_number + 1;
                elf_total_vector.push(current_total.try_into().unwrap());
                current_total = 0;
            }
        }
    }

    elf_total_vector.sort_unstable();
    let slice = &elf_total_vector[(&elf_total_vector.len() - 3)..];
    let mut top_three: usize = 0;
    for u in slice {
        top_three = top_three + u;
        println!("{}", u);
    }
    println!("Top Three Summed: {}", top_three);

    Ok(())
}
