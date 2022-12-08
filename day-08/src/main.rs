use std::error::Error;
use std::fs;
use std::ops::Add;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("day-08/input.txt")?;
    let matrix: Vec<Vec<u32>> = input
        .lines()
        .map(|line: &str| line
            .chars()
            .map(|char| char
                .to_digit(10)
                .unwrap())
            .collect())
        .collect();

    let mut visible = 0;
    let bound_y = input.lines().count();
    let mut max_score = 0;
    for y in 0..bound_y {
        let bound_x = matrix[y].len();

        for x in 0..bound_x {
            if x == 0 || y == 0 || x == bound_x - 1 || y == bound_y - 1 {
                visible += 1;
                continue;
            }

            let height = &matrix[y][x];

            if matrix[y][0..x].iter().max().unwrap() < height
                || matrix[y][x + 1..bound_x].iter().max().unwrap() < height
                || matrix[0..y].iter().map(|items: &Vec<u32>| items.get(x).unwrap()).max().unwrap() < height
                || matrix[y + 1..bound_y].iter().map(|items: &Vec<u32>| items.get(x).unwrap()).max().unwrap() < height
            {
                visible += 1;
            }

            let score =
                matrix[y][0..x].iter().rev().position(|h| h >= height).unwrap_or(x - 1).add(1).max(1)
                    * matrix[y][x + 1..bound_x].iter().position(|h| h >= height).unwrap_or(bound_x - x - 2).add(1).max(1)
                    * matrix[0..y].iter().map(|items: &Vec<u32>| items.get(x).unwrap()).rev().position(|h| h >= height).unwrap_or(y - 1).add(1).max(1)
                    * matrix[y + 1..bound_y].iter().map(|items: &Vec<u32>| items.get(x).unwrap()).position(|h| h >= height).unwrap_or(bound_y - y - 2).add(1).max(1);
            println!("x: {} y: {} score: {}", x, y, score);
            max_score = score.max(max_score);
        }
    }

    println!("Task 1: {}", visible);
    println!("Task 2: {} | 461175", max_score);

    Ok(())
}
