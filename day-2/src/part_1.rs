use std::fs::File;
use std::io::{BufRead, BufReader};

/* Advent of Code - Day 2-Part 1
* Rock = A, B = Paper, C = Scissors
* My Move: X = Rock, Y = Paper, Z = Scissors
* Points: Rock = 1, Paper = 2, Scissors = 3
* Loss = 0, Draw = 3, Win = 6.
*/

// Goal: Calculate the score from the strategy Guide.

// Main must return something so that "?" and "try!" work,
fn main() -> Result<(), std::io::Error> {
    println!("Advent of Code: Day 2-1!");
    let mut total_score: i32 = 0;
    let mut my_move: char = ' ';
    let mut their_move: char = ' ';
    let mut count: usize = 0;

    let file = File::open("input-2.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(val) = line {
            let i = val.split_whitespace();
            for x in i {
                if (count % 2) == 0 {
                    their_move = x.chars().next().expect("String Empty, Error");
                    println!("Opponent Move: {}", their_move);
                } else {
                    my_move = x.chars().next().expect("String Invalid, Error");
                    // Get score based on my Move.
                    if my_move == 'X' {
                        total_score = total_score + 1;
                    }
                    if my_move == 'Y' {
                        total_score = total_score + 2;
                    }
                    if my_move == 'Z' {
                        total_score = total_score + 3;
                    }

                    // If there is a draw, gain 3 points
                    if their_move == 'A' && my_move == 'X' {
                        total_score = total_score + 3;
                    }
                    if their_move == 'B' && my_move == 'Y' {
                        total_score = total_score + 3;
                    }
                    if their_move == 'C' && my_move == 'Z' {
                        total_score = total_score + 3;
                    }

                    // If I win, gain 6 points.
                    if my_move == 'X' && their_move == 'C' {
                        total_score = total_score + 6;
                    }
                    if my_move == 'Y' && their_move == 'A' {
                        total_score = total_score + 6;
                    }
                    if my_move == 'Z' && their_move == 'B' {
                        total_score = total_score + 6;
                    }
                    // Otherwise gain 0 points.
                    println!("My Move: {}", my_move);
                }
                count = count + 1;
            }
        }
    }
    println!("Total Score: {}", total_score);

    Ok(())
}
