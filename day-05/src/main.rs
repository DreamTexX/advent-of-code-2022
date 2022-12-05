#![feature(iter_array_chunks)]

use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("day-05/input.txt")?;
    let (stacks_template, instructions) = input.split_at(input.find("\n\n").unwrap());
    let stacks_template = stacks_template.lines();

    let mut stacks: HashMap<usize, Vec<String>> = stacks_template
        .rev()
        .map(|row: &str| {
            let mut chunked = row.chars().array_chunks::<4>();
            let mut chunks: Vec<String> = vec![];
            let mut chunk = chunked.next();
            while chunk != None {
                chunks.push(chunk.unwrap().iter().collect::<String>());
                chunk = chunked.next();
            }
            chunks.push(chunked.into_remainder().unwrap().collect::<String>());
            chunks
        })
        .fold(HashMap::new(), |mut map: HashMap<usize, Vec<String>>, line: Vec<String>| {
            if line[0].chars().filter(|c: &char| c.is_numeric()).count() > 0 {
                for element in line {
                    map.insert(element.trim().parse::<usize>().unwrap(), vec![]);
                }
            } else {
                for i in 1..=line.len() {
                    if line[i - 1].trim().len() > 0 {
                        map.get_mut(&i).unwrap().push(line[i - 1].trim().to_owned());
                    }
                }
            }
            map
        });

    let mut stacks1 = stacks.clone();
    instructions
        .clone()
        .lines()
        .for_each(|line: &str| {
            if line.trim().is_empty() {
                return
            }

            let instructions: Vec<usize> = line.split(" ").map(|c: &str| c.parse::<usize>().unwrap_or(0)).collect();
            let amount: usize = instructions[1];
            let from: usize = instructions[3];
            let to: usize = instructions[5];

            for _ in 0..amount {
                let stack = stacks1.get_mut(&from).unwrap();
                if stack.len() > 0 {
                    let value = stack.pop().unwrap().to_owned();
                    stacks1.get_mut(&to).unwrap().push(value);
                }
            }
        });

    let x: Vec<Vec<&str>> = vec![vec![]];

    instructions
        .lines()
        .for_each(|line: &str| {
            if line.trim().is_empty() {
                return
            }

            let instructions: Vec<usize> = line.split(" ").map(|c: &str| c.parse::<usize>().unwrap_or(0)).collect();
            let amount: usize = instructions[1];
            let from: usize = instructions[3];
            let to: usize = instructions[5];

            let stack = stacks.get_mut(&from).unwrap();
            let mut temp: Vec<String> = vec![];
            for _ in 0..amount {
                if stack.len() > 0 {
                    let value = stack.pop().unwrap().to_owned();
                    temp.push(value);
                }
            }
            for _ in 0..amount {
                stacks.get_mut(&to).unwrap().push(temp.pop().unwrap().to_owned());
            }
        });

    print!("Elements Task #1: ");
    for i in 1..=stacks1.len() {
        print!("{}", stacks1.get_mut(&i).unwrap().last().unwrap_or(&"".to_string()));
    }
    println!();

    print!("Elements Task #2: ");
    for i in 1..=stacks.len() {
        print!("{}", stacks.get_mut(&i).unwrap().last().unwrap_or(&"".to_string()));
    }
    println!();



    let slice = ["l", "o", "r", "e", "m"];
    let iter = slice.chunks_exact(2);

    Ok(())
}
