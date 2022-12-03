#![feature(iter_array_chunks)]
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("day-03/input.txt")?;
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();
    let lines = content.split("\n")
        .filter(|s| !s.is_empty());
    let s: usize = lines.clone()
        .map(|s| {
            let center = s.len() / 2;
            (&s[..center], &s[center..])
        })
        .map(|strs| {
            let str1chars: HashSet<char> = strs.1.chars().collect();
            strs.0.chars().find(|c| str1chars.contains(c)).unwrap()
        })
        .map(|c| {
            chars.clone().position(|c1| c1 == c).unwrap() + 1
        })
        .sum();

    println!("Sum: {}", s);

    let s: usize = lines
        .array_chunks::<3>()
        .map(|[first, second, third]| {
            let s: HashSet<char> = second.chars().collect();
            let t: HashSet<char> = third.chars().collect();
            first.chars().find(|c| { s.contains(c) && t.contains(c) }).unwrap()
        })
        .map(|c| {
            chars.clone().position(|c1| c1 == c).unwrap() + 1
        })
        .sum();

    println!("Sum #2: {}", s);

    Ok(())
}
