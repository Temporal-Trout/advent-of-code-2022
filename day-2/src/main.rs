use std::fs::File;
use std::io::{BufRead, BufReader};

// Main must return something so that "?" and "try!" work,
fn main() -> Result<(), std::io::Error> {
    println!("Advent of Code: Day 2-1!");
    let mut total_score: i32 = 0;
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
                    /* A = Rock, B = Paper, C = Scissors
                     * My Move: X = Lose, Y = Draw, Z = Win
                     * Points: Rock = 1, Paper = 2, Scissors = 3
                     * Loss = 0, Draw = 3, Win = 6.
                     */
                    let my_move = x.chars().next().expect("String Invalid, Error");
                    // Get score based on my Move.

                    // TODO: Clean this ugly thing up.
                    // ---- VS Rock
                    // Lose against rock = Scissors (3 Points)
                    if my_move == 'X' && their_move == 'A' {
                        total_score = total_score + 3;
                    }
                    // Draw against rock = Rock (1 Point) + Draw (3 Points)
                    if my_move == 'Y' && their_move == 'A' {
                        total_score = total_score + 4;
                    }
                    // Win against rock = Paper (2 Points) + Win (6 Points)
                    if my_move == 'Z' && their_move == 'A' {
                        total_score = total_score + 8;
                    }
                    // ---- VS Paper
                    // Lose against paper = Rock (1 Points)
                    if my_move == 'X' && their_move == 'B' {
                        total_score = total_score + 1;
                    }
                    // Draw against Paper = Paper (2 Point) + Draw (3 Points)
                    if my_move == 'Y' && their_move == 'B' {
                        total_score = total_score + 5;
                    }
                    // Win against Paper = Scissors (3 Points) + Win (6 Points)
                    if my_move == 'Z' && their_move == 'B' {
                        total_score = total_score + 9;
                    }
                    // ---- VS Scissors
                    // Lose against Scissors = Paper (2 Points)
                    if my_move == 'X' && their_move == 'C' {
                        total_score = total_score + 2;
                    }
                    // Draw against Scissors = Scissors (3 Point) + Draw (3 Points)
                    if my_move == 'Y' && their_move == 'C' {
                        total_score = total_score + 6;
                    }
                    // Win against Scissors = Rock (1 Points) + Win (6 Points)
                    if my_move == 'Z' && their_move == 'C' {
                        total_score = total_score + 7;
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
