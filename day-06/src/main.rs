#![feature(slice_group_by)]
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("day-06/input.txt")?;
    let sequence1 = input.chars().fold((0, vec![]), |(index, mut set): (usize, Vec<char>), next: char| {
        if set.len() >= 4 {
            let mut temp = set.clone();
            temp.sort();
            if temp.group_by(|a, b| a == b).count() >= 4 {
                return (index, set);
            }
            set.remove(0);
        }
        set.push(next);
        (index + 1, set)
    });
    println!("Task 1: '{}' at {}", sequence1.1.iter().collect::<String>(), sequence1.0);

    let sequence2 = input.chars().fold((0, vec![]), |(index, mut set): (usize, Vec<char>), next: char| {
        if set.len() >= 14 {
            let mut temp = set.clone();
            temp.sort();
            if temp.group_by(|a, b| a == b).count() >= 14 {
                return (index, set);
            }
            set.remove(0);
        }
        set.push(next);
        (index + 1, set)
    });
    println!("Task 2: '{}' at {}", sequence2.1.iter().collect::<String>(), sequence2.0);

    Ok(())
}