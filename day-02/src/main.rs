use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    // A, X - Rock      1
    // B, Y - Paper     2
    // C, Z - Scissors  3
    // Loose             0
    // Draw             3
    // Win              6

    // X - Loose
    // Y - Draw
    // Z - Win

    let input = fs::read_to_string("day-02/input.txt")?;
    let score: (i32, i32) = input.split("\n")
        .filter(|s| { !s.is_empty() })
        .map(|s| {
            match s {
                "A X" => (4, 3),
                "A Y" => (8, 4),
                "A Z" => (3, 8),
                "B X" => (1, 1),
                "B Y" => (5, 5),
                "B Z" => (9, 9),
                "C X" => (7, 2),
                "C Y" => (2, 6),
                "C Z" => (6, 7),
                _ => (0, 0)
            }
        })
        .fold((0, 0), |acc, curr| {
            (acc.0 + curr.0, acc.1 + curr.1)
        });

    println!("Score Wrong: {}", score.0);
    println!("Score Correct: {}", score.1);

    Ok(())
}
